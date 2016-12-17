extern crate output;
extern crate rzbackup;

use std::env;
use std::io;
use std::process;

use rzbackup::CryptoReader;
use rzbackup::Repository;

fn main () {

	let output =
		output::open ();

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () != 4 {

		output.message_format (
			format_args! (
				"Syntax: {} REPOSITORY-PATH PASSWORD-FILE-PATH ENCRYPTED-FILE-PATH",
				arguments [0]));

		process::exit (1);

	}

	let repository_path =
		& arguments [1];

	let password_file_path =
		& arguments [2];

	let encrypted_file_path =
		& arguments [3];

	output.status_format (
		format_args! (
			"Loading repository {} ...",
			repository_path));

	let repository =
		match Repository::open (
			& output,
			Repository::default_config (),
			repository_path,
			Some (password_file_path)) {

		Ok (repository) =>
			repository,

		Err (error) => {

			output.message_format (
				format_args! (
					"Error opening repository: {}",
					error));

			process::exit (1);

		},

	};

	output.status_done ();

	let encryption_key =
		match repository.encryption_key () {

		Some (encryption_key) =>
			encryption_key,

		None => {

			output.message (
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

			output.message_format (
				format_args! (
					"Error opening encrypted file: {}",
					error));

			process::exit (1);

		},

	};

	match io::copy (
		& mut input,
		& mut io::stdout ()) {

		Ok (input) =>
			input,

		Err (error) => {

			output.message_format (
				format_args! (
					"Error decrypting file: {}",
					error));

			process::exit (1);

		},

	};

	process::exit (0);

}

// ex: noet ts=4 filetype=rust
