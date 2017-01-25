use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter;

use adler32::RollingAdler32;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use protobuf;
use protobuf::stream::CodedInputStream;

use rustc_serialize::hex::ToHex;

use misc::*;
use compress::lzma;
use zbackup::crypto::*;
use zbackup::data::*;
use zbackup::proto;

#[ inline ]
pub fn read_storage_info <PathRef: AsRef <Path>> (
	info_path: PathRef,
) -> Result <proto::StorageInfo, String> {

	read_storage_info_impl (
		info_path.as_ref (),
	)

}

pub fn read_storage_info_impl (
	info_path: & Path,
) -> Result <proto::StorageInfo, String> {

	let storage_info: proto::StorageInfo;

	// open file

	let source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				info_path.to_string_lossy ()),
			File::open (
				info_path),
		) ?;

	let mut source =
		AdlerRead::new (
			Box::new (
				BufReader::new (
					source)));

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read file header

		let file_header: proto::FileHeader =
			read_message (
				& mut coded_input_stream,
				|| "file header".to_string (),
			) ?;

		if file_header.get_version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.get_version ());

		}

		// read storage info

		storage_info =
			read_message (
				& mut coded_input_stream,
				|| "storage info".to_string (),
			) ?;

	}

	// verify checksum

	verify_adler_and_eof (
		|| format! (
			"Error reading {}: ",
			info_path.to_string_lossy ()),
		source,
	) ?;

	// return

	Ok (storage_info)

}

pub fn read_backup_file <PathRef: AsRef <Path>> (
	backup_path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <proto::BackupInfo, String> {

	let backup_path = backup_path.as_ref ();

	let backup_info: proto::BackupInfo;

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error reading {}: ",
				backup_path.to_string_lossy ()),
			open_file_with_crypto_and_adler (
				backup_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read file header

		let file_header: proto::FileHeader =
			read_message (
				& mut coded_input_stream,
				|| "file header".to_string (),
			) ?;

		if file_header.get_version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.get_version ());

		}

		// read backup info

		backup_info =
			read_message (
				& mut coded_input_stream,
				|| "backup info".to_string (),
			) ?;

	}

	// verify checksum

	verify_adler_and_eof (
		|| format! (
			"Error reading {}: ",
			backup_path.to_string_lossy ()),
		source,
	) ?;

	// return

	Ok (backup_info)

}

pub fn read_index <PathRef: AsRef <Path>> (
	index_path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <IndexEntry>, String> {

	let index_path = index_path.as_ref ();

	let mut index_entries: Vec <IndexEntry> =
		vec! ();

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				index_path.to_string_lossy ()),
			open_file_with_crypto_and_adler (
				index_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read header

		let file_header: proto::FileHeader =
			string_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					index_path.to_string_lossy ()),
				read_message (
					& mut coded_input_stream,
					|| "file header".to_string ()),
			) ?;

		if file_header.get_version () != 1 {

			return Err (
				format! (
					"Error reading {}: Unsupported backup version {}",
					index_path.to_string_lossy (),
					file_header.get_version ()));

		}

		let mut bundle_info_index = 0;

		loop {

			let index_bundle_header: proto::IndexBundleHeader =
				string_result_with_prefix (
					|| format! (
						"Error reading {}: ",
						index_path.to_string_lossy ()),
					read_message (
						& mut coded_input_stream,
						|| format! (
							"index bundle header {}",
							bundle_info_index)),
				) ?;

			if ! index_bundle_header.has_id () {
				break;
			}

			let bundle_info: proto::BundleInfo =
				string_result_with_prefix (
					|| format! (
						"Error reading {}: ",
						index_path.to_string_lossy ()),
					read_message (
						& mut coded_input_stream,
						|| format! (
							"bundle info {}",
							bundle_info_index)),
				) ?;

			index_entries.push ( (
				index_bundle_header,
				bundle_info) );

			bundle_info_index += 1;

		}

	}

	// verify checksum

	verify_adler_and_eof (
		|| format! (
			"Error reading {}: ",
			index_path.to_string_lossy ()),
		source,
	) ?;

	Ok (index_entries)

}

pub fn read_bundle_info <PathRef: AsRef <Path>> (
	bundle_path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <proto::BundleInfo, String> {

	let bundle_path = bundle_path.as_ref ();

	let bundle_info: proto::BundleInfo;

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error reading {}: ",
				bundle_path.to_string_lossy ()),
			open_file_with_crypto_and_adler (
				bundle_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read bundle file header

		let bundle_file_header: proto::BundleFileHeader =
			string_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					bundle_path.to_string_lossy ()),
				read_message (
					& mut coded_input_stream,
					|| "bundle file header".to_string (),
				),
			) ?;

		if bundle_file_header.get_version () != 1 {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file version {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.get_version ()));

		}

		if bundle_file_header.get_compression_method () != "lzma" {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file compression \
						method {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.get_compression_method ()));

		}

		// read bundle info

		bundle_info =
			string_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					bundle_path.to_string_lossy ()),
				read_message (
					& mut coded_input_stream,
					|| "bundle info".to_owned (),
				),
			) ?;

	}

	// verify checksum

	verify_adler (
		|| format! (
			"Error reading {}: ",
			bundle_path.to_string_lossy ()),
		& mut source,
	) ?;

	Ok (bundle_info)

}

pub fn read_bundle <PathRef: AsRef <Path>> (
	bundle_path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <(ChunkId, Vec <u8>)>, String> {

	let bundle_path = bundle_path.as_ref ();

	let bundle_info: proto::BundleInfo;
	let mut chunks: Vec <(ChunkId, Vec <u8>)>;

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				bundle_path.to_string_lossy ()),
			open_file_with_crypto_and_adler (
				bundle_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read bundle file header

		let bundle_file_header: proto::BundleFileHeader =
			string_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					bundle_path.to_string_lossy ()),
				read_message (
					& mut coded_input_stream,
					|| "bundle file header".to_string (),
				),
			) ?;

		if bundle_file_header.get_version () != 1 {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file version {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.get_version ()));

		}

		if bundle_file_header.get_compression_method () != "lzma" {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file compression \
					method {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.get_compression_method ()));

		}

		// read bundle info

		bundle_info =
			string_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					bundle_path.to_string_lossy ()),
				read_message (
					& mut coded_input_stream,
					|| "bundle info".to_owned ()),
			) ?;

	}

	// verify checksum

	verify_adler (
		|| format! (
			"Error reading {}: ",
			bundle_path.to_string_lossy ()),
		& mut source,
	) ?;

	{

		// decode compressed data

		chunks = Vec::new ();

		let mut lzma_reader =
			lzma::LzmaReader::new (
				& mut source,
			) ?;

		// split into chunks

		for chunk_record in bundle_info.get_chunk_record () {

			let chunk_id =
				to_array_24 (
					chunk_record.get_id ());

			// verify sha1 sum

			let mut chunk_bytes: Vec <u8> =
				vec! [0u8; chunk_record.get_size () as usize];

			io_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					bundle_path.to_string_lossy ()),
				lzma_reader.read_exact (
					& mut chunk_bytes),
			) ?;

			let mut sha1_digest =
				Sha1::new ();

			sha1_digest.input (
				& chunk_bytes);

			let mut sha1_sum: [u8; 20] =
				[0u8; 20];

			sha1_digest.result (
				& mut sha1_sum);

			if chunk_id [0 .. 16] != sha1_sum [0 .. 16] {

				return Err (
					format! (
						"Error reading {}: Invalid sha1 sum for chunk {}: {}",
						bundle_path.to_string_lossy (),
						chunk_id.to_hex (),
						sha1_sum.to_hex ()));

			}

			// store it

			chunks.push (
				(
					chunk_id,
					chunk_bytes,
				)
			);

		}

		// finish reading lzma stream, otherwise checksum may not match

		{

			let mut extra_data: Vec <u8> =
				Vec::new ();

			io_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					bundle_path.to_string_lossy ()),
				lzma_reader.read_to_end (
					& mut extra_data,
				),
			) ?;

			if ! extra_data.is_empty () {

				return Err (
					format! (
						"Error reading {}: Got {} extra bytes",
						bundle_path.to_string_lossy (),
						extra_data.len ()));

			}

		}

	}

	// verify checksum

	verify_adler_and_eof (
		|| format! (
			"Error reading {}: ",
			bundle_path.to_string_lossy ()),
		source,
	) ?;

	Ok (chunks)

}

pub fn read_message <
	Type: protobuf::MessageStatic,
	NameFunction: Fn () -> String,
> (
	coded_input_stream: & mut CodedInputStream,
	name_function: NameFunction,
) -> Result <Type, String> {

	let message_length =
		protobuf_result_with_prefix (
			|| format! (
				"Error reading {} length: ",
				name_function ()),
			coded_input_stream.read_raw_varint32 (),
		) ?;

	let old_limit =
		protobuf_result_with_prefix (
			|| format! (
				"Error preparing to read {}: ",
				name_function ()),
			coded_input_stream.push_limit (
				message_length),
		) ?;

	let message =
		protobuf_result_with_prefix (
			|| format! (
				"Error reading {}: ",
				name_function ()),
			protobuf::core::parse_from::<Type> (
				coded_input_stream),
		) ?;

	coded_input_stream.pop_limit (
		old_limit);

	Ok (message)

}

fn open_file_with_crypto_and_adler <
	PathRef: AsRef <Path>
> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> io::Result <AdlerRead> {

	Ok (match key {

		Some (key) => {

			let mut crypto_reader =
				CryptoReader::open (
					path,
					key,
				) ?;

			let mut initialisation_vector =
				[0u8; IV_SIZE];

			crypto_reader.read_exact (
				& mut initialisation_vector,
			) ?;

			let mut adler_read =
				AdlerRead::new (
					Box::new (
						BufReader::new (
							crypto_reader)));

			adler_read.update (
				& initialisation_vector);

			adler_read

		},

		None =>
			AdlerRead::new (
				Box::new (
					BufReader::new (

			File::open (
				path,
			) ?

		))),

	})

}

fn verify_adler <
	PrefixFunction: Fn () -> String,
> (
	prefix_function: PrefixFunction,
	adler_read: & mut AdlerRead,
) -> Result <(), String> {

	// verify hash

	let calculated_hash =
		adler_read.hash ();

	let expected_hash =
		io_result_with_prefix (
			|| format! (
				"{}Error reading adler32 checksum: ",
				prefix_function ()),
			adler_read.read_u32::<LittleEndian> (),
		) ?;

	if calculated_hash != expected_hash {

		return Err (
			format! (
				"{}Adler32 hash calculated {} but expected {}, at position \
				0x{:x}",
				prefix_function (),
				calculated_hash,
				expected_hash,
				adler_read.byte_count - 4));

	}

	// return ok

	Ok (())

}

fn verify_adler_and_eof <
	PrefixFunction: Fn () -> String,
> (
	prefix_function: PrefixFunction,
	mut adler_read: AdlerRead,
) -> Result <(), String> {

	verify_adler (
		& prefix_function,
		& mut adler_read,
	) ?;

	// verify end of file

	let mut byte_buffer: [u8; 1] = [0u8; 1];

	let bytes_read =
		io_result_with_prefix (
			|| format! (
				"{}Error checking for end of file: ",
				& prefix_function ()),
			adler_read.read (
				& mut byte_buffer),
		) ?;

	if bytes_read != 0 {

		return Err (
			format! (
				"{}Extra data at end of file",
				prefix_function ()));

	}

	// return ok

	Ok (())

}

struct AdlerRead {
	source: Box <BufRead>,
	adler: RollingAdler32,
	byte_count: usize,
}

impl AdlerRead {

	fn new (
		source: Box <BufRead>,
	) -> AdlerRead {

		AdlerRead {
			source: source,
			adler: RollingAdler32::new (),
			byte_count: 0,
		}

	}

	fn hash (
		& self,
	) -> u32 {

		self.adler.hash ()

	}

	fn update (
		& mut self,
		data: & [u8],
	) {

		self.adler.update_buffer (
			data);

		self.byte_count +=
			data.len ();

	}

}

impl Read for AdlerRead {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> Result <usize, io::Error> {

		match self.source.read (
			buffer) {

			Ok (read_size) => {

				self.adler.update_buffer (
					& buffer [0 .. read_size]);

				self.byte_count +=
					read_size;

				Ok (read_size)

			},

			Err (error) =>
				Err (error),

		}

	}

}

impl BufRead for AdlerRead {

	fn fill_buf (
		& mut self,
	) -> Result <& [u8], io::Error> {

		self.source.fill_buf ()

	}

	fn consume (
		& mut self,
		amount: usize,
	) {

		let mut buffer: Vec <u8> =
			iter::repeat (0u8)
				.take (amount)
				.collect ();

		self.source.read_exact (
			& mut buffer,
		).unwrap ();

		self.adler.update_buffer (
			& buffer);

		self.byte_count +=
			amount;

	}

}

// ex: noet ts=4 filetype=rust
