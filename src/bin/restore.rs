#[ macro_use ]
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

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 4 {

		println! (
			"Syntax: {} REPOSITORY PASSWORD-FILE BACKUP",
			arguments [0]);

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
			Repository::default_config (),
			repository_path,
			Some (password_file_path)) {

		Ok (repository) =>
			repository,

		Err (error) => {

			println! (
				"Error opening repository: {}",
				error);

			return 1;

		},

	};

	let stdout_value =
		stdout ();

	let mut stdout_lock =
		stdout_value.lock ();

	match repository.restore (
		backup_name,
		& mut stdout_lock) {

		Ok (_) => {

			return 0;

		},

		Err (error) => {

			stderrln! (
				"Error performing restore: {}",
				error);

			return 1;

		},

	}

}

// ex: noet ts=4 filetype=rust
