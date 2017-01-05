use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

use clap;

use output::Output;

use ::client::*;
use ::misc::*;

pub fn client_restore_command (
) -> Box <Command> {

	Box::new (
		ClientRestoreCommand {},
	)

}

pub struct ClientRestoreArguments {
	server_hostname: String,
	server_port: u16,
	backup_name: String,
}

pub struct ClientRestoreCommand {
}

pub fn do_client_restore (
	output: & Output,
	arguments: & ClientRestoreArguments,
) -> Result <(), String> {

	let mut stream =
		io_result_with_prefix (
			|| format! (
				"Connection error: "),
			TcpStream::connect (
				(
					arguments.server_hostname.as_str (),
					arguments.server_port,
				),
			),
		) ?;

	io_result_with_prefix (
		|| format! (
			"Communication error: "),
		stream.write_fmt (
			format_args! (
				"restore {}\n",
				arguments.backup_name),
		),
	) ?;

	let mut reader =
		BufReader::new (
			stream);

	let mut response_line =
		String::new ();

	io_result_with_prefix (
		|| format! (
			"Communication error: "),
		reader.read_line (
			& mut response_line,
		),
	) ?;

	if response_line != "OK\n" {

		return Err (
			format! (
				"Server returned error: {}\n",
				response_line.trim ()));

	}

	io_result_with_prefix (
		|| format! (
			"Communication error: "),
		io::copy (
			& mut reader,
			& mut io::stdout (),
		),
	) ?;

	output.message (
		"Restore complete");

	Ok (())

}

impl CommandArguments for ClientRestoreArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		do_client_restore (
			output,
			self,
		)

	}

}

impl Command for ClientRestoreCommand {

	fn name (& self) -> & 'static str {
		"restore"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

		clap::SubCommand::with_name ("restore")
			.about ("Restores a backup from the server")

			.arg (
				clap::Arg::with_name ("server-address")

				.long ("server-address")
				.value_name ("SERVER-ADDRESS")
				.required (true)
				.help ("Server address, in 'host:port' format")

			)

			.arg (
				clap::Arg::with_name ("backup-name")

				.long ("backup-name")
				.value_name ("BACKUP-NAME")
				.required (true)
				.help ("Backup name, in '/path/file' format")

			)

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		let (server_hostname, server_port) =
			parse_server_address (
				args::string_required (
					clap_matches,
					"server-address"),
			);

		let arguments = ClientRestoreArguments {

			server_hostname: server_hostname,
			server_port: server_port,

			backup_name:
				args::string_required (
					& clap_matches,
					"backup-name"),

		};

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
