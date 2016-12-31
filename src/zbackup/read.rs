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

use protobuf;
use protobuf::stream::CodedInputStream;

use misc::*;
use compress::lzma;
use zbackup::crypto::*;
use zbackup::data::*;
use zbackup::proto;

pub fn read_storage_info <PathRef: AsRef <Path>> (
	path: PathRef,
) -> Result <proto::StorageInfo, String> {

	let storage_info: proto::StorageInfo;

	// open file

	let source = try! (

		io_result (
			File::open (
				path))

	);

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
			try! (
				read_message (
					& mut coded_input_stream,
					|| "file header".to_string ()));

		if file_header.get_version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.get_version ());

		}

		// read storage info

		storage_info =
			try! (
				read_message (
					& mut coded_input_stream,
					|| "storage info".to_string ()));

	}

	// verify checksum

	try! (
		verify_adler_and_eof (
			source));

	// return

	Ok (storage_info)

}

pub fn read_backup_file <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <proto::BackupInfo, String> {

	let backup_info: proto::BackupInfo;

	// open file

	let mut source =
		try! (
			io_result (
				open_file_with_crypto_and_adler (
					path,
					key)));

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read file header

		let file_header: proto::FileHeader =
			try! (
				read_message (
					& mut coded_input_stream,
					|| "file header".to_string ()));

		if file_header.get_version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.get_version ());

		}

		// read backup info

		backup_info =
			try! (
				read_message (
					& mut coded_input_stream,
					|| "backup info".to_string ()));

	}

	// verify checksum

	try! (
		verify_adler_and_eof (
			source));

	// return

	Ok (backup_info)

}

pub fn read_index <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <IndexEntry>, String> {

	let mut index_entries: Vec <IndexEntry> =
		vec! ();

	// open file

	let mut source =
		try! (
			io_result_with_prefix (
				"Error opening file: ",
				open_file_with_crypto_and_adler (
					path,
					key)));

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read header

		let file_header: proto::FileHeader =
			try! (
				read_message (
					& mut coded_input_stream,
					|| "file header".to_string ()));

		if file_header.get_version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.get_version ());

		}

		let mut bundle_info_index = 0;

		loop {

			let index_bundle_header: proto::IndexBundleHeader =
				try! (
					read_message (
						& mut coded_input_stream,
						|| format! (
							"index bundle header {}",
							bundle_info_index)));

			if ! index_bundle_header.has_id () {
				break;
			}

			let bundle_info: proto::BundleInfo =
				try! (
					read_message (
						& mut coded_input_stream,
						|| format! (
							"bundle info {}",
							bundle_info_index)));

			index_entries.push ( (
				index_bundle_header,
				bundle_info) );

			bundle_info_index += 1;

		}

	}

	// verify checksum

	try! (
		verify_adler_and_eof (
			source));

	Ok (index_entries)

}

pub fn read_bundle_info <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <proto::BundleInfo, String> {

	let bundle_info: proto::BundleInfo;

	// open file

	let mut source = try! (

		io_result (
			open_file_with_crypto_and_adler (
				path,
				key))

	);

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read bundle file header

		let bundle_file_header: proto::BundleFileHeader =
			read_message (
				& mut coded_input_stream,
				|| "bundle file header".to_string (),
			) ?;

		if bundle_file_header.get_version () != 1 {

			return Err (
				format! (
					"Unsupported bundle file version {}",
					bundle_file_header.get_version ()));

		}

		if bundle_file_header.get_compression_method () != "lzma" {

			return Err (
				format! (
					"Unsupported bundle file compression method {}",
					bundle_file_header.get_compression_method ()));

		}

		// read bundle info

		bundle_info =
			read_message (
				& mut coded_input_stream,
				|| "bundle info".to_owned (),
			) ?;

	}

	// verify checksum

	try! (
		verify_adler (
			& mut source));

	Ok (bundle_info)

}

pub fn read_bundle <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <(ChunkId, Vec <u8>)>, String> {

	let bundle_info: proto::BundleInfo;
	let mut chunks: Vec <(ChunkId, Vec <u8>)>;

	// open file

	let mut source = try! (

		io_result (
			open_file_with_crypto_and_adler (
				path,
				key))

	);

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read bundle file header

		let bundle_file_header: proto::BundleFileHeader =
			read_message (
				& mut coded_input_stream,
				|| "bundle file header".to_string (),
			) ?;

		if bundle_file_header.get_version () != 1 {

			return Err (
				format! (
					"Unsupported bundle file version {}",
					bundle_file_header.get_version ()));

		}

		if bundle_file_header.get_compression_method () != "lzma" {

			return Err (
				format! (
					"Unsupported bundle file compression method {}",
					bundle_file_header.get_compression_method ()));

		}

		// read bundle info

		bundle_info =
			read_message (
				& mut coded_input_stream,
				|| "bundle info".to_owned (),
			) ?;

	}

	// verify checksum

	try! (
		verify_adler (
			& mut source));

	{

		// decode compressed data

		chunks = Vec::new ();

		let mut lzma_reader =
			try! (
				lzma::LzmaReader::new (
					& mut source));

		// split into chunks

		for chunk_record in bundle_info.get_chunk_record () {

			let mut chunk_bytes: Vec <u8> =
				vec! [0u8; chunk_record.get_size () as usize];

			try! (
				io_result (
					lzma_reader.read_exact (
						& mut chunk_bytes)));

			chunks.push (
				(
					to_array_24 (chunk_record.get_id ()),
					chunk_bytes,
				)
			);

		}

		// finish reading lzma stream, otherwise checksum may not match

		{

			let mut extra_data: Vec <u8> =
				Vec::new ();

			io_result (
				lzma_reader.read_to_end (
					& mut extra_data,
				)
			) ?;

			if ! extra_data.is_empty () {

				panic! (
					"Got {} extra bytes",
					extra_data.len ());

			}

		}

	}

	// verify checksum

	try! (
		verify_adler_and_eof (
			source));

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
		try! (
			protobuf_result_with_prefix (
				|| format! (
					"Error reading {} length: ",
					name_function ()),
				coded_input_stream.read_raw_varint32 ()));

	let old_limit =
		try! (
			protobuf_result_with_prefix (
				|| format! (
					"Error preparing to read {}: ",
					name_function ()),
				coded_input_stream.push_limit (
					message_length)));

	let message =
		try! (
			protobuf_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					name_function ()),
				protobuf::core::parse_from::<Type> (
					coded_input_stream)));

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
				try! (
					CryptoReader::open (
						path,
						key));

			let mut initialisation_vector =
				[0u8; IV_SIZE];

			try! (
				crypto_reader.read_exact (
					& mut initialisation_vector));

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

			try! (
				File::open (
					path))

		))),

	})

}

fn verify_adler (
	adler_read: & mut AdlerRead,
) -> Result <(), String> {

	// verify hash

	let calculated_hash =
		adler_read.hash ();

	let expected_hash = try! (

		io_result_with_prefix (
			"Error reading adler32 checksum: ",
			adler_read.read_u32::<LittleEndian> ())

	);

	if calculated_hash != expected_hash {

		return Err (
			format! (
				"Adler32 hash calculated {} but expected {}, at position \
				0x{:x}",
				calculated_hash,
				expected_hash,
				adler_read.byte_count - 4));

	}

	// return ok

	Ok (())

}

fn verify_adler_and_eof (
	mut adler_read: AdlerRead,
) -> Result <(), String> {

	try! (
		verify_adler (
			& mut adler_read));

	// verify end of file

	let mut byte_buffer: [u8; 1] = [0u8; 1];

	let bytes_read = try! (

		io_result_with_prefix (
			"Error checking for end of file: ",
			adler_read.read (
				& mut byte_buffer))

	);

	if bytes_read != 0 {

		return Err (
			format! (
				"Extra data at end of file"));

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
