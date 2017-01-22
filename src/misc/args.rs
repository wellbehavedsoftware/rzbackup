use std::path::PathBuf;
use std::time::Duration;

use clap;

use regex::Regex;

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

// ex: noet ts=4 filetype=rust
