use std::path::PathBuf;
use std::time::Duration;

use clap;

use num_cpus;

use regex::Regex;

use zbackup::repository::*;

pub fn bool_flag (
	matches: & clap::ArgMatches,
	name: & str,
) -> bool {

	matches.is_present (
		name,
	)

}

pub fn u64_required (
	matches: & clap::ArgMatches,
	name: & str,
) -> u64 {

	matches.value_of (
		name,
	).unwrap ().parse::<u64> ().unwrap_or_else (
		|_|

		error_exit (
			format! (
				"Invalid value for --{}",
				name))

	)

}

pub fn string_required (
	matches: & clap::ArgMatches,
	name: & str,
) -> String {

	String::from (
		matches.value_of (
			name,
		).unwrap ()
	)

}

pub fn string_optional (
	matches: & clap::ArgMatches,
	name: & str,
) -> Option <String> {

	matches.value_of (
		name,
	).map (
		String::from,
	)

}

pub fn path_required (
	matches: & clap::ArgMatches,
	name: & str,
) -> PathBuf {

	PathBuf::from (
		matches.value_of_os (
			name,
		).unwrap ()
	)

}

pub fn path_optional (
	matches: & clap::ArgMatches,
	name: & str,
) -> Option <PathBuf> {

	matches.value_of_os (
		name,
	).map (
		|os_string|

		PathBuf::from (
			os_string)

	)

}

pub fn duration_required (
	matches: & clap::ArgMatches,
	name: & str,
) -> Duration {

	lazy_static! {

		static ref DURATION_REGEX: Regex =
			Regex::new (
				r"^(0|[1-9][0-9]*)\s*(ms|millisecond|milliseconds|s|sec|secs|second|seconds|m|min|mins|minute|minutes|h|hr|hrs|hour|hours|d|day|days)$",
			).unwrap ();

    }

	let string_value =
		string_required (
			matches,
			name,
		);

	if let Some (captures) =
		DURATION_REGEX.captures (
			& string_value,
		) {

		let quantity: u64 =
			captures.get (1).unwrap ().as_str ().parse ().unwrap ();

		let units_str =
			captures.get (2).unwrap ().as_str ();

		match units_str {

			"ms" | "millisecond" | "milliseconds" =>
				Duration::from_millis (
					quantity),

			"s" | "sec" | "secs" | "second" | "seconds" =>
				Duration::from_secs (
					quantity),

			"m" | "min" | "mins" | "minute" | "minutes" =>
				Duration::from_secs (
					quantity * 60),

			"h" | "hr" | "hrs" | "hour" | "hours" =>
				Duration::from_secs (
					quantity * 60 * 60),

			"d" | "day" | "days" =>
				Duration::from_secs (
					quantity * 60 * 60 * 24),

			_ =>
				panic! (
					"Internal error parsing duration: {}",
					string_value),

		}

	} else {

		error_exit (
			format! (
				"Invalid value for --{}",
				name))

	}

}

pub fn error_exit (
	message: String,
) -> ! {

	clap::Error {
		message: message,
		kind: clap::ErrorKind::InvalidValue,
		info: None,
	}.exit ()

}

pub trait ClapSubCommandRzbackupArgs {

	fn repository_config_args (
		self,
	) -> Self;

}

impl <'a, 'b> ClapSubCommandRzbackupArgs
for clap::App <'a, 'b> {

	fn repository_config_args (
		self,
	) -> Self {

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

		self

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


	}

}

pub fn repository_config (
	clap_matches: & clap::ArgMatches,
) -> RepositoryConfig {

	RepositoryConfig {

		max_uncompressed_memory_cache_entries:
			u64_required (
				clap_matches,
				"max-uncompressed-memory-cache-entries",
			) as usize,

		max_compressed_memory_cache_entries:
			u64_required (
				clap_matches,
				"max-compressed-memory-cache-entries",
			) as usize,

		max_compressed_filesystem_cache_entries:
			u64_required (
				clap_matches,
				"max-compressed-filesystem-cache-entries",
			) as usize,

		max_threads:
			u64_required (
				clap_matches,
				"max-threads",
			) as usize,

		filesystem_cache_path:
			path_required (
				clap_matches,
				"filesystem-cache-path",
			).to_string_lossy ().to_string (),

		work_jobs_total: 0, // deprecated and ignored
		work_jobs_batch: 0, // deprecated and ignored

	}

}

// ex: noet ts=4 filetype=rust
