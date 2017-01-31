use protobuf;
use protobuf::CodedInputStream;
use protobuf::CodedOutputStream;

use misc::*;

pub fn protobuf_message_read <
	Type: protobuf::MessageStatic,
	NameFunction: Fn () -> String,
> (
	coded_input_stream: & mut CodedInputStream,
	name_function: NameFunction,
) -> Result <Type, String> {

	let message_length =
		protobuf_result_with_prefix (
			|| format! (
				"Error reading {} length: ",
				name_function ()),
			coded_input_stream.read_raw_varint32 (),
		) ?;

	let old_limit =
		protobuf_result_with_prefix (
			|| format! (
				"Error preparing to read {}: ",
				name_function ()),
			coded_input_stream.push_limit (
				message_length as u64),
		) ?;

	let message =
		protobuf_result_with_prefix (
			|| format! (
				"Error reading {}: ",
				name_function ()),
			protobuf::core::parse_from::<Type> (
				coded_input_stream),
		) ?;

	coded_input_stream.pop_limit (
		old_limit);

	Ok (message)

}

#[ inline ]
pub fn protobuf_message_write <
	NameFunction: Fn () -> String,
	Type: protobuf::MessageStatic,
> (
	name_function: NameFunction,
	coded_output_stream: & mut CodedOutputStream,
	message: & Type,
) -> Result <(), String> {

	// write size

	protobuf_result_with_prefix (
		|| format! (
			"Error writing {} size",
			name_function ()),
		coded_output_stream.write_raw_varint32 (
			message.compute_size ()),
	) ?;

	// write message

	protobuf_result_with_prefix (
		|| format! (
			"Error writing {}",
			name_function ()),
		message.write_to_with_cached_sizes (
			coded_output_stream),
	) ?;

	// return

	Ok (())

}

// ex: noet ts=4 filetype=rust
