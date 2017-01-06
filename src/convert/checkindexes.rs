use std::collections::HashSet;
use std::path::PathBuf;

use clap;

use output::Output;

use rustc_serialize::hex::ToHex;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::proto;
use ::zbackup::read::*;
use ::zbackup::write::*;

pub fn check_indexes_command (
) -> Box <Command> {

	Box::new (
		CheckIndexesCommand {},
	)

}

pub struct CheckIndexesArguments {
	repository_path: PathBuf,
	password_file_path: Option <PathBuf>,
	repair: bool,
	verbose: bool,
}

pub struct CheckIndexesCommand {
}

pub fn check_indexes (
	output: & Output,
	arguments: & CheckIndexesArguments,
) -> Result <(), String> {

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

	// get list of index files

	let old_index_ids_and_sizes = (
		scan_index_files_with_sizes (
			& arguments.repository_path)
	) ?;

	let old_index_total_size: u64 =
		old_index_ids_and_sizes.iter ().map (
			|& (_, old_index_size)|
			old_index_size as u64
		).sum ();

	output.message_format (
		format_args! (
			"Found {} index files",
			old_index_ids_and_sizes.len ()));

	// get a list of bundle files

	let bundle_ids: HashSet <BundleId> = (
		scan_bundle_files (
			& arguments.repository_path,
		) ?
	).into_iter ().collect ();

	output.message_format (
		format_args! (
			"Found {} bundle files",
			bundle_ids.len ()));

	// check indexes

	let mut temp_files =
		TempFileManager::new (
			& arguments.repository_path,
		) ?;

	let mut checked_index_size: u64 = 0;

	let mut seen_chunk_ids: HashSet <ChunkId> =
		HashSet::new ();

	output.status_format (
		format_args! (
			"{} indexes ...",
			if arguments.repair { "Reparing" } else { "Checking" }));

	let mut missing_chunk_count: u64 = 0;
	let mut duplicated_chunk_count: u64 = 0;

	for (
		old_index_id,
		old_index_size,
	) in old_index_ids_and_sizes {

		output.status_progress (
			checked_index_size,
			old_index_total_size);

		let old_index_path =
			repository.index_path (
				old_index_id);

		let old_index_entries =
			read_index (
				& old_index_path,
				repository.encryption_key (),
			) ?;

		let mut new_index_entries: Vec <IndexEntry> =
			Vec::new ();

		let mut changes = false;

		for & (
			ref old_index_bundle_header,
			ref old_index_bundle_info,
		) in old_index_entries.iter () {

			let old_index_bundle_id =
				to_array_24 (
					old_index_bundle_header.get_id (),
				);

			if ! bundle_ids.contains (
				& old_index_bundle_id) {

				if arguments.verbose {

					output.message_format (
						format_args! (
							"Index {} refers to nonexistant bundle {}",
							old_index_id.to_hex (),
							old_index_bundle_id.to_hex ()));

				}

				missing_chunk_count += 1;
				changes = true;

				continue;

			}

			let mut new_index_bundle_info =
				proto::BundleInfo::new ();

			for old_index_chunk_record
			in old_index_bundle_info.get_chunk_record ().iter () {

				let chunk_id =
					to_array_24 (
						old_index_chunk_record.get_id ());

				if seen_chunk_ids.contains (
					& chunk_id) {

					if arguments.verbose {

						output.message_format (
							format_args! (
								"Index {} contains duplicated chunk {}",
								old_index_id.to_hex (),
								chunk_id.to_hex ()));

					}

					duplicated_chunk_count += 1;
					changes = true;

				} else {

					seen_chunk_ids.insert (
						chunk_id);

					new_index_bundle_info.mut_chunk_record ().push (
						old_index_chunk_record.clone ());

				}

			}

			if ! new_index_bundle_info.get_chunk_record ().is_empty () {

				new_index_entries.push (
					(
						old_index_bundle_header.clone (),
						new_index_bundle_info,
					)
				);

			}

		}

		if changes {

			if new_index_entries.is_empty () {

				if arguments.repair {

					output.message_format (
						format_args! (
							"Removing index {}",
							old_index_id.to_hex ()));

					temp_files.delete (
						old_index_path);

				} else if ! arguments.verbose {

					output.message_format (
						format_args! (
							"Index {} contains no valid entries",
							old_index_id.to_hex ()));

				}

			} else {

				if arguments.repair {

					output.message_format (
						format_args! (
							"Repairing index {}",
							old_index_id.to_hex ()));

					temp_files.delete (
						old_index_path);

					write_index_auto (
						& repository,
						& mut temp_files,
						& new_index_entries,
					) ?;

				} else if ! arguments.verbose {

					output.message_format (
						format_args! (
							"Index {} contains errors",
							old_index_id.to_hex ()));

				}

			}

		}

		checked_index_size +=
			old_index_size;

	}

	output.status_done ();

	if missing_chunk_count + duplicated_chunk_count > 0 {

		if missing_chunk_count > 0 {

			output.message_format (
				format_args! (
					"{} {} missing chunks",
					if arguments.repair { "Removed" } else { "Found" },
					missing_chunk_count));

		}

		if duplicated_chunk_count > 0 {

			output.message_format (
				format_args! (
					"{} {} duplicated chunks",
					if arguments.repair { "Removed" } else { "Found" },
					duplicated_chunk_count));

		}

		if ! arguments.repair {

			output.message (
				"To remove missing/duplicated chunks run again with --repair \
				option");

		}

	}

	output.status (
		"Committing changes ...");

	temp_files.commit () ?;

	output.status_done ();

	Ok (())

}

impl CommandArguments for CheckIndexesArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		check_indexes (
			output,
			self,
		)

	}

}

impl Command for CheckIndexesCommand {

	fn name (& self) -> & 'static str {
		"check-indexes"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

		clap::SubCommand::with_name ("check-indexes")
			.about ("Checks index files for duplicate or missing chunks")

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
				clap::Arg::with_name ("repair")

				.long ("repair")
				.help ("Remove invalid or duplicated index entries")

			)

			.arg (
				clap::Arg::with_name ("verbose")

				.long ("verbose")
				.help ("Show detailed information about errors")

			)

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		let arguments = CheckIndexesArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

			repair:
				args::bool_flag (
					& clap_matches,
					"repair"),

			verbose:
				args::bool_flag (
					& clap_matches,
					"verbose"),

		};

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
