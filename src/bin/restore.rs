#[ macro_use ]
extern crate rzbackup;

use std::env;
use std::io::stdout;
use std::process;

use rzbackup::Repository;

fn main () {

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 4 {

		println! (
			"Syntax: {} REPOSITORY-PATH PASSWORD-FILE-PATH BACKUP",
			arguments [0]);

		process::exit (1);

	}

	let repository_path =
		& arguments [1];

	let password_file_path =
		& arguments [2];

	let backup_name =
		& arguments [3];

	let mut repository =
		match Repository::open (
			repository_path,
			Some (password_file_path)) {

		Ok (repository) =>
			repository,

		Err (error) => {

			println! (
				"Error opening repository: {}",
				error);

			process::exit (1);

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

			process::exit (0);

		},

		Err (error) => {

			stderrln! (
				"Error performing restore: {}",
				error);

			process::exit (1);

		},

	}

}

// ex: noet ts=4 filetype=rust
