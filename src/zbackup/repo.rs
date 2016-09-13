use crypto;
use crypto::mac::Mac;
use crypto::symmetriccipher::BlockDecryptor;

use protobuf;
use protobuf::stream::CodedInputStream;

use rustc_serialize::hex::ToHex;

use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;

use misc::*;

use zbackup;
use zbackup::proto;
use zbackup::randaccess::*;
use zbackup::read::*;

const KEY_SIZE: usize =
	16;

const CACHE_MAX_SIZE: usize =
	0x10000;

pub struct MasterIndexEntry {
	pub bundle_id: [u8; 24],
	pub size: u64,
}

pub struct Repository {
	path: String,
	storage_info: proto::StorageInfo,
	master_index: HashMap <[u8; 24], MasterIndexEntry>,
	chunk_cache: HashMap <[u8; 24], Arc <Vec <u8>>>,
}

impl Repository {

	pub fn open (
		repository_path: & str,
		password_file_path: Option <& str>,
	) -> Result <Repository, String> {

		// load info file

		stderrln! (
			"Loading repository {}",
			repository_path);

		let storage_info =
			try! (
				read_storage_info (
					& format! (
						"{}/info",
						repository_path)));

		if storage_info.has_encryption_key () {

			stderrln! (
				"ENCRYPTION KEY SALT: {:?}",
				storage_info.get_encryption_key ().get_salt ());

			stderrln! (
				"ENCRYPTION KEY ROUNDS: {:?}",
				storage_info.get_encryption_key ().get_rounds ());

			stderrln! (
				"ENCRYPTED KEY: {:?}",
				storage_info.get_encryption_key ().get_encrypted_key ());

			stderrln! (
				"KEY CHECK INPUT: {:?}",
				storage_info.get_encryption_key ().get_key_check_input ());

			stderrln! (
				"KEY CHECK HMAC: {:?}",
				storage_info.get_encryption_key ().get_key_check_hmac ());

			// derive password key from password

			let mut password_hmac =
				crypto::hmac::Hmac::new (
					crypto::sha1::Sha1::new (),
					b"password");

			let mut password_result =
				[0u8; KEY_SIZE];

			crypto::pbkdf2::pbkdf2 (
				& mut password_hmac,
				storage_info.get_encryption_key ().get_salt (),
				storage_info.get_encryption_key ().get_rounds (),
				& mut password_result);

			stderrln! (
				"PASSWORD RESULT: {:?}",
				password_result);

			// decrypt actual key using password key

			let key_decryptor =
				crypto::aessafe::AesSafe128Decryptor::new (
					& password_result);

			let mut key_result =
				[0u8; KEY_SIZE];

			key_decryptor.decrypt_block (
				& storage_info.get_encryption_key ().get_encrypted_key (),
				& mut key_result);

			stderrln! (
				"KEY RESULT: {:?}",
				key_result);

			// derive check result to verify password

			let mut check_hmac =
				crypto::hmac::Hmac::new (
					crypto::sha1::Sha1::new (),
					& key_result);

			check_hmac.input (
				storage_info.get_encryption_key ().get_key_check_input ());

			let mut check_result =
				[0u8; 20];

			check_hmac.raw_result (
				& mut check_result);

			stderrln! (
				"SIGNED RESULT: {:?}",
				check_result);

		}

		// load indexes

		stderr! (
			"Loading indexes");

		let mut master_index: HashMap <[u8; 24], MasterIndexEntry> =
			HashMap::with_capacity (0x10000);

		let mut count: u64 = 0;

		for dir_entry_or_error in try! (
			io_result (
				fs::read_dir (
					format! (
						"{}/index",
						repository_path)))
		) {

			let dir_entry =
				try! (
					io_result (
						dir_entry_or_error));

			let file_name =
				dir_entry.file_name ();

			let index_name =
				file_name.to_str ().unwrap ();

			let index =
				try! (
					read_index (
						& format! (
							"{}/index/{}",
							repository_path,
							index_name)));

			for (index_bundle_header, bundle_info) in index {

				for chunk_record in bundle_info.get_chunk_record () {

					master_index.insert (
						to_array (chunk_record.get_id ()),
						MasterIndexEntry {

							bundle_id:
								to_array (index_bundle_header.get_id ()),

							size:
								chunk_record.get_size () as u64

						},
					);

				}

			}

			if count & 0x3f == 0x3f {
				stderr! (
					".");
			}

			count += 1;

		}

		stderr! (
			"\n");

		// return

		Ok (Repository {
			path: repository_path.to_string (),
			storage_info: storage_info,
			master_index: master_index,
			chunk_cache: HashMap::new (),
		})

	}

	pub fn read_and_expand_backup (
		& mut self,
		backup_name: & str,
	) -> Result <Vec <u8>, String> {

		// load backup

		stderr! (
			"Loading backup {}",
			backup_name);

		let backup_info =
			try! (
				read_backup_file (
					format! (
						"{}/backups/{}",
						& self.path,
						backup_name)));

		// expand backup data

		let mut input =
			Cursor::new (
				backup_info.get_backup_data ().to_owned ());

		for _iteration in 0 .. backup_info.get_iterations () {

			let mut temp_output: Cursor <Vec <u8>> =
				Cursor::new (
					Vec::new ());

			try! (
				self.follow_instructions (
					& mut input,
					& mut temp_output,
					& |count| {
						if count & 0xf == 0xf {
							stderr! (".");
						}
					}));

			input =
				Cursor::new (
					temp_output.into_inner ());

		}

		stderr! (
			"\n");

		Ok (input.into_inner ())

	}

	pub fn restore (
		& mut self,
		backup_name: & str,
		output: & mut Write,
	) -> Result <(), String> {

		let mut input =
			Cursor::new (
				try! (
					self.read_and_expand_backup (
						backup_name)));

		// restore backup

		stderr! (
			"Restoring backup");

		try! (
			self.follow_instructions (
				& mut input,
				output,
				& |count| {
					if count & 0x1ff == 0x1ff {
						stderr! (".");
					}
				}));

		stderr! (
			"\n");

		stderrln! (
			"Restore complete");

		Ok (())

	}

	pub fn restore_test (
		& mut self,
		backup_name: & str,
		output: & mut Write,
	) -> Result <(), String> {

		stderr! (
			"Loading backup {}",
			backup_name);

		let mut input =
			try! (
				RandomAccess::new (
					self,
					backup_name));

		let mut buffer: Vec <u8> =
			Vec::with_capacity (
				1024 * 1024);

		unsafe {

			buffer.set_len (
				1024 * 1024);

		}

		// restore backup

		stderr! (
			"Restoring backup");

		loop {

			let bytes_read =
				try! (
					io_result (
						input.read (
							& mut buffer)));

			if bytes_read == 0 {
				break;
			}

			try! (
				io_result (
					output.write (
						& buffer [
							0 .. bytes_read ])));

		}

		stderrln! (
			"Restore complete");

		Ok (())

	}

	pub fn follow_instruction (
		& mut self,
		backup_instruction: & proto::BackupInstruction,
		output: & mut Write,
	) -> Result <(), String> {

		if backup_instruction.has_chunk_to_emit () {

			let chunk_id =
				to_array (
					backup_instruction.get_chunk_to_emit ());

			let chunk_data = try! (
				self.get_chunk (
					chunk_id));

			try! (
				io_result (
					output.write (
						& chunk_data)));

		}

		if backup_instruction.has_bytes_to_emit () {

			try! (
				io_result (
					output.write (
						backup_instruction.get_bytes_to_emit ())));

		}

		Ok (())

	}

	pub fn follow_instructions (
		& mut self,
		input: & mut Read,
		output: & mut Write,
		progress: & Fn (u64),
	) -> Result <(), String> {

		let mut coded_input_stream =
			CodedInputStream::new (
				input);

		let mut count: u64 = 0;

		while ! try! (
			protobuf_result (
				coded_input_stream.eof ())
		) {

			let instruction_length =
				try! (
					protobuf_result (
						coded_input_stream.read_raw_varint32 ()));

			let instruction_old_limit =
				try! (
					protobuf_result (
						coded_input_stream.push_limit (
							instruction_length)));

			let backup_instruction =
				try! (
					protobuf_result (
						protobuf::core::parse_from::<proto::BackupInstruction> (
							&mut coded_input_stream)));

			coded_input_stream.pop_limit (
				instruction_old_limit);

			try! (
				self.follow_instruction (
					& backup_instruction,
					output));

			progress (
				count);

			count += 1;

		}

		Ok (())

	}

	pub fn get_chunk (
		& mut self,
		chunk_id: [u8; 24],
	) -> Result <Arc <Vec <u8>>, String> {

		if ! self.chunk_cache.contains_key (& chunk_id) {

			if self.chunk_cache.len () >= CACHE_MAX_SIZE {

				self.chunk_cache.clear ();

			}

			let index_entry = match
				self.master_index.get (& chunk_id) {

				Some (value) =>
					value,

				None => {

					return Err (
						format! (
							"Missing chunk: {}",
							chunk_id.to_hex ()));

				},

			};

			for (found_chunk_id, found_chunk_data) in try! (

				read_bundle (
					& format! (
						"{}/bundles/{}/{}",
						self.path,
						& index_entry.bundle_id.to_hex () [0 .. 2],
						index_entry.bundle_id.to_hex ()))

			) {

				self.chunk_cache.insert (
					found_chunk_id,
					Arc::new (
						found_chunk_data));

			}

		}

		let chunk_data =
			self.chunk_cache.get (
				& chunk_id,
			).unwrap ();

		Ok (
			chunk_data.clone ())

	}

	pub fn get_index_entry (
		& mut self,
		chunk_id: & [u8; 24],
	) -> Result <& MasterIndexEntry, String> {

		return match self.master_index.get (
			chunk_id,
		) {

			Some (value) =>
				Ok (value),

			None =>
				Err (
					format! (
						"Missing chunk: {}",
						chunk_id.to_hex ())
				),

		};

	}

	pub fn open_backup (
		& mut self,
		backup_name: & str,
	) -> Result <RandomAccess, String> {

		RandomAccess::new (
			self,
			backup_name)

	}

}
