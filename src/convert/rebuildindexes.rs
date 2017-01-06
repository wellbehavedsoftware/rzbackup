use std::path::PathBuf;
use std::process;

use clap;

use output::Output;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;
use ::zbackup::data::*;
use ::zbackup::proto;

pub fn rebuild_indexes_command (
) -> Box <Command> {

	Box::new (
		RebuildIndexesCommand {},
	)

}

pub struct RebuildIndexesArguments {
	repository_path: PathBuf,
	password_file_path: Option <PathBuf>,
	bundles_per_index: u64,
}

pub struct RebuildIndexesCommand {
}

pub fn rebuild_indexes (
	output: & Output,
	arguments: & RebuildIndexesArguments,
) -> Result <(), String> {

	// open repository

	let repository = (
		Repository::open (
			& output,
			Repository::default_config (),
			& arguments.repository_path,
			arguments.password_file_path.clone ())
	).unwrap_or_else (
		|error| {

		output.message_format (
			format_args! (
				"Error opening repository {}: {}",
				arguments.repository_path.to_string_lossy (),
				error));

		process::exit (1);

	});

	// get list of bundle files

	let bundle_ids =
		scan_bundle_files (
			& arguments.repository_path,
		) ?;

	output.message_format (
		format_args! (
			"Found {} bundle files",
			bundle_ids.len ()));

	// rebuild indexes

	let mut temp_files =
		TempFileManager::new (
			& arguments.repository_path,
		) ?;

	let mut entries_buffer: Vec <IndexEntry> =
		Vec::new ();

	let mut bundle_count: u64 = 0;

	output.status (
		"Rebuilding indexes");

	for & bundle_id in bundle_ids.iter () {

		output.status_progress (
			bundle_count,
			bundle_ids.len () as u64);

		let bundle_path =
			repository.bundle_path (
				bundle_id);

		let bundle_info =
			read_bundle_info (
				bundle_path,
				repository.encryption_key (),
			) ?;

		let mut index_bundle_header =
			proto::IndexBundleHeader::new ();

		index_bundle_header.set_id (
			bundle_id.to_vec ());

		entries_buffer.push (
			(
				index_bundle_header,
				bundle_info,
			)
		);

		// write out a new

		if entries_buffer.len () as u64 == arguments.bundles_per_index {

			flush_index_entries (
				& repository,
				& mut temp_files,
				& mut entries_buffer,
			) ?;

		}

		bundle_count += 1;

	}

	if ! entries_buffer.is_empty () {

		flush_index_entries (
			& repository,
			& mut temp_files,
			& mut entries_buffer,
		) ?;

	}

	output.status_done ();

	// remove old indexes

	let old_index_ids =
		scan_index_files (
			& arguments.repository_path,
		) ?;

	output.message_format (
		format_args! (
			"Removing {} old index files",
			old_index_ids.len ()));

	for old_index_id in old_index_ids {

		temp_files.delete (
			repository.index_path (
				old_index_id));

	}

	// commit changes and return

	output.status (
		"Committing changes ...");

	temp_files.commit () ?;

	output.status_done ();

	Ok (())

}

impl CommandArguments for RebuildIndexesArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		rebuild_indexes (
			output,
			self,
		)

	}

}

impl Command for RebuildIndexesCommand {

	fn name (& self) -> & 'static str {
		"rebuild-indexes"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

		clap::SubCommand::with_name ("rebuild-indexes")
			.about ("Builds a new set of index files by scanning all bundles")

			.arg (
				clap::Arg::with_name ("repository")

				.long ("repository")
				.value_name ("REPOSITORY")
				.required (true)
				.help ("Path to the repository")

			)

			.arg (
				clap::Arg::with_name ("password-file")

				.long ("password-file")
				.value_name ("PASSWORD-FILE")
				.required (false)
				.help ("Path to the password file")

			)

			.arg (
				clap::Arg::with_name ("bundles-per-index")

				.long ("bundles-per-index")
				.value_name ("BUNDLES-PER-INDEX")
				.default_value ("4096")
				.help ("Bundles per index")

			)

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		let arguments = RebuildIndexesArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

			bundles_per_index:
				args::u64_required (
					& clap_matches,
					"bundles-per-index"),

		};

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
