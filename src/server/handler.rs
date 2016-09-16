use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use ::Repository;

use ::misc::*;

pub fn handle_client (
	repository: Arc <Mutex <Repository>>,
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
	repository: Arc <Mutex <Repository>>,
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

		let parts: Vec <& str> =
			line.splitn (2, ' ').collect ();

		let command_string =
			parts [0].to_lowercase ();

		let command =
			& command_string;

		let rest =
			if parts.len () > 1 {
				parts [1].trim ()
			} else {
				""
			};

		println! (
			"Command: [{}]",
			command);

		println! (
			"Rest: [{}]",
			rest);

		if command == "restore" {

			try! (
				handle_restore (
					& repository,
					& stream,
					rest));

			return Ok (());

		} else if command == "exit" {

			return Ok (());

		} else {

			println! (
				"Don't recognise command: {}",
				command);

		}

	}

}

fn handle_restore (
	repository_mutex: & Arc <Mutex <Repository>>,
	stream: & TcpStream,
	path: & str,
) -> Result <(), String> {

	println! (
		"Will restore: {}",
		path);

	let mut repository =
		repository_mutex.lock ().unwrap ();

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
			path,
			& mut writer));

	Ok (())

}

// ex: noet ts=4 filetype=rust
