#![ allow (unused_parens) ]

extern crate num_cpus;
extern crate rzbackup;

#[ macro_use ]
extern crate clap;

use clap::App;
use clap::Arg;
use clap::ArgMatches;

use std::process;

use rzbackup::Repository;
use rzbackup::RepositoryConfig;

struct DefaultArgumentStringValues {
	max_uncompressed_memory_cache_entries: String,
	max_compressed_memory_cache_entries: String,
	max_compressed_filesystem_cache_entries: String,
	max_threads: String,
	filesystem_cache_path: String,
}

fn build_default_argument_string_values (
) -> DefaultArgumentStringValues {

	DefaultArgumentStringValues {

		max_uncompressed_memory_cache_entries:
			rzbackup::MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES.to_string (),

		max_compressed_memory_cache_entries:
			rzbackup::MAX_COMPRESSED_MEMORY_CACHE_ENTRIES.to_string (),

		max_compressed_filesystem_cache_entries:
			rzbackup::MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES.to_string (),

		max_threads:
			num_cpus::get ().to_string (),

		filesystem_cache_path:
			rzbackup::FILESYSTEM_CACHE_PATH.to_string (),

	}

}

fn main () {

	process::exit (
		main_real (),
	);

}

fn main_real (
) -> i32 {

	let default_argument_string_values =
		build_default_argument_string_values ();

	let clap_app =
		build_clap_app (
			& default_argument_string_values);

	let argument_matches =
		clap_app.get_matches ();

	let listen_address =
		argument_matches.value_of (
			"listen-address",
		).unwrap ();

	let repository_path =
		argument_matches.value_of (
			"repository-path",
		).unwrap ();

	let password_file_path =
		argument_matches.value_of (
			"password-file");

	let repository_config =
		build_repository_config (
			& argument_matches);

	let repository = match (

		Repository::open (
			repository_config,
			repository_path,
			password_file_path,
		)

	) {

		Ok (repository) =>
			repository,

		Err (error) => {

			println! (
				"Error opening repository: {}",
				error);

			return 1;

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

			return 1;

		},

	};

	println! (
		"RZBackup server terminating normally");

	0

}

fn build_repository_config (
	argument_matches: & ArgMatches,
) -> RepositoryConfig {

	RepositoryConfig {

		max_uncompressed_memory_cache_entries:
			value_t! (
				argument_matches,
				"max-uncompressed-memory-cache-entries",
				usize
			).unwrap_or_else (
				|error|
				error.exit ()
			),

		max_compressed_memory_cache_entries:
			value_t! (
				argument_matches,
				"max-compressed-memory-cache-entries",
				usize
			).unwrap_or_else (
				|error|
				error.exit ()
			),

		max_compressed_filesystem_cache_entries:
			value_t! (
				argument_matches,
				"max-compressed-filesystem-cache-entries",
				usize
			).unwrap_or_else (
				|error|
				error.exit ()
			),

		max_threads:
			value_t! (
				argument_matches,
				"max-threads",
				usize
			).unwrap_or_else (
				|error|
				error.exit ()
			),

		filesystem_cache_path:
			argument_matches.value_of (
				"filesystem-cache-path",
			).unwrap ().to_owned (),

	}

}

fn build_clap_app <'a, 'b> (
	defaults: & 'a DefaultArgumentStringValues,
) -> App <'a, 'b> {

	App::new (
		"rzbackup-server")

	.version (
		rzbackup::VERSION)

	.author (
		"James Pharaoh <james@wellbehavedsoftware.com>")

	.about (
		"Server component of partial ZBackup clone")

	.arg (
		Arg::with_name (
			"repository-path",
		).long (
			"repository-path",
		).value_name (
			"REPOSITORY",
		).help (
			"Path of the ZBackup repository to serve backups from.",
		).required (
			true,
		).takes_value (
			true,
		)
	)

	.arg (
		Arg::with_name (
			"password-file",
		).long (
			"password-file",
		).value_name (
			"PASSWORD-FILE",
		).help (
			"Path to the file containing encryption password, must be \
			present to access an encrypted repository and absent if the \
			repository is not encrypted.",
		).required (
			false,
		).takes_value (
			true,
		)
	)

	.arg (
		Arg::with_name (
			"listen-address",
		).long (
			"listen-address",
		).value_name (
			"ADDRESS:PORT",
		).help (
			"Address to listen on, in host:port or ip:port format.",
		).required (
			false,
		).default_value (
			"localhost:4152",
		)
	)

	.arg (
		Arg::with_name (
			"max-uncompressed-memory-cache-entries",
		).long (
			"max-uncompressed-memory-cache-entries",
		).value_name (
			"ENTRIES",
		).help (
			"Size of high-speed, fairly-high-cost, in memory cache of \
			compressed cache entries."
		).required (
			false,
		).default_value (
			& defaults.max_uncompressed_memory_cache_entries,
		)
	)

	.arg (
		Arg::with_name (
			"max-compressed-memory-cache-entries",
		).long (
			"max-compressed-memory-cache-entries",
		).value_name (
			"ENTRIES",
		).help (
			"Size of high-speed, fairly-high-cost, in memory cache of \
			compressed cache entries."
		).required (
			false,
		).default_value (
			& defaults.max_compressed_memory_cache_entries,
		)
	)

	.arg (
		Arg::with_name (
			"max-compressed-filesystem-cache-entries",
		).long (
			"max-compressed-filesystem-cache-entries",
		).value_name (
			"ENTRIES",
		).help (
			"Size of lower-speed, relatively low-cost, on disk compressed \
			cache entries."
		).required (
			false,
		).default_value (
			& defaults.max_compressed_filesystem_cache_entries,
		)
	)

	.arg (
		Arg::with_name (
			"max-threads",
		).long (
			"max-threads",
		).value_name (
			"THREADS",
		).help (
			"Number of worker threads to execute. The default value is \
			determined by the number of CPU threads reported by the operating \
			system.",
		).required (
			false,
		).default_value (
			& defaults.max_threads,
		)
	)

	.arg (
		Arg::with_name (
			"filesystem-cache-path",
		).long (
			"filesystem-cache-path",
		).value_name (
			"PATH",
		).help (
			"Location of the filesystem cache. This will be used to store \
			chunks which have been decompressed, typically from slow but \
			efficient LZMA compressed bundles, and recompressed using LZO, \
			then saved to disk and managed individually using an LRU cache \
			algorithm."
		).required (
			false,
		).default_value (
			& defaults.filesystem_cache_path,
		)
	)

}

// ex: noet ts=4 filetype=rust
