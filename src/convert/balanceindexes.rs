use std::mem;
use std::path::PathBuf;

use clap;

use output::Output;

use ::RawIndexEntry;
use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;

pub fn balance_indexes (
	output: & Output,
	arguments: & BalanceIndexesArguments,
) -> Result <bool, String> {

	// open repository

	let repository =
		string_result_with_prefix (
			|| format! (
				"Error opening repository {}: ",
				arguments.repository_path.to_string_lossy ()),
			Repository::open (
				& output,
				Repository::default_config (),
				& arguments.repository_path,
				arguments.password_file_path.clone ()),
		) ?;

	// begin transaction

	let mut temp_files =
		TempFileManager::new (
			output,
			& arguments.repository_path,
			None,
		) ?;

	// get list of index files

	let old_index_ids_and_sizes = (
		scan_index_files_with_sizes (
			& arguments.repository_path)
	) ?;

	let total_index_size =
		old_index_ids_and_sizes.iter ().map (
			|& (_, old_index_size)|
			old_index_size
		).sum ();

	output.message_format (
		format_args! (
			"Found {} index files with total size {}",
			old_index_ids_and_sizes.len (),
			total_index_size));

	// balance indexes

	let mut entries_buffer: Vec <RawIndexEntry> =
		Vec::new ();

	let mut balanced_index_size: u64 = 0;

	let output_job =
		output_job_start! (
			output,
			"Balancing indexes");

	for (
		old_index_id,
		old_index_size,
	) in old_index_ids_and_sizes {

		let old_index_path =
			repository.index_path (
				old_index_id);

		for old_index_entry in (

			read_index (
				& old_index_path,
				repository.encryption_key ())

		) ? {

			entries_buffer.push (
				old_index_entry);

			if entries_buffer.len () as u64 == arguments.bundles_per_index {

				let index_entries =
					mem::replace (
						& mut entries_buffer,
						Vec::new ());

				flush_index_entries (
					output,
					& repository,
					& temp_files,
					& index_entries,
				) ?;

			}

		}

		temp_files.delete (
			old_index_path);

		balanced_index_size +=
			old_index_size;

		output_job.progress (
			balanced_index_size,
			total_index_size);

	}

	if ! entries_buffer.is_empty () {

		flush_index_entries (
			output,
			& repository,
			& mut temp_files,
			& mut entries_buffer,
		) ?;

	}

	output_job.complete ();

	// write changes to disk

	let output_job =
		output_job_start! (
			output,
			"Committing changes");

	temp_files.commit () ?;

	output_job.complete ();

	// clean up and return

	repository.close (
		output);

	Ok (true)

}

command! (

	name = balance_indexes,
	export = balance_indexes_command,

	arguments = BalanceIndexesArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		bundles_per_index: u64,
	},

	clap_subcommand = {

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
				.default_value ("16384")
				.help ("Bundles per index")

			)

	},

	clap_arguments_parse = |clap_matches| {

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

	},

	action = |output, arguments| {
		balance_indexes (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
