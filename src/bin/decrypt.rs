#![ allow (unused_parens) ]

extern crate clap;
extern crate output;
extern crate rzbackup;

use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use rzbackup::misc::args;
use rzbackup::CryptoReader;
use rzbackup::Repository;

fn main () {

	let output =
		output::open ();

	let arguments =
		parse_arguments ();

	let repository =
		match Repository::open (
			& output,
			Repository::default_config (),
			& arguments.repository_path,
			Some (arguments.password_file_path)) {

		Ok (repository) =>
			repository,

		Err (error) => {

			output.message_format (
				format_args! (
					"Error opening repository {}: {}",
					arguments.repository_path.to_string_lossy (),
					error));

			process::exit (1);

		},

	};

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

	output.status_format (
		format_args! (
			"Decrypting file {} ...",
			arguments.encrypted_file_path.to_string_lossy ()));

	let mut input =
		match CryptoReader::open (
			arguments.encrypted_file_path,
			encryption_key) {

		Ok (input) =>
			input,

		Err (error) => {

			output.clear_status ();

			output.message_format (
				format_args! (
					"Error opening encrypted file: {}",
					error));

			process::exit (1);

		},

	};

	if ! arguments.include_iv {

		let mut iv_buffer: [u8; rzbackup::KEY_SIZE] =
			[0u8; rzbackup::KEY_SIZE];

		if let Err (error) =
			input.read_exact (
				& mut iv_buffer) {

			output.clear_status ();

			output.message_format (
				format_args! (
					"Error opening encrypted file: {}",
					error));

			process::exit (1);

		}

	}

	match io::copy (
		& mut input,
		& mut io::stdout ()) {

		Ok (input) =>
			input,

		Err (error) => {

			output.clear_status ();

			output.message_format (
				format_args! (
					"Error decrypting file: {}",
					error));

			process::exit (1);

		},

	};

	output.status_done ();

	process::exit (0);

}

struct Arguments {
	repository_path: PathBuf,
	password_file_path: PathBuf,
	encrypted_file_path: PathBuf,
	include_iv: bool,
}

fn parse_arguments (
) -> Arguments {

	let clap_application = (
		clap::App::new ("RZBackup-decrypt")

		.version (rzbackup::VERSION)
		.author (rzbackup::AUTHOR)
		.about ("Extracts decrypted versions of encrypted zbackup files")

		.arg (
			clap::Arg::with_name ("repository")

			.index (1)
			.value_name ("REPOSITORY")
			.required (true)
			.help ("Path to the repository, used to obtain encryption key")

		)

		.arg (
			clap::Arg::with_name ("password-file")

			.index (2)
			.value_name ("PASSWORD-FILE")
			.required (true)
			.help ("Path to the password file")

		)

		.arg (
			clap::Arg::with_name ("encrypted-file")

			.index (3)
			.value_name ("ENCRYPTED-FILE")
			.required (true)
			.help ("Path to the encrypted file")

		)

		.arg (
			clap::Arg::with_name ("include-iv")

			.long ("include-iv")
			.help ("Include the decrypted initialisation vector")

		)

	);

	let clap_matches =
		clap_application.get_matches ();

	Arguments {

		repository_path:
			args::path_required (
				& clap_matches,
				"repository"),

		password_file_path:
			args::path_required (
				& clap_matches,
				"password-file"),

		encrypted_file_path:
			args::path_required (
				& clap_matches,
				"encrypted-file"),

		include_iv:
			args::bool_flag (
				& clap_matches,
				"include-iv"),

	}

}

// ex: noet ts=4 filetype=rust
