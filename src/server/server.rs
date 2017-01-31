use std::error::Error;
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

use clap;

use num_cpus;

use output::Output;

pub use server::handler::handle_client;

use ::zbackup::repository::*;
use ::misc::*;

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

fn repository_config_parse (
	clap_matches: & clap::ArgMatches,
) -> RepositoryConfig {

	RepositoryConfig {

		max_uncompressed_memory_cache_entries:
			args::u64_required (
				clap_matches,
				"max-uncompressed-memory-cache-entries",
			) as usize,

		max_compressed_memory_cache_entries:
			args::u64_required (
				clap_matches,
				"max-compressed-memory-cache-entries",
			) as usize,

		max_compressed_filesystem_cache_entries:
			args::u64_required (
				clap_matches,
				"max-compressed-filesystem-cache-entries",
			) as usize,

		max_threads:
			args::u64_required (
				clap_matches,
				"max-threads",
			) as usize,

		filesystem_cache_path:
			args::path_required (
				clap_matches,
				"filesystem-cache-path",
			).to_string_lossy ().to_string (),

		work_jobs_total: 0, // deprecated and ignored
		work_jobs_batch: 0, // deprecated and ignored

	}

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

		lazy_static! {

			static ref DEFAULT_MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES_STRING: String =
				::MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES.to_string ();

			static ref DEFAULT_MAX_COMPRESSED_MEMORY_CACHE_ENTRIES_STRING: String =
				::MAX_COMPRESSED_MEMORY_CACHE_ENTRIES.to_string ();

			static ref DEFAULT_MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES_STRING: String =
				::MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES.to_string ();

			static ref DEFAULT_MAX_THREADS_STRING: String =
				num_cpus::get ().to_string ();

			static ref DEFAULT_FILESYSTEM_CACHE_PATH_STRING: String =
				::FILESYSTEM_CACHE_PATH.to_string ();

		}

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

			.arg (
				clap::Arg::with_name ("max-uncompressed-memory-cache-entries")

				.long ("max-uncompressed-memory-cache-entries")
				.value_name ("ENTRIES")
				.default_value (
					& DEFAULT_MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES_STRING)
				.help ("Size of very high speed, very high cost, in-memory \
					cache of uncompressed cache entries.")

			)

			.arg (
				clap::Arg::with_name ("max-compressed-memory-cache-entries")

				.long ("max-compressed-memory-cache-entries")
				.value_name ("ENTRIES")
				.default_value (
					& DEFAULT_MAX_COMPRESSED_MEMORY_CACHE_ENTRIES_STRING)
				.help ("Size of high speed, high cost, in memory cache of \
					compressed cache entries.")

			)

			.arg (

				clap::Arg::with_name ("max-compressed-filesystem-cache-entries")

				.long ("max-compressed-filesystem-cache-entries")
				.value_name ("ENTRIES")
				.default_value (
					& DEFAULT_MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES_STRING)
				.help ("Size of medium speed, low cost, on-disk cache of \
					compressed cache entries.")

			)

			.arg (
				clap::Arg::with_name ("max-threads")

				.long ("max-threads")
				.value_name ("THREADS")
				.default_value (
					& DEFAULT_MAX_THREADS_STRING)
				.help ("Number of worker threads to execute. The default value \
					is determined by the number of CPU threads reported by the \
					operating system.")

			)

			.arg (
				clap::Arg::with_name ("filesystem-cache-path")

				.long ("filesystem-cache-path")
				.value_name ("PATH")
				.default_value (
					& DEFAULT_FILESYSTEM_CACHE_PATH_STRING)
				.help ("Location of the filesystem cache. This will be used to \
					store chunks which have been decompressed, typically from \
					slow but efficient LZMA compressed bundles, and \
					recompressed using LZO, then saved to disk and managed \
					individually using an LRU cache algorithm.")

			)

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
				repository_config_parse (
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
