use std::io::stdout;
use std::path::PathBuf;

use clap;

use output::Output;

use ::Repository;
use ::misc::*;

pub fn restore_command (
) -> Box <Command> {

	Box::new (
		RestoreCommand {},
	)

}

pub struct RestoreArguments {
	repository_path: PathBuf,
	password_file_path: Option <PathBuf>,
	backup_name: String,
}

pub struct RestoreCommand {
}

pub fn do_restore (
	output: & Output,
	arguments: & RestoreArguments,
) -> Result <(), String> {

	let repository =
		string_result_with_prefix (
			|| format! (
				"Error opening repository: "),
			Repository::open (
				& output,
				Repository::default_config (),
				& arguments.repository_path,
				arguments.password_file_path.as_ref (),
			),
		) ?;

	let stdout_value =
		stdout ();

	let mut stdout_lock =
		stdout_value.lock ();

	string_result_with_prefix (
		|| format! (
			"Error performing restore: "),
		repository.restore (
			& output,
			& arguments.backup_name,
			& mut stdout_lock),
	) ?;

	Ok (())

}

impl CommandArguments for RestoreArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		do_restore (
			output,
			self,
		)

	}

}

impl Command for RestoreCommand {

	fn name (& self) -> & 'static str {
		"restore"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

		clap::SubCommand::with_name ("restore")
			.about ("Restores a backup from a ZBackup repository")

			.arg (
				clap::Arg::with_name ("repository")

				.long ("repository")
				.value_name ("REPOSITORY")
				.required (true)
				.help ("Path to the repository")

			)

			.arg (
				clap::Arg::with_name ("password-file")

				.long ("password-file")
				.value_name ("PASSWORD-FILE")
				.required (false)
				.help ("Path to the password file")

			)

			.arg (
				clap::Arg::with_name ("backup-name")

				.long ("backup-name")
				.value_name ("BACKUP-NAME")
				.required (true)
				.help ("Name of backup to restore")

			)

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		let arguments = RestoreArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

			backup_name:
				args::string_required (
					& clap_matches,
					"backup-name"),

		};

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
