use std::path::PathBuf;
use std::process;

use clap;

use output::Output;

use rand;
use rand::Rng;

use rustc_serialize::hex::ToHex;

use ::IndexEntry;
use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;
use ::write::*;

pub fn balance_indexes (
	output: & Output,
	arguments: & BalanceIndexesArguments,
) -> Result <(), String> {

	// open repository

	let repository = match (

		Repository::open (
			& output,
			Repository::default_config (),
			& arguments.repository_path,
			arguments.password_file_path.clone ())

	) {

		Ok (repository) =>
			repository,

		Err (error) => {

			output.message_format (
				format_args! (
					"Error opening repository {}: {}",
					arguments.repository_path.to_string_lossy (),
					error));

			process::exit (1);

		},

	};

	// get list of index files

	let old_indexes = (
		scan_index_files (
			& arguments.repository_path)
	) ?;

	let total_index_size =
		old_indexes.iter ().map (
			|& (_, old_index_size)|
			old_index_size
		).sum ();

	output.message_format (
		format_args! (
			"Found {} index files with total size {}",
			old_indexes.len (),
			total_index_size));

	// balance indexes

	let mut temp_files =
		TempFileManager::new (
			& arguments.repository_path,
		) ?;

	let mut entries_buffer: Vec <IndexEntry> =
		Vec::new ();

	let mut balanced_index_size: u64 = 0;

	output.status (
		"Balancing indexes ...");

	for (old_index_name, old_index_size) in old_indexes {

		let old_index_path =
			arguments.repository_path
				.join ("index")
				.join (old_index_name);

		for old_index_entry in (

			read_index (
				& old_index_path,
				repository.encryption_key ())

		) ? {

			entries_buffer.push (
				old_index_entry);

			if entries_buffer.len () as u64 == arguments.bundles_per_index {

				flush_index_entries (
					& repository,
					& mut temp_files,
					& mut entries_buffer,
				) ?;

			}

		}

		temp_files.delete (
			old_index_path);

		balanced_index_size +=
			old_index_size;

		output.status_progress (
			balanced_index_size,
			total_index_size);

	}

	if ! entries_buffer.is_empty () {

		flush_index_entries (
			& repository,
			& mut temp_files,
			& mut entries_buffer,
		) ?;

	}

	output.status_done ();

	output.status (
		"Committing changes ...");

	temp_files.commit () ?;

	output.status_done ();

	process::exit (0);

}

fn flush_index_entries (
	repository: & Repository,
	temp_files: & mut TempFileManager,
	entries_buffer: & mut Vec <IndexEntry>,
) -> Result <(), String> {

	let new_index_bytes: Vec <u8> =
		rand::thread_rng ()
			.gen_iter::<u8> ()
			.take (24)
			.collect ();

	let new_index_name: String =
		new_index_bytes.to_hex ();

	let new_index_path =
		repository.path ()
			.join ("index")
			.join (new_index_name);

	let new_index_file =
		Box::new (
			temp_files.create (
				new_index_path,
			) ?
		);

	write_index (
		new_index_file,
		repository.encryption_key (),
		& entries_buffer,
	) ?;

	entries_buffer.clear ();

	Ok (())

}

pub struct BalanceIndexesArguments {
	repository_path: PathBuf,
	password_file_path: Option <PathBuf>,
	bundles_per_index: u64,
}

pub fn balance_indexes_subcommand <'a, 'b> (
) -> clap::App <'a, 'b> {

	clap::SubCommand::with_name ("balance-indexes")
		.about ("rewrites index files so they are a consistent size")

		.arg (
			clap::Arg::with_name ("repository")

			.long ("repository")
			.value_name ("REPOSITORY")
			.required (true)
			.help ("Path to the repository, used to obtain encryption key")

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
			.default_value ("65536")
			.help ("Bundles per index")

		)

}

pub fn balance_indexes_arguments_parse (
	clap_matches: & clap::ArgMatches,
) -> BalanceIndexesArguments {

	BalanceIndexesArguments {

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

	}

}

// ex: noet ts=4 filetype=rust
