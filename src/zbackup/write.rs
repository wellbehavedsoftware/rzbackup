use std::error::Error;
use std::io;
use std::io::Write;

use adler32::RollingAdler32;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;

use protobuf;
use protobuf::stream::CodedOutputStream;

use rand;
use rand::Rng;

use ::Repository;
use ::TempFileManager;
use ::compress::lzma;
use ::misc::*;
use ::zbackup::crypto::CryptoWriter;
use ::zbackup::data::*;
use ::zbackup::proto;

pub fn write_bundle <
	ProgressCallback: Fn (u64) -> (),
> (
	target: Box <Write>,
	key: Option <[u8; KEY_SIZE]>,
	chunk_ids_and_data: & [(ChunkId, Vec <u8>)],
	progress_callback: ProgressCallback,
) -> Result <proto::BundleInfo, String> {

	let mut target =
		io_result (
			wrap_writer (
				target,
				key),
		) ?;

	let mut bundle_info;

	{

		let mut coded_output_stream =
			CodedOutputStream::new (
				& mut target);

		// write bundle file header

		let mut bundle_file_header =
			proto::BundleFileHeader::new ();

		bundle_file_header.set_version (
			1);

		bundle_file_header.set_compression_method (
			"lzma".to_string ());

		write_message (
			|| "bundle file header".to_string (),
			& mut coded_output_stream,
			& bundle_file_header,
		) ?;

		// write bundle info

		bundle_info =
			proto::BundleInfo::new ();

		for & (chunk_id, ref chunk_data)
		in chunk_ids_and_data.iter () {

			let mut chunk_record =
				proto::BundleInfo_ChunkRecord::new ();

			chunk_record.set_id (
				chunk_id.to_vec ());

			chunk_record.set_size (
				chunk_data.len () as u32);

			bundle_info.mut_chunk_record ().push (
				chunk_record);

		}

		write_message (
			|| "bundle info".to_string (),
			& mut coded_output_stream,
			& bundle_info,
		) ?;

		protobuf_result (
			coded_output_stream.flush ()
		) ?;

	}

	// write checksum

	io_result (
		target.flush (),
	) ?;

	write_adler (
		|| format! (""),
		& mut target,
	) ?;

	// write compressed data

	{

		let mut lzma_writer =
			lzma::LzmaWriter::new (
				& mut target,
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
		target.flush (),
	) ?;

	write_adler (
		|| format! (""),
		& mut target,
	) ?;

	// close file

	io_result (
		target.close ()
	) ?;

	// return

	Ok (bundle_info)

}

pub fn write_index_auto (
	repository: & Repository,
	temp_files: & mut TempFileManager,
	index_entries: & [IndexEntry],
) -> Result <IndexId, String> {

	let index_id =
		index_id_generate ();

	let index_path =
		repository.index_path (
			index_id);

	let index_file =
		Box::new (
			temp_files.create (
				index_path,
			) ?
		);

	write_index (
		index_file,
		repository.encryption_key (),
		& index_entries,
	) ?;

	Ok (index_id)

}

pub fn write_index (
	target: Box <Write>,
	key: Option <[u8; KEY_SIZE]>,
	index_entries: & [IndexEntry],
) -> Result <(), String> {

	let mut target =
		io_result (
			wrap_writer (
				target,
				key),
		) ?;

	{

		let mut coded_output_stream =
			CodedOutputStream::new (
				& mut target);

		// write file header

		let mut file_header =
			proto::FileHeader::new ();

		file_header.set_version (1);

		write_message (
			|| "file header".to_string (),
			& mut coded_output_stream,
			& file_header,
		) ?;

		// write index entries

		let mut index = 0;

		for & (ref index_bundle_header, ref index_bundle_info)
		in index_entries.iter () {

			write_message (
				|| format! (
					"index bundle header {}",
					index),
				& mut coded_output_stream,
				index_bundle_header,
			) ?;

			write_message (
				|| format! (
					"index bundle info {}",
					index),
				& mut coded_output_stream,
				index_bundle_info,
			) ?;

			index += 1;

		}

		let terminal_index_bundle_header =
			proto::IndexBundleHeader::new ();

		write_message (
			|| "terminal index bundle header".to_string (),
			& mut coded_output_stream,
			& terminal_index_bundle_header,
		) ?;

		protobuf_result (
			coded_output_stream.flush ()
		) ?;

	}

	// write checksum

	io_result (
		target.flush (),
	) ?;

	write_adler (
		|| format! (""),
		& mut target,
	) ?;

	// close file

	io_result (
		target.close ()
	) ?;

	// return

	Ok (())

}

fn write_message <
	NameFunction: Fn () -> String,
	Type: protobuf::MessageStatic,
> (
	name_function: NameFunction,
	coded_output_stream: & mut CodedOutputStream,
	message: & Type,
) -> Result <(), String> {

	// write size

	protobuf_result_with_prefix (
		|| format! (
			"Error writing {} size",
			name_function ()),
		coded_output_stream.write_raw_varint32 (
			message.compute_size ()),
	) ?;

	// write message

	protobuf_result_with_prefix (
		|| format! (
			"Error writing {}",
			name_function ()),
		message.write_to_with_cached_sizes (
			coded_output_stream),
	) ?;

	// return

	Ok (())

}

pub fn wrap_writer (
	target: Box <Write>,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <AdlerWrite, io::Error> {

	Ok (match key {

		Some (key) => {

			let mut crypto_writer = (
				CryptoWriter::wrap (
					target,
					key)
			) ?;

			let initialisation_vector: Vec <u8> =
				rand::thread_rng ()
					.gen_iter::<u8> ()
					.take (IV_SIZE)
					.collect ();

			crypto_writer.write (
				& initialisation_vector,
			) ?;

			let mut adler_write =
				AdlerWrite::new (
					Box::new (
						crypto_writer));

			adler_write.update (
				& initialisation_vector);

			adler_write

		},

		None =>
			AdlerWrite::new (
				Box::new (
					CloseableWriter::wrap (
						target))),

	})

}

fn write_adler <
	PrefixFunction: Fn () -> String,
> (
	prefix_function: PrefixFunction,
	adler_write: & mut AdlerWrite,
) -> Result <(), String> {

	// calculate and write hash

	let calculated_hash =
		adler_write.hash ();

	io_result_with_prefix (
		|| format! (
			"{}Error writing adler32 checksum: ",
			prefix_function ()),
		adler_write.write_u32::<LittleEndian> (
			calculated_hash),
	) ?;

	// return ok

	Ok (())

}

pub struct AdlerWrite {
	target: Box <CloseableWrite>,
	adler: RollingAdler32,
	byte_count: usize,
}

impl AdlerWrite {

	fn new (
		target: Box <CloseableWrite>,
	) -> AdlerWrite {

		AdlerWrite {
			target: target,
			adler: RollingAdler32::new (),
			byte_count: 0,
		}

	}

	#[ allow (dead_code) ]
	fn byte_count (& self) -> usize {
		self.byte_count
	}

	fn hash (& self) -> u32 {
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

impl Write for AdlerWrite {

	fn write (
		& mut self,
		buffer: & [u8],
	) -> Result <usize, io::Error> {

		match self.target.write (
			buffer) {

			Ok (write_size) => {

				self.adler.update_buffer (
					& buffer [0 .. write_size]);

				self.byte_count +=
					write_size;

				Ok (write_size)

			},

			Err (error) =>
				Err (error),

		}

	}

	fn flush (
		& mut self,
	) -> Result <(), io::Error> {

		self.target.flush ()

	}

}

impl CloseableWrite for AdlerWrite {

	fn close (
		& mut self,
	) -> Result <(), io::Error> {

		self.target.close ()

	}

}

// ex: noet ts=4 filetype=rust
