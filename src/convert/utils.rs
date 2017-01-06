use std::fs;
use std::path::Path;
use std::path::PathBuf;

use rustc_serialize::hex::ToHex;

use ::Repository;
use ::TempFileManager;
use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::write::*;

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

		let file_name =
			dir_entry.file_name ();

		let index_name =
			file_name.to_string_lossy ();

		let index_id =
			index_id_parse (
				index_name.as_ref (),
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

		let file_name =
			dir_entry.file_name ();

		let index_name =
			file_name.to_string_lossy ();

		let index_id =
			index_id_parse (
				index_name.as_ref (),
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

			let bundle_id =
				bundle_id_parse (
					bundle_name.as_ref (),
				) ?;

			bundle_ids.push (
				bundle_id);

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
				bundle_id_parse (
					bundle_name.as_ref (),
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
	repository: & Repository,
	temp_files: & mut TempFileManager,
	index_entries_buffer: & mut Vec <IndexEntry>,
) -> Result <IndexId, String> {

	let index_id =
		write_index_auto (
			repository,
			temp_files,
			& index_entries_buffer,
		) ?;

	index_entries_buffer.clear ();

	Ok (index_id)

}

// ex: noet ts=4 filetype=rust
