#![ allow (unused_parens) ]

extern crate clap;
extern crate output;
extern crate rand;
extern crate rustc_serialize;
extern crate rzbackup;

use std::process;

use output::Output;

use rzbackup::convert::*;

fn main () {

	let output =
		output::open ();

	let arguments =
		parse_arguments ();

	match arguments {

		Arguments::BalanceBundles (arguments) =>
			balance_bundles_command (
				& output,
				arguments),

		Arguments::BalanceIndexes (arguments) =>
			balance_indexes_command (
				& output,
				arguments),

	}

}

fn balance_bundles_command (
	output: & Output,
	arguments: BalanceBundlesArguments,
) {

	if let Err (error) =
		balance_bundles (
			output,
			& arguments) {

		output.message (
			error);

		process::exit (1);

	}

}

fn balance_indexes_command (
	output: & Output,
	arguments: BalanceIndexesArguments,
) {

	if let Err (error) =
		balance_indexes (
			output,
			& arguments) {

		output.message (
			error);

		process::exit (1);

	}

}

enum Arguments {
	BalanceBundles (BalanceBundlesArguments),
	BalanceIndexes (BalanceIndexesArguments),
}

fn parse_arguments (
) -> Arguments {

	let mut clap_application = (
		clap::App::new ("RZBackup-convert")

		.version (rzbackup::VERSION)
		.author (rzbackup::AUTHOR)
		.about ("Performs various operations on zbackup repostories")

		.subcommand (balance_bundles_subcommand ())
		.subcommand (balance_indexes_subcommand ())

	);

	let clap_matches =
		clap_application.clone ().get_matches ();

	if let Some (clap_matches) =
		clap_matches.subcommand_matches (
			"balance-bundles") {

		Arguments::BalanceBundles (
			balance_bundles_arguments_parse (
				clap_matches))

	} else if let Some (clap_matches) =
		clap_matches.subcommand_matches (
			"balance-indexes") {

		Arguments::BalanceIndexes (
			balance_indexes_arguments_parse (
				clap_matches))

	} else {

		println! ("");

		clap_application.print_help ().unwrap ();

		println! ("");
		println! ("");

		process::exit (0);

	}

}

// ex: noet ts=4 filetype=rust
