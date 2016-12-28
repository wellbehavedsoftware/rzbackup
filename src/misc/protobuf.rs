use protobuf::ProtobufError;

use std::error::Error;

pub fn protobuf_result <Type> (
	result: Result <Type, ProtobufError>,
) -> Result <Type, String> {

	result.map_err (
		|protobuf_error|
		protobuf_error.description ().to_string ()
	)

}

pub fn protobuf_result_with_prefix <
	PrefixFunction: FnOnce () -> String,
	Type,
> (
	prefix_function: PrefixFunction,
	result: Result <Type, ProtobufError>,
) -> Result <Type, String> {

	result.map_err (
		|protobuf_error|

		format! (
			"{}{}",
			prefix_function (),
			protobuf_error.description ())

	)

}

// ex: noet ts=4 filetype=rust
