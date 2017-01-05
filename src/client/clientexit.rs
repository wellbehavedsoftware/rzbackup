use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

use clap;

use output::Output;

use ::client::*;
use ::misc::*;

pub fn client_exit_command (
) -> Box <Command> {

	Box::new (
		ClientExitCommand {},
	)

}

pub struct ClientExitArguments {
	server_hostname: String,
	server_port: u16,
}

pub struct ClientExitCommand {
}

pub fn do_client_exit (
	output: & Output,
	arguments: & ClientExitArguments,
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
				"reindex\n"),
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

	output.message (
		"Server exit requested");

	Ok (())

}

impl CommandArguments for ClientExitArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		do_client_exit (
			output,
			self,
		)

	}

}

impl Command for ClientExitCommand {

	fn name (& self) -> & 'static str {
		"exit"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

		clap::SubCommand::with_name ("exit")
			.about ("Instructs the server to exit")

			.arg (
				clap::Arg::with_name ("server-address")

				.index (1)
				.value_name ("SERVER-ADDRESS")
				.required (true)
				.help ("Server address, in 'host:port' format")

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

		let arguments = ClientExitArguments {

			server_hostname: server_hostname,
			server_port: server_port,

		};

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
