use std::env;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::process;

use output::Output;

pub fn client_command (
) -> Box <Command> {

	Box::new (
		ClientCommand {},
	)

}

pub struct ClientArguments {
}

pub struct ClientCommand {
}

pub fn do_client (
	output: & Output,
	arguments: & ClientArguments,
) -> Result <(), String> {
}

impl CommandArguments for ClientArguments {
}

impl Command for ClientCommand {

	fn name (& self) -> & 'static str {
		"client"
	}

}










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

// ex: noet ts=4 filetype=rust
