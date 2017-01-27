use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap;

use output::Output;

use ::CryptoReader;
use ::Repository;
use ::misc::*;

pub fn do_decrypt (
	output: & Output,
	arguments: & DecryptArguments,
) -> Result <bool, String> {

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

	let output_job =
		output_job_start! (
			output,
			"Decrypting file {}",
			arguments.encrypted_file_path.to_string_lossy ());

	let mut input =
		match CryptoReader::open (
			& arguments.encrypted_file_path,
			encryption_key) {

		Ok (input) =>
			input,

		Err (error) => {

			output_job_replace! (
				output_job,
				"Error opening encrypted file: {}",
				error);

			return Ok (false);

		},

	};

	if ! arguments.include_iv {

		let mut iv_buffer: [u8; ::KEY_SIZE] =
			[0u8; ::KEY_SIZE];

		if let Err (error) =
			input.read_exact (
				& mut iv_buffer) {

			output_job_replace! (
				output_job,
				"Error opening encrypted file: {}",
				error);

			return Ok (false);

		}

	}

	match io::copy (
		& mut input,
		& mut io::stdout ()) {

		Ok (input) =>
			input,

		Err (error) => {

			output_job_replace! (
				output_job,
				"Error decrypting file: {}",
				error);

			return Ok (false);

		},

	};

	output_job.complete ();

	Ok (true)

}

command! (

	name = decrypt,
	export = decrypt_command,

	arguments = DecryptArguments {
		repository_path: PathBuf,
		password_file_path: PathBuf,
		encrypted_file_path: PathBuf,
		include_iv: bool,
	},

	clap_subcommand = {

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

	},

	clap_arguments_parse = |clap_matches| {

		DecryptArguments {

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

	},

	action = |output, arguments| {
		do_decrypt (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
