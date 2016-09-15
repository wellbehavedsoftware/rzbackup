use crypto;
use crypto::aessafe::AesSafe128Decryptor;
use crypto::mac::Mac;
use crypto::symmetriccipher::BlockDecryptor;
use crypto::symmetriccipher::Decryptor;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use zbackup::data::*;
use zbackup::proto;

use misc::*;

pub struct CryptoReader {

	input: File,
	eof: bool,

	decryptor: AesSafe128Decryptor,
	initialisation_vector: [u8; IV_SIZE],
	encryption_key: [u8; KEY_SIZE],

	input_buffer: [u8; PAGE_SIZE],
	output_buffer: [u8; PAGE_SIZE],

	start_index: usize,
	end_index: usize,

}

impl CryptoReader {

	pub fn open <PathRef: AsRef <Path>> (
		path: PathRef,
		encryption_key: [u8; KEY_SIZE],
	) -> Result <CryptoReader, String> {

		// open file

		let mut file =
			try! (
				io_result (
					File::open (
						path)));

		let mut buffer =
			[0u8; PAGE_SIZE];

		// read first iv

		let mut initialisation_vector: [u8; IV_SIZE] =
			[0u8; IV_SIZE];

		try! (
			io_result (
				file.read_exact (
					& mut initialisation_vector)));

		// setup decryptor

		let decryptor =
			AesSafe128Decryptor::new (
				& encryption_key);

		// return

		Ok (CryptoReader {

			input: file,
			eof: false,

			decryptor: decryptor,
			initialisation_vector: initialisation_vector,
			encryption_key: encryption_key,

			input_buffer: [0u8; PAGE_SIZE],
			output_buffer: [0u8; PAGE_SIZE],

			start_index: 0,
			end_index: 0,

		})

	}

	fn read_and_decrypt (
		& mut self,
	) -> io::Result <()> {

		assert! (
			! self.eof);

		assert! (
			self.start_index == self.end_index);

		if self.start_index == 0 {

			// read in some data

			self.end_index =
				try! (
					self.input.read (
						& mut self.input_buffer));

		} else {

			// shift the last block back

			assert! (
				self.start_index == PAGE_SIZE - BLOCK_SIZE);

			assert! (
				self.end_index == PAGE_SIZE);

			for index in 0 .. BLOCK_SIZE {

				self.input_buffer [index] =
					self.input_buffer [
						PAGE_SIZE
						- BLOCK_SIZE
						+ index];

			}

			// read in some more data

			self.start_index = 0;

			self.end_index =
				BLOCK_SIZE +
				try! (
					self.input.read (
						& mut self.input_buffer [
							BLOCK_SIZE ..
							PAGE_SIZE]));

		}

		if self.end_index < PAGE_SIZE {
			self.eof = true;
		}

		// decrypt the data in the buffer, except the last block if full

		let start_position =
			if self.end_index == PAGE_SIZE {
				self.end_index - PAGE_SIZE
			} else {
				self.end_index
			};

		let mut position =
			start_position;

		while position > 0 {

			// decrypt block

			let source_block =
				& self.input_buffer [
					position - BLOCK_SIZE ..
					position];

			let target_block =
				& mut self.output_buffer [
					position - BLOCK_SIZE ..
					position];

			self.decryptor.decrypt_block (
				source_block,
				target_block);

			position -= BLOCK_SIZE;

			// combine iv

			let iv_to_use =
				if position == 0 {
					& self.initialisation_vector
				} else {
					& self.input_buffer [
						position - BLOCK_SIZE ..
						position]
				};

			for index in 0 .. BLOCK_SIZE {

				target_block [index] ^=
					iv_to_use [index];

			}

		}

		self.initialisation_vector.copy_from_slice (
			& self.input_buffer [
				start_position - BLOCK_SIZE ..
				start_position]);

		// and return

		Ok (())

	}

}

impl Read for CryptoReader {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> io::Result <usize> {

		let mut total_bytes_read: usize =
			0;

		// loop to fill buffer while decrypting

		while ! self.eof {

			// read more data if appropriate

			if self.start_index == 0
			|| self.start_index == PAGE_SIZE - BLOCK_SIZE {

				try! (
					self.read_and_decrypt ());

			}

			// check available data

			let buffer_remaining =
				& mut buffer [total_bytes_read .. ];

			let available_bytes =
				if self.end_index == PAGE_SIZE {
					self.end_index - self.start_index - BLOCK_SIZE
				} else {
					self.end_index - self.start_index
				};

			if available_bytes == 0 {
				return Ok (total_bytes_read);
			}

			// if we have enough data, copy that and we're done

			if buffer_remaining.len () <= available_bytes {

				let buffer_remaining_len =
					buffer_remaining.len ();

				buffer_remaining.copy_from_slice (
					& self.output_buffer [
						self.start_index ..
						self.start_index + buffer_remaining_len]);

				self.start_index +=
					buffer_remaining.len ();

				total_bytes_read +=
					buffer_remaining.len ();

				return Ok (total_bytes_read);

			}

			// we have some data but not enough, copy, decrypt and loop

			buffer_remaining [0 .. available_bytes].copy_from_slice (
				& self.output_buffer [
					self.start_index ..
					self.end_index]);

			self.start_index =
				self.end_index;

			total_bytes_read +=
				buffer_remaining.len ();

		}

		Ok (total_bytes_read)

	}

}

pub fn decrypt_key (
	password_file_path: & str,
	encryption_key: & proto::EncryptionKeyInfo,
) -> Result <Option <[u8; KEY_SIZE]>, String> {

	// read password from file

	let mut password_file =
		try! (
			io_result (
				File::open (
					password_file_path)));

	let mut password_string =
		String::new ();

	try! (
		io_result (
			password_file.read_to_string (
				& mut password_string)));

	// remove trailing newline

	if password_string.ends_with ("\n") {

		let password_length =
			password_string.len ();

		password_string.truncate (
			password_length - 1);

	}

	let password_bytes =
		password_string.as_bytes ();

	// derive password key from password

	let mut password_hmac =
		crypto::hmac::Hmac::new (
			crypto::sha1::Sha1::new (),
			password_bytes);

	let mut password_result =
		[0u8; KEY_SIZE];

	crypto::pbkdf2::pbkdf2 (
		& mut password_hmac,
		encryption_key.get_salt (),
		encryption_key.get_rounds (),
		& mut password_result);

	// decrypt actual key using password key

	let key_decryptor =
		crypto::aessafe::AesSafe128Decryptor::new (
			& password_result);

	let mut key_result =
		[0u8; KEY_SIZE];

	key_decryptor.decrypt_block (
		& encryption_key.get_encrypted_key (),
		& mut key_result);

	// derive check result to verify password

	let mut check_hmac =
		crypto::hmac::Hmac::new (
			crypto::sha1::Sha1::new (),
			& key_result);

	check_hmac.input (
		encryption_key.get_key_check_input ());

	let mut check_result =
		[0u8; HMAC_SIZE];

	check_hmac.raw_result (
		& mut check_result);

	// return

	if check_result == encryption_key.get_key_check_hmac () {

		Ok (Some (
			key_result
		))

	} else {

		Ok (None)

	}

}

// ex: noet ts=4 filetype=rs
