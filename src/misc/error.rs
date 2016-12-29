pub fn string_result_with_prefix <
	Type,
	PrefixFunction: FnOnce () -> String,
> (
	prefix_function: PrefixFunction,
	result: Result <Type, String>,
) -> Result <Type, String> {

	result.map_err (
		|string_error|

		format! (
			"{}{}",
			prefix_function (),
			string_error)

	)

}

// ex: noet ts=4 filetype=rust
