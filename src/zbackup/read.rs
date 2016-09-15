use protobuf;
use protobuf::stream::CodedInputStream;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use misc::*;
use compress::lzma;

use zbackup::data::*;
use zbackup::proto;

pub fn read_storage_info <P: AsRef <Path>> (
	path: P,
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

	// read header

	let header_length =
		try! (
			protobuf_result (
				coded_input_stream.read_raw_varint32 ()));

	let header_old_limit =
		try! (
			protobuf_result (
				coded_input_stream.push_limit (
					header_length)));

	let file_header =
		try! (
			protobuf_result (
				protobuf::core::parse_from::<proto::FileHeader> (
					& mut coded_input_stream)));

	if file_header.get_version () != 1 {

		panic! (
			"Unsupported backup version {}",
			file_header.get_version ());

	}

	coded_input_stream.pop_limit (
		header_old_limit);

	// read storage info

	let storage_info_length =
		try! (
			protobuf_result (
				coded_input_stream.read_raw_varint32 ()));

	let storage_info_old_limit =
		try! (
			protobuf_result (
				coded_input_stream.push_limit (
					storage_info_length)));

	let storage_info =
		try! (
			protobuf_result (
				protobuf::core::parse_from::<proto::StorageInfo> (
					& mut coded_input_stream)));

	coded_input_stream.pop_limit (
		storage_info_old_limit);

	Ok (storage_info)

}

pub fn read_backup_file <P: AsRef <Path>> (
	path: P,
) -> Result <proto::BackupInfo, String> {

	// open file

	let mut input =
		try! (
			io_result (
				File::open (
					path)));

	let mut coded_input_stream =
		CodedInputStream::new (
			& mut input);

	// read header

	let header_length =
		try! (
			protobuf_result (
				coded_input_stream.read_raw_varint32 ()));

	let header_old_limit =
		try! (
			protobuf_result (
				coded_input_stream.push_limit (
					header_length)));

	let file_header =
		try! (
			protobuf_result (
				protobuf::core::parse_from::<proto::FileHeader> (
					& mut coded_input_stream)));

	if file_header.get_version () != 1 {

		panic! (
			"Unsupported backup version {}",
			file_header.get_version ());

	}

	coded_input_stream.pop_limit (
		header_old_limit);

	// read backup info

	let backup_info_length =
		try! (
			protobuf_result (
				coded_input_stream.read_raw_varint32 ()));

	let backup_info_old_limit =
		try! (
			protobuf_result (
				coded_input_stream.push_limit (
					backup_info_length)));

	let backup_info =
		try! (
			protobuf_result (
				protobuf::core::parse_from::<proto::BackupInfo> (
					& mut coded_input_stream)));

	coded_input_stream.pop_limit (
		backup_info_old_limit);

	Ok (backup_info)

}

pub fn read_index <P: AsRef <Path>> (
	path: P,
) -> Result <Vec <IndexEntry>, String> {

	let mut index_entries: Vec <IndexEntry> =
		vec! ();

	// open file

	let mut input =
		try! (
			io_result (
				File::open (
					path)));

	let mut coded_input_stream =
		CodedInputStream::new (
			&mut input);

	// read header

	let header_length =
		try! (
			protobuf_result (
				coded_input_stream.read_raw_varint32 ()));

	let header_old_limit =
		try! (
			protobuf_result (
				coded_input_stream.push_limit (
					header_length)));

	let file_header =
		try! (
			protobuf_result (
				protobuf::core::parse_from::<proto::FileHeader> (
					& mut coded_input_stream)));

	if file_header.get_version () != 1 {

		panic! (
			"Unsupported backup version {}",
			file_header.get_version ());

	}

	coded_input_stream.pop_limit (
		header_old_limit);

	loop {

		// read index bundle header

		let index_bundle_header_length =
			try! (
				protobuf_result (
					coded_input_stream.read_raw_varint32 ()));

		let index_bundle_header_old_limit =
			try! (
				protobuf_result (
					coded_input_stream.push_limit (
						index_bundle_header_length)));

		let index_bundle_header =
			try! (
				protobuf_result (
					protobuf::core::parse_from::<proto::IndexBundleHeader> (
						& mut coded_input_stream)));

		coded_input_stream.pop_limit (
			index_bundle_header_old_limit);

		if ! index_bundle_header.has_id () {
			break;
		}

		// read bundle info

		let bundle_info_length =
			try! (
				protobuf_result (
					coded_input_stream.read_raw_varint32 ()));

		let bundle_info_old_limit =
			try! (
				protobuf_result (
					coded_input_stream.push_limit (
						bundle_info_length)));

		let bundle_info =
			try! (
				protobuf_result (
					protobuf::core::parse_from::<proto::BundleInfo> (
						& mut coded_input_stream)));

		coded_input_stream.pop_limit (
			bundle_info_old_limit);

		index_entries.push ( (
			index_bundle_header,
			bundle_info) );

	}

	Ok (index_entries)

}

pub fn read_bundle <P: AsRef <Path>> (
	path: P,
) -> Result <Vec <([u8; 24], Vec <u8>)>, String> {

	// open file

	let input =
		try! (
			io_result (
				File::open (
					path)));

	let mut buf_input =
		BufReader::new (
			input);

	let bundle_info = {

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				&mut buf_input);

		// read header

		let header_length =
			try! (
				protobuf_result (
					coded_input_stream.read_raw_varint32 ()));

		let header_old_limit =
			try! (
				protobuf_result (
					coded_input_stream.push_limit (
						header_length)));

		let file_header =
			try! (
				protobuf_result (
					protobuf::core::parse_from::<proto::FileHeader> (
						&mut coded_input_stream)));

		if file_header.get_version () != 1 {

			return Err (
				format! (
					"Unsupported backup version {}",
					file_header.get_version ()));

		}

		coded_input_stream.pop_limit (
			header_old_limit);

		// read bundle infos

		let bundle_info_length =
			try! (
				protobuf_result (
					coded_input_stream.read_raw_varint32 ()));

		let bundle_info_old_limit =
			try! (
				protobuf_result (
					coded_input_stream.push_limit (
						bundle_info_length)));

		let bundle_info =
			try! (
				protobuf_result (
					protobuf::core::parse_from::<proto::BundleInfo> (
						& mut coded_input_stream)));

		coded_input_stream.pop_limit (
			bundle_info_old_limit);

		bundle_info

	};

	// skip checksum

	try! (
		io_result (
			buf_input.seek (
				SeekFrom::Current (4))));
	
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
			Vec::with_capacity (
				chunk_record.get_size () as usize);

		unsafe {

			chunk_bytes.set_len (
				chunk_record.get_size () as usize);

		}

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

// ex: noet ts=4 filetype=rust
