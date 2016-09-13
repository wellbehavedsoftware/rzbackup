extern crate rzbackup;

use std::env;
use std::process;

use rzbackup::Repository;

fn main () {

	println! (
		"RZBackup server starting");

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 3 {

		println! (
			"Syntax: {} LISTEN-ADDRESS REPOSITORY",
			arguments [0]);

		process::exit (1);

	}

	let listen_address =
		& arguments [1];

	let repository_path =
		& arguments [2];

	let repository =
		match Repository::open (
			repository_path) {

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
