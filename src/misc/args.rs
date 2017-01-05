use std::path::PathBuf;

use clap;

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
