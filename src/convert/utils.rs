use std::collections::HashSet;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;

use rust_crypto::sha1::Sha1;

use output::Output;

use protobuf::stream::CodedInputStream;

use rustc_serialize::hex::ToHex;

use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::disk_format::*;
use ::zbackup::repository::*;
use ::zbackup::repository_core::*;

pub fn scan_index_files <
	RepositoryPath: AsRef <Path>,
> (
	repository_path: RepositoryPath,
) -> Result <Vec <IndexId>, String> {

	let repository_path =
		repository_path.as_ref ();

	let mut index_ids: Vec <IndexId> =
		Vec::new ();

	// read directory

	for dir_entry_result in (

		io_result (
			fs::read_dir (
				repository_path.join (
					"index")))

	) ? {

		let dir_entry = (

			io_result (
				dir_entry_result)

		) ?;

		let index_id =
			IndexId::parse (
				dir_entry.file_name ().to_string_lossy (),
			) ?;

		index_ids.push (
			index_id);

	}

	// return

	Ok (index_ids)

}

pub fn scan_index_files_with_sizes <
	RepositoryPath: AsRef <Path>,
> (
	repository_path: RepositoryPath,
) -> Result <Vec <(IndexId, u64)>, String> {

	let repository_path =
		repository_path.as_ref ();

	let mut index_ids_and_sizes: Vec <(IndexId, u64)> =
		Vec::new ();

	// read directory

	let indexes_path =
		repository_path.join (
			"index");

	for dir_entry_result in
		io_result_with_prefix (
			|| format! (
				"Error opening directory {}: ",
				indexes_path.to_string_lossy ()),
			fs::read_dir (
				& indexes_path),
		) ? {

		let dir_entry =
			io_result_with_prefix (
				|| format! (
					"Error reading directory {}: ",
					indexes_path.to_string_lossy ()),
				dir_entry_result,
			) ?;

		let index_id =
			IndexId::parse (
				dir_entry.file_name ().to_string_lossy (),
			) ?;

		let index_metadata =
			io_result_with_prefix (
				|| format! (
					"Error getting metadata for {}",
					dir_entry.path ().to_string_lossy ()),
				fs::metadata (
					dir_entry.path ()),
			) ?;

		index_ids_and_sizes.push (
			(
				index_id,
				index_metadata.len (),
			)
		);

	}

	// return

	Ok (index_ids_and_sizes)

}

pub fn scan_backup_files <
	RepositoryPath: AsRef <Path>,
> (
	repository_path: RepositoryPath,
) -> Result <Vec <PathBuf>, String> {

	let repository_path =
		repository_path.as_ref ();

	let mut backup_files: Vec <PathBuf> =
		Vec::new ();

	let backups_root =
		repository_path.join (
			"backups");

	scan_backup_files_real (
		& mut backup_files,
		& backups_root,
		& PathBuf::new (),
	) ?;

	Ok (backup_files)

}

fn scan_backup_files_real (
	backup_files: & mut Vec <PathBuf>,
	backups_root: & Path,
	directory: & Path,
) -> Result <(), String> {

	for dir_entry_result in (
		io_result (
			fs::read_dir (
				backups_root.join (
					directory)))
	) ? {

		let dir_entry = (
			io_result (
				dir_entry_result)
		) ?;

		let entry_metadata = (
			io_result (
				fs::metadata (
					dir_entry.path ()))
		) ?;

		if entry_metadata.is_dir () {

			scan_backup_files_real (
				backup_files,
				backups_root,
				& directory.join (
					dir_entry.file_name ()),
			) ?;

		} else if entry_metadata.is_file () {

			backup_files.push (
				directory.join (
					dir_entry.file_name ()));

		} else {

			panic! (
				"Don't know how to handle {:?}: {}",
				entry_metadata.file_type (),
				dir_entry.path ().to_string_lossy ());

		}

	}

	// return

	Ok (())

}

pub fn scan_bundle_files <
	RepositoryPath: AsRef <Path>,
> (
	output: & Output,
	repository_path: RepositoryPath,
) -> Result <Vec <BundleId>, String> {

	let repository_path =
		repository_path.as_ref ();

	let mut bundle_ids: Vec <BundleId> =
		Vec::new ();

	for prefix in (0 .. 256).map (
		|byte| [ byte as u8 ].to_hex ()
	) {

		let bundles_directory =
			repository_path
				.join ("bundles")
				.join (prefix);

		if ! bundles_directory.exists () {
			continue;
		}

		for dir_entry_result in (
			io_result (
				fs::read_dir (
					bundles_directory))
		) ? {

			let dir_entry =
				io_result (
					dir_entry_result,
				) ?;

			let file_name =
				dir_entry.file_name ();

			let bundle_name =
				file_name.to_string_lossy ();

			match BundleId::parse (
				& bundle_name,
			) {

				Ok (bundle_id) =>
					bundle_ids.push (
						bundle_id),

				Err (_) =>
					output.message_format (
						format_args! (
							"Ignoring invalid bundle name: {}",
							bundle_name)),

			}

		}

	}

	Ok (bundle_ids)

}

pub fn scan_bundle_files_with_sizes <
	RepositoryPath: AsRef <Path>,
> (
	repository_path: RepositoryPath,
) -> Result <Vec <(BundleId, u64)>, String> {

	let repository_path =
		repository_path.as_ref ();

	let mut bundle_ids_and_sizes: Vec <(BundleId, u64)> =
		Vec::new ();

	for prefix in (0 .. 256).map (
		|byte| [ byte as u8 ].to_hex ()
	) {

		let bundles_directory =
			repository_path
				.join ("bundles")
				.join (prefix);

		if ! bundles_directory.exists () {
			continue;
		}

		for dir_entry_result in (
			io_result (
				fs::read_dir (
					bundles_directory))
		) ? {

			let dir_entry =
				io_result (
					dir_entry_result,
				) ?;

			let file_name =
				dir_entry.file_name ();

			let bundle_name =
				file_name.to_string_lossy ();

			let bundle_id =
				BundleId::parse (
					bundle_name,
				) ?;

			let bundle_metadata =
				io_result_with_prefix (
					|| format! (
						"Error getting metadata for {}",
						dir_entry.path ().to_string_lossy ()),
					fs::metadata (
						dir_entry.path ()),
				) ?;

			bundle_ids_and_sizes.push (
				(
					bundle_id,
					bundle_metadata.len (),
				)
			);

		}

	}

	Ok (bundle_ids_and_sizes)

}

pub fn flush_index_entries (
	output: & Output,
	repository_core: & RepositoryCore,
	atomic_file_writer: & AtomicFileWriter,
	index_entries_buffer: & Vec <RawIndexEntry>,
) -> Result <IndexId, String> {

	let index_id =
		IndexId::random ();

	let output_job =
		output_job_start! (
			output,
			"Writing index {}",
			index_id);

	index_write_auto (
		repository_core,
		atomic_file_writer,
		& index_entries_buffer,
	) ?;

	output_job.remove ();

	Ok (index_id)

}

pub fn collect_chunks_from_backup (
	repository: & Repository,
	chunk_ids: & mut HashSet <ChunkId>,
	backup_name: & Path,
) -> Result <(), String> {

	// load backup

	let backup_info =
		backup_read_path (
			repository.path ()
				.join ("backups")
				.join (backup_name),
			repository.encryption_key (),
		) ?;

	// collect chunk ids

	collect_chunks_from_instructions (
		chunk_ids,
		& backup_info.backup_data (),
	) ?;

	// expand backup data

	let mut input =
		Cursor::new (
			backup_info.backup_data ().to_owned ());

	for _iteration in 0 .. backup_info.iterations () {

		let mut temp_output: Cursor <Vec <u8>> =
			Cursor::new (
				Vec::new ());

		let mut sha1_digest =
			Sha1::new ();

		repository.follow_instructions (
			& mut input,
			& mut temp_output,
			& mut sha1_digest,
			& |_count| (),
		) ?;

		let result =
			temp_output.into_inner ();

		// collect chunk ids

		collect_chunks_from_instructions (
			chunk_ids,
			& result,
		) ?;

		// prepare for next iteration

		input =
			Cursor::new (
				result);

	}

	Ok (())

}

pub fn collect_chunks_from_instructions (
	chunk_ids: & mut HashSet <ChunkId>,
	instructions: & [u8],
) -> Result <(), String> {

	let mut instructions_cursor =
		Cursor::new (
			& instructions);

	let mut coded_input_stream =
		CodedInputStream::new (
			& mut instructions_cursor);

	while ! protobuf_result (
		coded_input_stream.eof (),
	) ? {

		let backup_instruction =
			DiskBackupInstruction::read (
				& mut coded_input_stream,
			) ?;

		if backup_instruction.has_chunk_to_emit () {

			chunk_ids.insert (
				backup_instruction.chunk_to_emit ());

		}

	}

	Ok (())

}

// ex: noet ts=4 filetype=rust
