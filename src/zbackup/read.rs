use protobuf;
use protobuf::stream::CodedInputStream;

use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufReader;
use std::io::Read;

use misc::*;
use compress::lzma;

use zbackup::crypto::*;
use zbackup::data::*;
use zbackup::proto;

pub fn read_storage_info <PathRef: AsRef <Path>> (
	path: PathRef,
) -> Result <proto::StorageInfo, String> {

	// open file

	let mut input =
		try! (
			io_result (
				File::open (
					path)));

	let mut coded_input_stream =
		CodedInputStream::new (
			& mut input);

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

	let storage_info: proto::StorageInfo =
		try! (
			read_message (
				& mut coded_input_stream,
				|| "storage info".to_string ()));

	// return

	Ok (storage_info)

}

pub fn read_backup_file <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <proto::BackupInfo, String> {

	// open file

	let mut input =
		try! (
			io_result (
				open_file_with_crypto (
					path,
					key)));

	let mut coded_input_stream =
		CodedInputStream::new (
			& mut input);

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

	let backup_info: proto::BackupInfo =
		try! (
			read_message (
				& mut coded_input_stream,
				|| "backup info".to_string ()));

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

	let mut input =
		try! (
			io_result_with_prefix (
				"Error opening file: ",
				open_file_with_crypto (
					path,
					key)));

	let mut coded_input_stream =
		CodedInputStream::new (
			& mut input);

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

	Ok (index_entries)

}

pub fn read_bundle <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <([u8; 24], Vec <u8>)>, String> {

	// open file

	let input =
		try! (
			io_result (
				open_file_with_crypto (
					path,
					key)));

	let mut buf_input =
		BufReader::new (
			input);

	let bundle_info = {

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut buf_input);

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

		let bundle_info: proto::BundleInfo =
			try! (
				read_message (
					& mut coded_input_stream,
					|| "bundle info".to_owned ()));

		bundle_info

	};

	// skip checksum TODO

	let mut checksum_buffer: [u8; 4] =
		[0u8; 4];

	try! (
		io_result (
			buf_input.read_exact (
				& mut checksum_buffer)));
	
	// decode compressed data

	let mut chunks: Vec <([u8; 24], Vec <u8>)> =
		vec! {};

	let mut lzma_reader =
		try! (
			lzma::LzmaReader::new (
				& mut buf_input));

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
				to_array (chunk_record.get_id ()),
				chunk_bytes,
			)
		);

	}

	Ok (chunks)

}

fn read_message <
	Type: protobuf::MessageStatic,
	NameFunction: Fn () -> String,
> (
	coded_input_stream: & mut CodedInputStream,
	name_function: NameFunction,
) -> Result <Type, String> {

	let message_length =
		try! (
			protobuf_result_with_prefix (
				format! (
					"Error reading {} length: ",
					name_function ()),
				coded_input_stream.read_raw_varint32 ()));

	let old_limit =
		try! (
			protobuf_result_with_prefix (
				format! (
					"Error preparing to read {}: ",
					name_function ()),
				coded_input_stream.push_limit (
					message_length)));

	let message =
		try! (
			protobuf_result_with_prefix (
				format! (
					"Error reading {}: ",
					name_function ()),
				protobuf::core::parse_from::<Type> (
					coded_input_stream)));

	coded_input_stream.pop_limit (
		old_limit);

	Ok (message)

}

fn open_file_with_crypto <PathRef: AsRef <Path>> (
	path: PathRef,
	key: Option <[u8; KEY_SIZE]>,
) -> io::Result <Box <Read>> {

	Ok (match key {

		Some (key) =>
			Box::new (
				try! (
					CryptoReader::open (
						path,
						key))),

		None =>
			Box::new (
				try! (
					File::open (
						path))),

	})

}

// ex: noet ts=4 filetype=rust
