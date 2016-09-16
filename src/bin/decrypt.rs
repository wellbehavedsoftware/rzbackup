#[ macro_use ]
extern crate rzbackup;

use std::env;
use std::io;
use std::process;

use rzbackup::CryptoReader;
use rzbackup::Repository;

fn main () {

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 4 {

		stderrln! (
			"Syntax: {} REPOSITORY-PATH PASSWORD-FILE-PATH ENCRYPTED-FILE-PATH",
			arguments [0]);

		process::exit (1);

	}

	let repository_path =
		& arguments [1];

	let password_file_path =
		& arguments [2];

	let encrypted_file_path =
		& arguments [3];

	let repository =
		match Repository::open (
			repository_path,
			Some (password_file_path)) {

		Ok (repository) =>
			repository,

		Err (error) => {

			stderrln! (
				"Error opening repository: {}",
				error);

			process::exit (1);

		},

	};

	let encryption_key =
		match repository.encryption_key () {

		Some (encryption_key) =>
			encryption_key,

		None => {

			stderrln! (
				"Repository metadata does not contain an encryption key");

			process::exit (1);

		},

	};

	let mut input =
		match CryptoReader::open (
			encrypted_file_path,
			encryption_key) {

		Ok (input) =>
			input,

		Err (error) => {

			stderrln! (
				"Error opening encrypted file: {}",
				error);

			process::exit (1);

		},

	};

	match io::copy (
		& mut input,
		& mut io::stdout ()) {

		Ok (input) =>
			input,

		Err (error) => {

			stderrln! (
				"Error decrypting file: {}",
				error);

			process::exit (1);

		},

	};

	process::exit (0);

}

// ex: noet ts=4 filetype=rust
