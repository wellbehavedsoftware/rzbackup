use std::io::stdout;
use std::path::PathBuf;

use clap;

use output::Output;

use ::Repository;
use ::misc::*;

pub fn do_restore (
	output: & Output,
	arguments: & RestoreArguments,
) -> Result <bool, String> {

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

	Ok (true)

}

command! (

	name = restore,
	export = restore_command,

	arguments = RestoreArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		backup_name: String,
	},

	clap_subcommand = {

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

	},

	clap_arguments_parse = |clap_matches| {

		RestoreArguments {

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

		}

	},

	action = |output, arguments| {
		do_restore (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
