use std::fs;
use std::path::Path;

use ::misc::*;

pub fn scan_index_files <
	RepositoryPath: AsRef <Path>,
> (
	repository_path: RepositoryPath,
) -> Result <Vec <(String, u64)>, String> {

	let repository_path =
		repository_path.as_ref ();

	let mut indexes_and_sizes: Vec <(String, u64)> =
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
			file_name.to_str ().unwrap ().to_owned ();

		let index_metadata = (

			io_result (
				fs::metadata (
					dir_entry.path ()))

		) ?;

		indexes_and_sizes.push (
			(
				index_name,
				index_metadata.len (),
			)
		);

	}

	// return

	Ok (indexes_and_sizes)

}

// ex: noet ts=4 filetype=rust
