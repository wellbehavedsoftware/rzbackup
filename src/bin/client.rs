extern crate output;
extern crate rzbackup;

use std::env;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::process;

use output::Output;

fn main () {

	let output =
		output::open ();

	let arguments: Vec <String> =
		env::args ().collect ();

	if arguments.len () == 1 {

		show_help (
			& output);

		process::exit (0);

	}

	let command_name =
		arguments [1].to_lowercase ();

	let command_arguments =
		arguments [2 .. ].to_owned ();

	if command_name == "reindex" {

		reindex_command (
			& output,
			& command_arguments);

	} else if command_name == "restore" {

		restore_command (
			& output,
			& command_arguments);

	} else {

		exit_with_error_and_show_help (
			& output,
			& format! (
				"Command not recognised: {}",
				command_name));

	}

}

fn reindex_command (
	output: & Output,
	arguments: & [String],
) {

	if arguments.len () != 1 {

		exit_with_error_and_show_help (
			output,
			"Invalid syntax");

	}

	let (server_hostname, server_port) =
		parse_server_address (
			output,
			& arguments [0]);

	match do_reindex (
		output,
		server_hostname,
		server_port,
	) {

		Ok (_) =>
			(),

		Err (error) => {

			exit_with_error (
				output,
				& format! (
					"Error performing reindex: {}",
					error));

		},

	};

}

fn restore_command (
	output: & Output,
	arguments: & [String],
) {

	if arguments.len () != 2 {

		exit_with_error_and_show_help (
			output,
			"Invalid syntax");

	}

	let (server_hostname, server_port) =
		parse_server_address (
			output,
			& arguments [0]);

	let backup_filename =
		& arguments [1];

	match do_restore (
		server_hostname,
		server_port,
		& backup_filename,
	) {

		Ok (_) =>
			(),

		Err (error) => {

			exit_with_error (
				output,
				& format! (
					"Error performing restore: {}",
					error));

		},

	};

}

fn do_reindex (
	_output: & Output,
	server_hostname: & str,
	server_port: u16,
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
				"reindex\n"),
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

	Ok (())

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

fn parse_server_address <'a> (
	output: & Output,
	server_address_string: & 'a str,
) -> (& 'a str, u16) {

	let server_address_parts: Vec <& str> =
		server_address_string.split (":").collect ();

	if server_address_parts.len () != 2 {

		exit_with_error_and_show_help (
			output,
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
				output,
				& format! (
					"Invalid server address: {}",
					error.description ()));

		},

	};

	(server_hostname, server_port)

}

fn exit_with_error (
	output: & Output,
	error_message: & str,
) -> ! {

	output.message (
		error_message);

	process::exit (1);

}

fn exit_with_error_and_show_help (
	output: & Output,
	error_message: & str,
) -> ! {

	output.message (
		"");

	output.message (
		error_message);

	show_help (
		output);

	process::exit (1);

}

fn show_help (
	output: & Output,
) {

	output.message (
		"");

	output.message (
		"Syntax:");

	output.message (
		"");

	output.message_format (
		format_args! (
			"  {} reindex SERVER:PORT",
			env::args ().next ().unwrap ()));

	output.message_format (
		format_args! (
			"  {} restore SERVER:PORT PATH",
			env::args ().next ().unwrap ()));

	output.message (
		"");

}

// ex: noet ts=4 filetype=rust
