extern crate rzbackup;

use std::env;
use std::process;

use rzbackup::Repository;

fn main () {

	println! (
		"RZBackup server starting");

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 3
	&& arguments.len () != 4 {

		println! (
			"Syntax: {} LISTEN-ADDRESS REPOSITORY [PASSWORD-FILE]",
			arguments [0]);

		process::exit (1);

	}

	let listen_address =
		& arguments [1];

	let repository_path =
		& arguments [2];

	let password_file_path: Option <& str> =
		if arguments.len () >= 4 {
			Some (& arguments [3])
		} else {
			None
		};

	let repository =
		match Repository::open (
			Repository::default_config (),
			repository_path,
			password_file_path) {

		Ok (repository) =>
			repository,

		Err (error) => {

			println! (
				"Error opening repository: {}",
				error);

			process::exit (1);

		},

	};

	println! (
		"RZBackup startup complete");

	match rzbackup::run_server (
		repository,
		listen_address) {

		Ok (_) =>
			(),

		Err (error) => {

			println! (
				"RZBackup server encountered error: {}",
				error);

			process::exit (1);

		},

	};

	println! (
		"RZBackup server terminating");

}

// ex: noet ts=4 filetype=rust
