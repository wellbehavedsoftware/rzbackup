use std::io::Write;
use std::path::Path;

use output;
use output::OutputJob;

use protobuf::stream::CodedInputStream;
use protobuf::stream::CodedOutputStream;

use misc::*;
use zbackup::data::*;
use zbackup::disk_format::*;
use zbackup::repository_core::*;

pub struct DiskIndexBundleHeader {
	raw: protobuf_types::IndexBundleHeader,
}

impl DiskIndexBundleHeader {

	#[ inline ]
	pub fn new (
		bundle_id: BundleId,
	) -> DiskIndexBundleHeader {

		let mut raw =
			protobuf_types::IndexBundleHeader::new ();

		raw.set_id (
			bundle_id.into_vec ());

		DiskIndexBundleHeader {
			raw: raw,
		}

	}

	#[ inline ]
	pub fn terminal (
	) -> DiskIndexBundleHeader {

		let raw =
			protobuf_types::IndexBundleHeader::new ();

		DiskIndexBundleHeader {
			raw: raw,
		}

	}

	#[ inline ]
	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskIndexBundleHeader, String> {

		Ok (DiskIndexBundleHeader {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"index bundle header"),
			) ?,
		})

	}

	#[ inline ]
	pub fn write (
		& self,
		coded_output_stream: & mut CodedOutputStream,
	) -> Result <(), String> {

		protobuf_message_write (
			|| "index bundle header".to_string (),
			coded_output_stream,
			& self.raw,
		)

	}

	#[ inline ]
	pub fn has_bundle_id (& self) -> bool {
		self.raw.has_id ()
	}

	#[ inline ]
	pub fn bundle_id (& self) -> BundleId {
		BundleId::from_slice (
			self.raw.get_id (),
		).unwrap ()
	}

}

pub fn index_read_path <
	IndexPath: AsRef <Path>,
> (
	index_path: IndexPath,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <RawIndexEntry>, String> {

	index_read_path_impl (
		index_path.as_ref (),
		key,
	)

}

fn index_read_path_impl (
	index_path: & Path,
	key: Option <[u8; KEY_SIZE]>,
) -> Result <Vec <RawIndexEntry>, String> {

	let mut index_entries: Vec <RawIndexEntry> =
		vec! ();

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				index_path.to_string_lossy ()),
			file_open_with_crypto_and_adler (
				index_path,
				key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read header

		let file_header: protobuf_types::FileHeader =
			string_result_with_prefix (
				|| format! (
					"Error reading {}: ",
					index_path.to_string_lossy ()),
				protobuf_message_read (
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

		loop {

			let index_bundle_header =
				string_result_with_prefix (
					|| format! (
						"Error reading {}: ",
						index_path.to_string_lossy ()),
					DiskIndexBundleHeader::read (
						& mut coded_input_stream,
					),
				) ?;

			if ! index_bundle_header.has_bundle_id () {
				break;
			}

			let bundle_info =
				string_result_with_prefix (
					|| format! (
						"Error reading {}: ",
						index_path.to_string_lossy ()),
					DiskBundleInfo::read (
						& mut coded_input_stream,
					)
				) ?;

			index_entries.push (
				RawIndexEntry {
					index_bundle_header: index_bundle_header,
					bundle_info: bundle_info,
				},
			);

		}

	}

	// verify checksum

	adler_verify_hash_and_eof (
		|| format! (
			"Error reading {}: ",
			index_path.to_string_lossy ()),
		source,
	) ?;

	Ok (index_entries)

}

pub fn index_write_auto (
	repository_core: & RepositoryCore,
	atomic_file_writer: & AtomicFileWriter,
	index_entries: & [RawIndexEntry],
) -> Result <IndexId, String> {

	let index_id =
		IndexId::random ();

	let index_path =
		repository_core.index_path (
			index_id);

	let mut index_file =
		atomic_file_writer.create (
			index_path,
		) ?;

	index_write_direct (
		& mut index_file,
		repository_core.encryption_key (),
		& index_entries,
	) ?;

	Ok (index_id)

}

#[ inline ]
pub fn index_write_with_id (
	repository_core: & RepositoryCore,
	atomic_file_writer: & AtomicFileWriter,
	index_id: IndexId,
	index_entries: & [RawIndexEntry],
) -> Result <(), String> {

	let index_path =
		repository_core.index_path (
			index_id);

	let mut index_file =
		atomic_file_writer.create (
			index_path,
		) ?;

	index_write_direct (
		& mut index_file,
		repository_core.encryption_key (),
		index_entries,
	)

}

#[ inline ]
pub fn index_write_direct (
	target: & mut Write,
	key: Option <[u8; KEY_SIZE]>,
	index_entries: & [RawIndexEntry],
) -> Result <(), String> {

	let output =
		output::null ();

	let output_job =
		output_job_start! (
			output,
			"");

	write_index_output_job (
		& output_job,
		target,
		key,
		index_entries,
	)

}

pub fn write_index_output_job (
	output_job: & OutputJob,
	target: & mut Write,
	key: Option <[u8; KEY_SIZE]>,
	index_entries: & [RawIndexEntry],
) -> Result <(), String> {

	let mut target =
		io_result (
			writer_wrap_with_crypto_and_adler (
				target,
				key),
		) ?;

	{

		let mut coded_output_stream =
			CodedOutputStream::new (
				& mut target);

		// write file header

		let file_header =
			DiskFileHeader::new (
				1,
			);

		file_header.write (
			& mut coded_output_stream,
		) ?;

		// write index entries

		let mut entries_count: u64 = 0;
		let entries_total = index_entries.len () as u64;

		for & RawIndexEntry {
			ref index_bundle_header,
			ref bundle_info,
		} in index_entries.iter () {

			output_job.progress (
				entries_count,
				entries_total);

			index_bundle_header.write (
				& mut coded_output_stream,
			) ?;

			bundle_info.write (
				& mut coded_output_stream,
			) ?;

			entries_count += 1;

		}

		let terminal_index_bundle_header =
			DiskIndexBundleHeader::terminal ();

		terminal_index_bundle_header.write (
			& mut coded_output_stream,
		) ?;

		protobuf_result (
			coded_output_stream.flush ()
		) ?;

	}

	// write checksum

	io_result (
		target.flush (),
	) ?;

	adler_write_hash (
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

// ex: noet ts=4 filetype=rust
