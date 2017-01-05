use ::misc::*;

pub fn parse_server_address <
	ServerAddress: AsRef <str>,
> (
	server_address: ServerAddress,
) -> (String, u16) {

	let server_address =
		server_address.as_ref ();

	let server_address_parts: Vec <& str> =
		server_address.split (":").collect ();

	if server_address_parts.len () != 2 {

		args::error_exit (
			format! (
				"Server address must be in 'host:port' format"));

	}

	let server_hostname =
		server_address_parts [0];

	let server_port =
		server_address_parts [1].parse::<u16> (
		).unwrap_or_else (
			|_|

			args::error_exit (
				format! (
					"Error parsing server port"))

		);

	(
		server_hostname.to_string (),
		server_port,
	)

}

// ex: noet ts=4 filetype=rust
