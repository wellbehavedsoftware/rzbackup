use std::process;

use clap;

use output::Output;

pub trait Command : Sync {

	fn name (
		& self,
	) -> & 'static str;

	fn clap_subcommand <'a: 'b, 'b> (
		& 'a self,
	) -> clap::App <'a, 'b>;

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments>;

}

pub trait CommandArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String>;

}

pub struct ParentCommand {
	name: & 'static str,
	description: & 'static str,
	commands: Vec <Box <Command>>,
}

impl ParentCommand {

	pub fn new (
		name: & 'static str,
		description: & 'static str,
		commands: Vec <Box <Command>>,
	) -> ParentCommand {

		ParentCommand {
			name: name,
			description: description,
			commands: commands,
		}

	}

}

impl Command for ParentCommand {

	fn name (& self) -> & 'static str {
		self.name
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& 'a self,
	) -> clap::App <'a, 'b> {

		self.commands.iter ().fold (

			clap::SubCommand::with_name (self.name)
				.version (::VERSION)
				.author (::AUTHOR)
				.about (self.description),

			|clap_application, command|
			clap_application.subcommand (
				command.clap_subcommand (),
			)

		)

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		self.commands.iter ().map (
			|command|

			clap_matches.subcommand_matches (
				command.name (),
			).map (
				|clap_matches|

				command.clap_arguments_parse (
					clap_matches)

			)

		).find (
			|command_arguments|

			command_arguments.is_some ()

		).unwrap_or_else (|| {

			println! ("");

			self.clap_subcommand ().print_help ().unwrap ();

			println! ("");
			println! ("");

			process::exit (0);

		}).unwrap ()

	}

}


// ex: noet ts=4 filetype=rust
