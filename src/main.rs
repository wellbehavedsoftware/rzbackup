#![ allow (unused_parens) ]

#[ macro_use ]
extern crate lazy_static;

extern crate clap;
extern crate output;
extern crate rand;
extern crate rustc_serialize;
extern crate rzbackup;

use std::process;

use rzbackup::client::*;
use rzbackup::commands::*;
use rzbackup::convert::*;
use rzbackup::misc::*;
use rzbackup::server::*;

fn main () {

	let output =
		output::open ();

	let commands = vec! [
		client_command (),
		convert_command (),
		decrypt_command (),
		restore_command (),
		server_command (),
	];

	let arguments =
		parse_arguments (
			& commands);

	match arguments.perform (
		& output,
	) {

		Ok (true) =>
			process::exit (0),

		Ok (false) =>
			process::exit (1),

		Err (error) => {

			output.message (
				error);

			process::exit (1);

		}

	}

}

fn parse_arguments (
	commands: & [Box <Command>],
) -> Box <CommandArguments> {

	let mut clap_application =
		commands.iter ().fold (

		clap::App::new ("RZBackup")
			.version (rzbackup::VERSION)
			.author (rzbackup::AUTHOR)
			.about ("Backup tool compatible with ZBackup"),

		|clap_application, command|

		clap_application.subcommand (
			command.clap_subcommand (),
		)

	);

	let clap_matches =
		clap_application.clone ().get_matches ();

	commands.iter ().map (
		|command|

		clap_matches.subcommand_matches (
			command.name (),
		).map (
			|clap_matches|

			command.clap_arguments_parse (
				clap_matches,
			)

		)

	).find (
		|clap_matches|

		clap_matches.is_some ()

	).unwrap_or_else (|| {

		println! ("");

		clap_application.print_help ().unwrap ();

		println! ("");
		println! ("");

		process::exit (0);

	}).unwrap ()

}

// ex: noet ts=4 filetype=rust
