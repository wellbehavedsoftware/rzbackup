use std::error::Error;
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

use clap;

use output::Output;

pub use server::handler::handle_client;

use zbackup::repository::*;
use misc::*;
use misc::args::ClapSubCommandRzbackupArgs;

pub fn run_server (
	output: & Output,
	arguments: & ServerArguments,
) -> Result <bool, String> {

	let repository =
		string_result_with_prefix (
			|| format! (
				"Error opening repository: "),
			Repository::open (
				& output,
				arguments.repository_config.clone (),
				& arguments.repository_path,
				arguments.password_file_path.as_ref (),
			),
		) ?;

	output.message (
		"RZBackup startup complete");

	string_result_with_prefix (
		|| format! (
			"RZBackup server encountered error: "),
		run_server_listener (
			repository.clone (),
			& arguments.listen_address,
		),
	) ?;

	// clean up and return

	repository.close (
		output);

	output.message (
		"RZBackup server terminating normally");

	Ok (true)

}

pub fn run_server_listener (
	repository: Repository,
	bind_address: & str,
) -> Result <(), String> {

	let listener =
		io_result (
			TcpListener::bind (
				bind_address),
		) ?;

	for stream
	in listener.incoming () {

		match stream {

			Ok (stream) => {

				let repository_copy =
					repository.clone ();

				thread::spawn (
					move || {

						handle_client (
							& repository_copy,
							stream)

					}
				);
			},

			Err (error) => {

				println! (
					"Connection failed: {}",
					error.description ());

			},

		}

	};

	Ok (())

}

command! (

	name = server,
	export = server_command,

	arguments = ServerArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		repository_config: RepositoryConfig,
		listen_address: String,
	},

	clap_subcommand = {

		clap::SubCommand::with_name ("server")
			.about ("Server component")

			.arg (
				clap::Arg::with_name ("repository")

				.long ("repository")
				.alias ("repository-path")
				.value_name ("REPOSITORY")
				.required (true)
				.help ("Path to the repository, used to obtain encryption key")

			)

			.arg (
				clap::Arg::with_name ("password-file")

				.long ("password-file")
				.alias ("password-file-path")
				.value_name ("PASSWORD-FILE")
				.required (false)
				.help ("Path to the password file")

			)

			.arg (
				clap::Arg::with_name ("listen-address")

				.long ("listen-address")
				.value_name ("ADDRESS:PORT")
				.default_value ("localhost:4152")
				.help ("Address to listen on, in host:port or ip:port format.")

			)

			.repository_config_args ()

			.arg (
				clap::Arg::with_name ("work-jobs-total")

				.long ("work-jobs-total")
				.value_name ("JOBS")
				.default_value ("0")
				.hidden (true)
				.help ("Deprecated and ignored")

			)

			.arg (
				clap::Arg::with_name ("work-jobs-batch")

				.long ("work-jobs-batch")
				.value_name ("JOBS")
				.default_value ("0")
				.hidden (true)
				.help ("Deprecated and ignored")

			)

	},

	clap_arguments_parse = |clap_matches| {

		ServerArguments {

			repository_path:
				args::path_required (
					clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					clap_matches,
					"password-file"),

			repository_config:
				args::repository_config (
					clap_matches),

			listen_address:
				args::string_required (
					clap_matches,
					"listen-address"),

		}

	},

	action = |output, arguments| {
		run_server (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
