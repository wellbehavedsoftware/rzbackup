extern crate output;
extern crate rzbackup;

use std::env;
use std::io::stdout;
use std::process;

use rzbackup::Repository;

fn main () {

	process::exit (
		main_real (),
	);

}

fn main_real (
) -> i32 {

	let output =
		output::open ();

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 4 {

		output.message_format (
			format_args! (
				"Syntax: {} REPOSITORY PASSWORD-FILE BACKUP",
				arguments [0]));

		return 1;

	}

	let repository_path =
		& arguments [1];

	let password_file_path =
		& arguments [2];

	let backup_name =
		& arguments [3];

	let repository =
		match Repository::open (
			& output,
			Repository::default_config (),
			repository_path,
			if password_file_path != "" {
				Some (password_file_path)
			} else {
				None
			}) {

		Ok (repository) =>
			repository,

		Err (error) => {

			output.message_format (
				format_args! (
					"Error opening repository: {}",
					error));

			return 1;

		},

	};

	let stdout_value =
		stdout ();

	let mut stdout_lock =
		stdout_value.lock ();

	match repository.restore (
		& output,
		backup_name,
		& mut stdout_lock) {

		Ok (_) => {

			return 0;

		},

		Err (error) => {

			output.message_format (
				format_args! (
					"Error performing restore: {}",
					error));

			return 1;

		},

	}

}

// ex: noet ts=4 filetype=rust
