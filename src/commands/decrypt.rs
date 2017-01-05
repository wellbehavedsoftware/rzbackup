use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use clap;

use output::Output;

use ::CryptoReader;
use ::Repository;
use ::misc::*;

pub fn decrypt_command (
) -> Box <Command> {

	Box::new (
		DecryptCommand {}
	)

}

pub struct DecryptArguments {
	repository_path: PathBuf,
	password_file_path: PathBuf,
	encrypted_file_path: PathBuf,
	include_iv: bool,
}

pub struct DecryptCommand {
}

fn do_decrypt (
	output: & Output,
	arguments: & DecryptArguments,
) -> Result <(), String> {

	let repository =
		string_result_with_prefix (
			|| format! (
				"Error opening repository: {}",
				arguments.repository_path.to_string_lossy ()),
			Repository::open (
				& output,
				Repository::default_config (),
				& arguments.repository_path,
				Some (& arguments.password_file_path)),
		) ?;

	let encryption_key =
		repository.encryption_key (
		).ok_or_else (||

			format! (
				"Repository metadata does not contain an encryption key")

		) ?;

	output.status_format (
		format_args! (
			"Decrypting file {} ...",
			arguments.encrypted_file_path.to_string_lossy ()));

	let mut input =
		match CryptoReader::open (
			& arguments.encrypted_file_path,
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

		let mut iv_buffer: [u8; ::KEY_SIZE] =
			[0u8; ::KEY_SIZE];

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

	Ok (())

}

impl CommandArguments for DecryptArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		do_decrypt (
			output,
			self,
		)

	}

}

impl Command for DecryptCommand {

	fn name (& self) -> & 'static str {
		"decrypt"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

		clap::SubCommand::with_name ("decrypt")
			.about ("Decrypts an encrypted file in a ZBackup repository")

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

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		let arguments = DecryptArguments {

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

		};

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
