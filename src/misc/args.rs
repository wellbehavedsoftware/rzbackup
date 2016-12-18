use std::path::PathBuf;

use clap::*;

pub fn path_required (
	matches: & ArgMatches,
	name: & str,
) -> PathBuf {

	PathBuf::from (
		matches.value_of_os (
			name,
		).unwrap ()
	)

}

pub fn bool_flag (
	matches: & ArgMatches,
	name: & str,
) -> bool {

	matches.is_present (
		name,
	)

}

// ex: noet ts=4 filetype=rust
