use std::error::Error;
use std::io;

pub fn io_result <Type> (
	result: Result <Type, io::Error>,
) -> Result <Type, String> {

	result.map_err (
		|io_error|

		io_error.description ().to_string ()

	)

}

pub fn io_result_with_prefix <
	Prefix: Into <String>,
	Type,
> (
	prefix: Prefix,
	result: Result <Type, io::Error>,
) -> Result <Type, String> {

	result.map_err (
		|io_error|

		format! (
			"{}{}",
			prefix.into (),
			io_error.description ())

	)

}

// ex: noet ts=4 filetype=rust
