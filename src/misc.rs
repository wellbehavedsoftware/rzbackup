use protobuf::ProtobufError;

use std::error::Error;
use std::io;

pub fn protobuf_result <Type> (
	result: Result <Type, ProtobufError>,
) -> Result <Type, String> {

	result.map_err (
		|protobuf_error|
		protobuf_error.description ().to_string ()
	)

}

pub fn io_result <Type> (
	result: Result <Type, io::Error>,
) -> Result <Type, String> {

	result.map_err (
		|io_error|
		io_error.description ().to_string ()
	)

}

#[ doc (hidden) ]
#[ macro_export ]
macro_rules! stderr {

	( $ ( $arg : tt ) * ) => (

		match write! (
			&mut ::std::io::stderr () as &mut ::std::io::Write,
			$ ( $arg ) *,
		) {

			Ok (_) => {},

			Err (error) => panic! (
				"Unable to write to stderr: {}",
				error),

		}

	)

}

#[ doc (hidden) ]
#[ macro_export ]
macro_rules! stderrln {

	( $ ( $arg : tt ) * ) => (

		match writeln! (
			&mut ::std::io::stderr () as &mut ::std::io::Write,
			$ ( $arg ) *,
		) {

			Ok (_) => {},

			Err (error) => panic! (
				"Unable to write to stderr: {}",
				error),

		}

	)

}

pub fn to_array (
	slice: & [u8],
) -> [u8; 24] {

	[
		slice [0],  slice [1],  slice [2],  slice [3],  slice [4],  slice [5],
		slice [6],  slice [7],  slice [8],  slice [9],  slice [10], slice [11],
		slice [12], slice [13], slice [14], slice [15], slice [16], slice [17],
		slice [18], slice [19], slice [20], slice [21], slice [22], slice [23],
	]

}

// ex: noet ts=4 filetype=rust
