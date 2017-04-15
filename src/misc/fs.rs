use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn rename_or_copy_and_delete <
	SourcePath: AsRef <Path>,
	TargetPath: AsRef <Path>,
> (
	source_path: SourcePath,
	target_path: TargetPath,
) -> Result <(), String> {

	rename_or_copy_and_delete_impl (
		source_path.as_ref (),
		target_path.as_ref (),
	)

}

fn rename_or_copy_and_delete_impl (
	source_path: & Path,
	target_path: & Path,
) -> Result <(), String> {

	// try a simple rename

	if let Ok (()) = (
		fs::rename (
			source_path,
			target_path)
	) {
		return Ok (());
	}

	// copy the file contents

	{

		let mut source =
			File::open (
				source_path,
			).map_err (|error|
				format! (
					"Error opening source file: {}",
					error)
			) ?;

		let target_temp_path =
			target_path.with_extension (
				"temp");

		let mut target_temp =
			File::create (
				& target_temp_path,
			).map_err (|error|
				format! (
					"Error opening target temp file {}: {}",
					target_temp_path.to_string_lossy (),
					error)
			) ?;

		io::copy (
			& mut source,
			& mut target_temp,
		).map_err (|error|
			format! (
				"Error copying contents to target temp file {}: {}",
				target_temp_path.to_string_lossy (),
				error)
		) ?;

		target_temp.sync_all (
		).map_err (|error|
			format! (
				"Error syncing target temp file {}: {}",
				target_temp_path.to_string_lossy (),
				error)
		) ?;

		fs::rename (
			& target_temp_path,
			target_path,
		).map_err (|error|
			format! (
				"Error renaming temp file {}: {}",
				target_temp_path.to_string_lossy (),
				error)
		) ?;

	}

	// delete the original

	fs::remove_file (
		source_path,
	).map_err (|error|
		format! (
			"Error removing source file: {}",
			error)
	) ?;

	Ok (())

}

// ex: noet ts=4 filetype=rust
