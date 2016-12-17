use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;

use output;
use output::Output;

use ::Repository;

use ::misc::*;

pub fn handle_client (
	repository: & Repository,
	stream: TcpStream,
) {

	let peer_address =
		stream.peer_addr ().unwrap ();

	println! (
		"Connection from: {}",
		peer_address);

	match handle_client_real (
		repository,
		stream) {

		Ok (_) => {

			println! (
				"Disconnection from: {}",
				peer_address);

		},

		Err (error) => {

			println! (
				"Error from: {}: {}",
				peer_address,
				error);

		},

	}

}

fn handle_client_real (
	repository: & Repository,
	stream: TcpStream,
) -> Result <(), String> {

	let mut reader =
		BufReader::new (
			& stream);

	loop {

		let mut line =
			String::new ();

		try! (
			io_result (
				reader.read_line (
					& mut line)));

		if line.is_empty () {

			println! (
				"Disconnect");

			return Ok (());

		}

		let parts: Vec <& str> =
			line.splitn (2, ' ').collect ();

		let command_string =
			parts [0].trim ().to_lowercase ();

		let command =
			& command_string;

		let rest =
			if parts.len () > 1 {
				parts [1].trim ()
			} else {
				""
			};

		let output =
			output::pipe ();

		if command == "exit" {

			println! (
				"Exiting");

			return Ok (());

		} else if command == "reindex" {

			try! (
				handle_reindex (
					& output,
					repository,
					& stream));

		} else if command == "restore" {

			try! (
				handle_restore (
					& output,
					repository,
					& stream,
					rest));

			return Ok (());

		} else {

			try! (
				handle_command_not_recognised (
					& stream,
					command));

		}

	}

}

fn handle_reindex (
	output: & Output,
	repository: & Repository,
	stream: & TcpStream,
) -> Result <(), String> {

	output.message (
		"Will reindex");

	let mut writer =
		BufWriter::new (
			stream);

	try! (

		repository.reload_indexes (
			output,
		).map_err (
			|error|

			format! (
				"Error during reindex: {}",
				error)

		)

	);

	try! (
		io_result (
			writer.write_fmt (
				format_args! (
					"OK\n"))));

	Ok (())

}

fn handle_restore (
	output: & Output,
	repository: & Repository,
	stream: & TcpStream,
	path: & str,
) -> Result <(), String> {

	output.message_format (
		format_args! (
			"Will restore: {}",
			path));

	let mut writer =
		BufWriter::new (
			stream);

	try! (
		io_result (
			writer.write_fmt (
				format_args! (
					"OK\n"))));

	try! (
		repository.restore (
			output,
			path,
			& mut writer));

	Ok (())

}

fn handle_command_not_recognised (
	stream: & TcpStream,
	command_name: & str,
) -> Result <(), String> {

	println! (
		"Command not recognised: {}",
		command_name);

	let mut writer =
		BufWriter::new (
			stream);

	try! (
		io_result (
			writer.write_fmt (
				format_args! (
					"ERROR Command not recognised: {}\n",
					command_name))));

	Ok (())

}

// ex: noet ts=4 filetype=rust
