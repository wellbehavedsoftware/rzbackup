use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;

use output;
use output::Output;

use ::zbackup::repository::*;
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

		io_result (
			reader.read_line (
				& mut line),
		) ?;

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

			handle_reindex (
				& output,
				repository,
				& stream,
			) ?;

		} else if command == "restore" {

			handle_restore (
				& output,
				repository,
				& stream,
				rest,
			) ?;

			return Ok (());

		} else if command == "status" {

			handle_status (
				& output,
				repository,
				& stream,
			) ?;

			return Ok (());

		} else {

			handle_command_not_recognised (
				& stream,
				command,
			) ?;

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

	repository.reload_indexes (
		output,
	).map_err (
		|error|

		format! (
			"Error during reindex: {}",
			error)

	) ?;

	io_result (
		writer.write_fmt (
			format_args! (
				"OK\n")),
	) ?;

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

	io_result (
		writer.write_fmt (
			format_args! (
				"OK\n")),
	) ?;

	repository.restore (
		output,
		path,
		& mut writer,
	) ?;

	Ok (())

}

fn handle_status (
	output: & Output,
	repository: & Repository,
	stream: & TcpStream,
) -> Result <(), String> {

	output.message_format (
		format_args! (
			"Will return status"));

	let mut writer =
		BufWriter::new (
			stream);

	let status =
		repository.status ();

	io_result (write! (
		writer,
		"OK\n",
	)) ?;

	io_result (write! (
		writer,
		"\n",
	)) ?;

	io_result (write! (
		writer,
		"storage-manager:\n",
	)) ?;

	io_result (write! (
		writer,
		"\n",
	)) ?;

	io_result (write! (
		writer,
		"  uncompressed-memory-items: {}\n",
		status.storage_manager.uncompressed_memory_items,
	)) ?;

	io_result (write! (
		writer,
		"  compressed-memory-items: {}\n",
		status.storage_manager.compressed_memory_items,
	)) ?;

	io_result (write! (
		writer,
		"  live-filesystem-items: {}\n",
		status.storage_manager.live_filesystem_items,
	)) ?;

	io_result (write! (
		writer,
		"  dead-filesystem-items: {}\n",
		status.storage_manager.dead_filesystem_items,
	)) ?;

	io_result (write! (
		writer,
		"\n",
	)) ?;

	io_result (write! (
		writer,
		"  uncompressed-memory-hits: {}\n",
		status.storage_manager.uncompressed_memory_hits,
	)) ?;

	io_result (write! (
		writer,
		"  compressed-memory-hits: {}\n",
		status.storage_manager.compressed_memory_hits,
	)) ?;

	io_result (write! (
		writer,
		"  live-filesystem-hits: {}\n",
		status.storage_manager.live_filesystem_hits,
	)) ?;

	io_result (write! (
		writer,
		"  dead-filesystem-hits: {}\n",
		status.storage_manager.dead_filesystem_hits,
	)) ?;

	io_result (write! (
		writer,
		"  misses: {}\n",
		status.storage_manager.misses,
	)) ?;

	io_result (write! (
		writer,
		"\n",
	)) ?;

	io_result (write! (
		writer,
		"bundle-loader:\n",
	)) ?;

	io_result (write! (
		writer,
		"\n",
	)) ?;

	io_result (write! (
		writer,
		"  num-loads: {}\n",
		status.bundle_loader.num_loads,
	)) ?;

	io_result (write! (
		writer,
		"\n",
	)) ?;

	if status.bundle_loader.loading_later.is_empty () {

		io_result (write! (
			writer,
			"  loading-now: []\n",
		)) ?;

	} else {

		io_result (write! (
			writer,
			"  loading-now:\n",
		)) ?;

		io_result (write! (
			writer,
			"\n",
		)) ?;

		for bundle_id
		in status.bundle_loader.loading_now.iter () {

			io_result (write! (
				writer,
				"    - \"{}\"\n",
				bundle_id,
			)) ?;

		}

	}

	io_result (write! (
		writer,
		"\n",
	)) ?;

	if status.bundle_loader.loading_later.is_empty () {

		io_result (write! (
			writer,
			"  loading-later: []\n",
		)) ?;

	} else {

		io_result (write! (
			writer,
			"  loading-later:\n",
		)) ?;

		io_result (write! (
			writer,
			"\n",
		)) ?;

		for bundle_id
		in status.bundle_loader.loading_later.iter () {

			io_result (write! (
				writer,
				"    - \"{}\"\n",
				bundle_id,
			)) ?;

		}

	}

	io_result (write! (
		writer,
		"\n",
	)) ?;

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

	io_result (
		writer.write_fmt (
			format_args! (
				"ERROR Command not recognised: {}\n",
				command_name)),
	) ?;

	Ok (())

}

// ex: noet ts=4 filetype=rust

