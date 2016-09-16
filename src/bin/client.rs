#[ macro_use ]
extern crate rzbackup;

use std::env;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::process;

fn main () {

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () == 1 {

		show_help ();

		process::exit (0);

	}

	let command_name =
		arguments [1].to_lowercase ();

	let command_arguments =
		arguments [2 .. ].to_owned ();

	if command_name == "restore" {

		restore_command (
			& command_arguments);

	} else {

		exit_with_error_and_show_help (
			& format! (
				"Command not recognised: {}",
				command_name));

	}

}

fn restore_command (
	arguments: & [String],
) {

	if arguments.len () != 2 {

		exit_with_error_and_show_help (
			"Invalid syntax");

	}

	let server_address_string =
		& arguments [0];

	let backup_filename =
		& arguments [1];

	let server_address_parts: Vec <& str> =
		server_address_string.split (":").collect ();

	if server_address_parts.len () != 2 {

		exit_with_error_and_show_help (
			"Invalid server address");

	}

	let server_hostname =
		server_address_parts [0];

	let server_port =
		match server_address_parts [1].parse::<u16> () {

		Ok (port) =>
			port,

		Err (error) => {

			exit_with_error_and_show_help (
				& format! (
					"Invalid server address: {}",
					error.description ()));

		},

	};

	match do_restore (
		server_hostname,
		server_port,
		& backup_filename,
	) {

		Ok (_) =>
			(),

		Err (error) => {

			exit_with_error (
				& format! (
					"Error performing restore: {}",
					error));

		},

	};

}

fn do_restore (
	server_hostname: & str,
	server_port: u16,
	backup_filename: & str,
) -> Result <(), String> {

	let mut stream =
		try! (

		TcpStream::connect (
			(server_hostname, server_port),
		).map_err (
			|error|

			format! (
				"Connection error: {}",
				error.description ())

		)

	);

	try! (

		stream.write_fmt (
			format_args! (
				"restore {}\n",
				backup_filename),
		).map_err (
			|error|

			format! (
				"Communication error: {}",
				error.description ())

		)

	);

	let mut reader =
		BufReader::new (
			stream);

	let mut response_line =
		String::new ();

	try! (
		reader.read_line (
			& mut response_line,
		).map_err (
			|error|

			format! (
				"Communication error: {}",
				error.description ())

		)

	);

	if response_line != "OK\n" {

		return Err (
			format! (
				"Server returned error: {}\n",
				response_line.trim ()));

	}

	try! (

		io::copy (
			& mut reader,
			& mut io::stdout (),
		).map_err (
			|error|

			format! (
				"Communication error: {}",
				error.description ())

		)

	);

	Ok (())

}

fn exit_with_error (
	error_message: & str,
) -> ! {

	stderrln! (
		"{}",
		error_message);

	process::exit (1);

}

fn exit_with_error_and_show_help (
	error_message: & str,
) -> ! {

	stderrln! (
		"");

	stderrln! (
		"{}",
		error_message);

	show_help ();

	process::exit (1);

}

fn show_help () {

	stderrln! (
		"");

	stderrln! (
		"Syntax:");

	stderrln! (
		"");

	stderrln! (
		"  {} restore SERVER:PORT PATH",
		env::args ().next ().unwrap ());

	stderrln! (
		"");

}

// ex: noet ts=4 filetype=rust
