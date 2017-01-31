use std::error::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::slice;

use protobuf::stream::CodedInputStream;
use protobuf::stream::CodedOutputStream;

use rust_crypto::digest::Digest;
use rust_crypto::sha1::Sha1;

use rustc_serialize::hex::ToHex;

use compress::lzma;
use misc::*;
use zbackup::data::*;
use zbackup::disk_format::*;

#[ derive (Clone) ]
pub struct DiskBundleFileHeader {
	raw: protobuf_types::BundleFileHeader,
}

#[ derive (Clone) ]
pub struct DiskBundleInfoChunkRecord {
	raw: protobuf_types::BundleInfo_ChunkRecord,
}

pub struct DiskBundleInfoChunkRecordIter <'a> {
	inner: slice::Iter <'a, protobuf_types::BundleInfo_ChunkRecord>,
}

#[ derive (Clone) ]
pub struct DiskBundleInfo {
	raw: protobuf_types::BundleInfo,
}

impl DiskBundleFileHeader {

	pub fn new (
		version: u32,
		compression_method: String,
	) -> DiskBundleFileHeader {

		let mut raw =
			protobuf_types::BundleFileHeader::new ();

		raw.set_version (
			version);

		raw.set_compression_method (
			compression_method);

		DiskBundleFileHeader {
			raw: raw,
		}

	}

	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskBundleFileHeader, String> {

		Ok (DiskBundleFileHeader {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"bundle header"),
			) ?,
		})

	}

	pub fn write (
		& self,
		coded_output_stream: & mut CodedOutputStream,
	) -> Result <(), String> {

		protobuf_message_write (
			|| "bundle file header".to_string (),
			coded_output_stream,
			& self.raw,
		)

	}

	pub fn version (& self) -> u32 {
		self.raw.get_version ()
	}

	pub fn compression_method (& self) -> & str {
		self.raw.get_compression_method ()
	}

}

impl DiskBundleInfo {

	pub fn new <
		Chunks: IntoIterator <Item = DiskBundleInfoChunkRecord>,
	> (
		chunks: Chunks,
	) -> DiskBundleInfo {

		let mut raw =
			protobuf_types::BundleInfo::new ();

		for chunk in chunks {

			raw.mut_chunk_record ().push (
				chunk.raw);

		}

		DiskBundleInfo {
			raw: raw,
		}

	}

	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskBundleInfo, String> {

		Ok (DiskBundleInfo {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"bundle info"),
			) ?,
		})

	}

	pub fn write (
		& self,
		coded_output_stream: & mut CodedOutputStream,
	) -> Result <(), String> {

		protobuf_message_write (
			|| "bundle info".to_string (),
			coded_output_stream,
			& self.raw,
		)

	}

	pub fn num_chunks (& self) -> u64 {
		self.raw.get_chunk_record ().len () as u64
	}

	pub fn chunks <'a> (
		& 'a self,
	) -> DiskBundleInfoChunkRecordIter <'a> {

		DiskBundleInfoChunkRecordIter {
			inner: self.raw.get_chunk_record ().iter (),
		}

	}

}

impl DiskBundleInfoChunkRecord {

	pub fn new (
		chunk_id: ChunkId,
		size: u32,
	) -> DiskBundleInfoChunkRecord {

		let mut raw =
			protobuf_types::BundleInfo_ChunkRecord::new ();

		raw.set_id (
			chunk_id.into_vec ());

		raw.set_size (
			size);

		DiskBundleInfoChunkRecord {
			raw: raw,
		}

	}

	pub fn chunk_id (& self) -> ChunkId {
		ChunkId::from_slice (
			self.raw.get_id (),
		).unwrap ()
	}

	pub fn size (& self) -> u32 {
		self.raw.get_size ()
	}

}

impl <'a> Iterator for DiskBundleInfoChunkRecordIter <'a> {

	type Item = DiskBundleInfoChunkRecord;

	fn next (
		& mut self,
	) -> Option <DiskBundleInfoChunkRecord> {

		self.inner.next ().map (
			|protobuf_chunk_record|

			DiskBundleInfoChunkRecord {
				raw: protobuf_chunk_record.clone (),
			}

		)

	}

}

#[ inline ]
pub fn bundle_info_read_path <
	BundlePath: AsRef <Path>,
> (
	bundle_path: BundlePath,
	encryption_key: Option <EncryptionKey>,
) -> Result <DiskBundleInfo, String> {

	bundle_info_read_path_impl (
		bundle_path.as_ref (),
		encryption_key,
	)

}

fn bundle_info_read_path_impl (
	bundle_path: & Path,
	key: Option <EncryptionKey>,
) -> Result <DiskBundleInfo, String> {

	let bundle_info: DiskBundleInfo;

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				bundle_path.to_string_lossy ()),
			file_open_with_crypto_and_adler (
				bundle_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read bundle file header

		let bundle_file_header =
			DiskBundleFileHeader::read (
				& mut coded_input_stream,
			) ?;

		if bundle_file_header.version () != 1 {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file version {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.version ()));

		}

		if bundle_file_header.compression_method () != "lzma" {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file compression \
						method {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.compression_method ()));

		}

		// read bundle info

		bundle_info =
			DiskBundleInfo::read (
				& mut coded_input_stream,
			) ?;

	}

	// verify checksum

	adler_verify_hash (
		|| format! (
			"Error reading {}: ",
			bundle_path.to_string_lossy ()),
		& mut source,
	) ?;

	Ok (bundle_info)

}

#[ inline ]
pub fn bundle_read_path <
	BundlePath: AsRef <Path>,
> (
	bundle_path: BundlePath,
	encryption_key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <(ChunkId, Vec <u8>)>, String> {

	bundle_read_path_impl (
		bundle_path.as_ref (),
		encryption_key,
	)

}

pub fn bundle_read_path_impl (
	bundle_path: & Path,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <(ChunkId, Vec <u8>)>, String> {

	let bundle_info: DiskBundleInfo;
	let mut chunks: Vec <(ChunkId, Vec <u8>)>;

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				bundle_path.to_string_lossy ()),
			file_open_with_crypto_and_adler (
				bundle_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read bundle file header

		let bundle_file_header =
			DiskBundleFileHeader::read (
				& mut coded_input_stream,
			) ?;

		if bundle_file_header.version () != 1 {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file version {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.version ()));

		}

		if bundle_file_header.compression_method () != "lzma" {

			return Err (
				format! (
					"Error reading {}: Unsupported bundle file compression \
					method {}",
					bundle_path.to_string_lossy (),
					bundle_file_header.compression_method ()));

		}

		// read bundle info

		bundle_info =
			DiskBundleInfo::read (
				& mut coded_input_stream,
			) ?;

	}

	// verify checksum

	adler_verify_hash (
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

		for chunk in bundle_info.chunks () {

			let chunk_id =
				chunk.chunk_id ();

			// verify sha1 sum

			let mut chunk_bytes: Vec <u8> =
				vec! [0u8; chunk.size () as usize];

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

			if chunk_id.bytes () [0 .. 16] != sha1_sum [0 .. 16] {

				return Err (
					format! (
						"Error reading {}: Invalid sha1 sum for chunk {}: {}",
						bundle_path.to_string_lossy (),
						chunk_id,
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

	adler_verify_hash_and_eof (
		|| format! (
			"Error reading {}: ",
			bundle_path.to_string_lossy ()),
		source,
	) ?;

	Ok (chunks)

}

pub fn bundle_write_direct <
	ProgressCallback: Fn (u64),
> (
	target: & mut Write,
	encryption_key: Option <EncryptionKey>,
	chunk_ids_and_data: & [(ChunkId, Vec <u8>)],
	progress_callback: ProgressCallback,
) -> Result <DiskBundleInfo, String> {

	let mut adler_writer =
		io_result (
			writer_wrap_with_crypto_and_adler (
				target,
				encryption_key),
		) ?;

	let bundle_info;

	{

		let mut coded_output_stream =
			CodedOutputStream::new (
				& mut adler_writer);

		// write bundle file header

		let bundle_file_header =
			DiskBundleFileHeader::new (
				1,
				"lzma".to_string ());

		bundle_file_header.write (
			& mut coded_output_stream,
		) ?;

		// write bundle info

		bundle_info = DiskBundleInfo::new (
			chunk_ids_and_data.iter ().map (
				|& (ref chunk_id, ref chunk_data)|

			DiskBundleInfoChunkRecord::new (
				* chunk_id,
				chunk_data.len () as u32)

		));

		bundle_info.write (
			& mut coded_output_stream,
		) ?;

		protobuf_result (
			coded_output_stream.flush ()
		) ?;

	}

	// write checksum

	io_result (
		adler_writer.flush (),
	) ?;

	adler_write_hash (
		|| format! (""),
		& mut adler_writer,
	) ?;

	// write compressed data

	{

		let mut lzma_writer =
			lzma::LzmaWriter::new (
				& mut adler_writer,
			).map_err (
				|lzma_error|

				format! (
					"Error starting LZMA compression: {}",
					lzma_error)

			) ?;

		let mut chunks_written: u64 = 0;

		for & (_, ref chunk_data)
		in chunk_ids_and_data.iter () {

			lzma_writer.write_all (
				& chunk_data,
			).map_err (
				|io_error|

				format! (
					"Error writing LZMA data: {}",
					io_error.description ())

			) ?;

			// callback

			chunks_written += 1;

			progress_callback (
				chunks_written);

		}

		lzma_writer.close (
		).map_err (
			|lzma_error|

			format! (
				"Error finishing LZMA compression: {}",
				lzma_error.description ())

		) ?;

	}

	// write checksum

	io_result (
		adler_writer.flush (),
	) ?;

	adler_write_hash (
		|| format! (""),
		& mut adler_writer,
	) ?;

	// close file

	io_result (
		adler_writer.close ()
	) ?;

	// return

	Ok (bundle_info)

}

// ex: noet ts=4 filetype=rust
