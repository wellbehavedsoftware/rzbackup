use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process;

use clap;

use output::Output;

use rustc_serialize::hex::FromHex;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;
use ::write::*;
use ::zbackup::data::*;

pub struct GcBundlesArguments {
	repository_path: PathBuf,
	password_file_path: Option <PathBuf>,
}

pub fn gc_bundles (
	output: & Output,
	arguments: & GcBundlesArguments,
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

	let old_bundles = (
		scan_bundle_files (
			& arguments.repository_path)
	) ?;

	output.message_format (
		format_args! (
			"Found {} bundle files",
			old_bundles.len ()));

	// get list of index files

	let index_files = (
		scan_index_files (
			& arguments.repository_path)
	) ?;

	output.message_format (
		format_args! (
			"Found {} index files",
			index_files.len ()));

	// read indexes

	output.status (
		"Reading indexes ...");

	let mut index_entries: HashSet <(BundleId, ChunkId)> =
		HashSet::new ();

	let total_index_size: u64 =
		index_files.iter ().map (
			|& (_, index_size)|
			index_size
		).sum ();

	let mut read_index_size: u64 = 0;

	for & (
		ref index_file_name,
		ref index_file_size,
	) in index_files.iter () {

		output.status_progress (
			read_index_size,
			total_index_size);

		let index_file_path =
			arguments.repository_path
				.join ("index")
				.join (& index_file_name);

		let index_file_entries = (
			read_index (
				& index_file_path,
				repository.encryption_key ())
		) ?;

		for & (
			ref bundle_index_header,
			ref bundle_info,
		) in index_file_entries.iter () {

			for chunk_record
			in bundle_info.get_chunk_record ().iter () {

				index_entries.insert (
					(
						to_array_24 (
							bundle_index_header.get_id ()),
						to_array_24 (
							chunk_record.get_id ()),
					)
				);

			}

		}

		read_index_size +=
			* index_file_size as u64;

	}

	output.status_done ();

	// read bundle headers

	output.status (
		"Reading bundle metadata ...");

	let mut bundles_to_compact: Vec <String> =
		Vec::new ();

	let mut bundles_to_delete: Vec <String> =
		Vec::new ();

	let mut old_bundles_count: u64 = 0;
	let old_bundles_total = old_bundles.len () as u64;

	for old_bundle_name in old_bundles {

		output.status_progress (
			old_bundles_count,
			old_bundles_total);

		let old_bundle_id: BundleId =
			to_array_24 (
				& old_bundle_name.from_hex ().unwrap ());

		let old_bundle_path =
			arguments.repository_path
				.join ("bundles")
				.join (& old_bundle_name [0 .. 2])
				.join (& old_bundle_name);

		let old_bundle_info =
			read_bundle_info (
				old_bundle_path,
				repository.encryption_key (),
			) ?;

		let mut num_to_keep: u64 = 0;
		let mut num_to_reap: u64 = 0;

		for chunk_record
		in old_bundle_info.get_chunk_record () {

			if index_entries.contains (
				& (
					old_bundle_id,
					to_array_24 (
						chunk_record.get_id ()),
				)
			) {

				num_to_keep += 1;

			} else {

				num_to_reap += 1;

			}

		}

		if num_to_keep == 0 {

			bundles_to_delete.push (
				old_bundle_name);

		} else if num_to_reap > 0 {

			bundles_to_compact.push (
				old_bundle_name);

		}

		old_bundles_count += 1;

	}

	output.status_done ();

	output.message_format (
		format_args! (
			"Found {} bundles to compact and {} to delete",
			bundles_to_compact.len (),
			bundles_to_delete.len ()));

	// delete bundles

	if ! bundles_to_delete.is_empty () {

		output.status (
			"Deleting bundles ...");

		let bundles_to_delete_total = bundles_to_delete.len () as u64;
		let mut bundles_to_delete_count: u64 = 0;

		for bundle_to_delete in bundles_to_delete {

			output.status_progress (
				bundles_to_delete_count,
				bundles_to_delete_total);

			io_result (
				fs::remove_file (
					repository.path ()
						.join ("bundles")
						.join (& bundle_to_delete [0 .. 2])
						.join (& bundle_to_delete)),
			) ?;

			bundles_to_delete_count += 1;

		}

		output.status_done ();

	}

	// garbage collect bundles

	let mut temp_files =
		TempFileManager::new (
			& arguments.repository_path,
		) ?;

	let bundles_to_compact_total = bundles_to_compact.len () as u64;
	let bundles_to_compact_count: u64 = 0;

	for bundle_to_compact in bundles_to_compact {

		let bundle_id: BundleId =
			to_array_24 (
				& bundle_to_compact.from_hex ().unwrap ());

		let bundle_path =
			repository.path ()
				.join ("bundles")
				.join (& bundle_to_compact [0 .. 2])
				.join (& bundle_to_compact);

		output.status_format (
			format_args! (
				"Reading bundle {} of {} ...",
				bundles_to_compact_count + 1,
				bundles_to_compact_total));

		let uncompacted_bundle =
			read_bundle (
				& bundle_path,
				repository.encryption_key ()
			) ?;

		output.status_format (
			format_args! (
				"Compacting bundle {} of {} ...",
				bundles_to_compact_count + 1,
				bundles_to_compact_total));

		let compacted_bundle_file =
			Box::new (
				temp_files.create (
					bundle_path,
				) ?
			);

		let compacted_bundle: Vec <(ChunkId, Vec <u8>)> =
			uncompacted_bundle.into_iter ().filter (
				|& (chunk_id, _)|

				index_entries.contains (
					& (
						bundle_id,
						chunk_id,
					)
				)

			).collect ();

		let total_chunks =
			compacted_bundle.len () as u64;

		write_bundle (
			compacted_bundle_file,
			repository.encryption_key (),
			& compacted_bundle,
			|chunks_written| {

				output.status_progress (
					chunks_written,
					total_chunks)

			}
		) ?;

		temp_files.commit () ?;

		output.status_done ();

		bundles_to_compact_count += 1;

	}

	// done, return

	Ok (())

}

pub fn gc_bundles_subcommand <'a, 'b> (
) -> clap::App <'a, 'b> {

	clap::SubCommand::with_name ("gc-bundles")
		.about ("Removes chunks from bundles which are not in any index")

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

}

pub fn gc_bundles_arguments_parse (
	clap_matches: & clap::ArgMatches,
) -> GcBundlesArguments {

	GcBundlesArguments {

		repository_path:
			args::path_required (
				& clap_matches,
				"repository"),

		password_file_path:
			args::path_optional (
				& clap_matches,
				"password-file"),

	}

}

// ex: noet ts=4 filetype=rust
