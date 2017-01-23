use std::collections::HashSet;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use clap;

use output::Output;

use rand;
use rand::Rng;

use rustc_serialize::hex::ToHex;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;
use ::write::*;
use ::zbackup::data::*;
use ::zbackup::proto;

pub fn balance_bundles (
	output: & Output,
	arguments: & BalanceBundlesArguments,
) -> Result <(), String> {

	let minimum_chunk_count: u64 =
		arguments.chunks_per_bundle * arguments.fill_factor / 100;

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

	loop {

		{

			// begin transaction

			let mut temp_files =
				TempFileManager::new (
					output,
					& arguments.repository_path,
					Some (arguments.sleep_time),
				) ?;

			// get list of index files

			let old_index_ids_and_sizes = (
				scan_index_files_with_sizes (
					& arguments.repository_path)
			) ?;

			output.message_format (
				format_args! (
					"Found {} index files",
					old_index_ids_and_sizes.len ()));

			// read indexes and discard any which are balanced

			let mut unbalanced_indexes: Vec <(IndexId, Vec <IndexEntry>)> =
				Vec::new ();

			let mut new_bundles_total: u64 = 0;

			read_indexes_find_unbalanced (
				output,
				& repository,
				& arguments,
				minimum_chunk_count,
				& old_index_ids_and_sizes,
				& mut unbalanced_indexes,
				& mut new_bundles_total,
			) ?;

			// balance bundles

			if balance_bundles_real (
				output,
				& repository,
				& mut temp_files,
				& arguments,
				minimum_chunk_count,
				& unbalanced_indexes,
				new_bundles_total,
			) ? {
				break;
			}

		}

		// sleep a while

		if arguments.sleep_time != Duration::from_secs (0) {

			output.status_format (
				format_args! (
					"Sleeping ..."));

			thread::sleep (
				arguments.sleep_time);

			output.status_done ();

		}

	}

	Ok (())

}

fn read_indexes_find_unbalanced (
	output: & Output,
	repository: & Repository,
	arguments: & BalanceBundlesArguments,
	minimum_chunk_count: u64,
	old_index_ids_and_sizes: & Vec <(IndexId, u64)>,
	unbalanced_indexes: & mut Vec <(IndexId, Vec <IndexEntry>)>,
	new_bundles_total: & mut u64,
) -> Result <(), String> {

	output.status (
		"Reading indexes ...");

	let total_index_size =
		old_index_ids_and_sizes.iter ().map (
			|& (_, old_index_size)|
			old_index_size
		).sum ();

	let mut seen_bundle_ids: HashSet <BundleId> =
		HashSet::new ();

	let mut read_index_size: u64 = 0;
	let mut unbalanced_chunks_count: u64 = 0;

	for & (
		old_index_id,
		old_index_size,
	) in old_index_ids_and_sizes.iter () {

		let old_index_path =
			repository.index_path (
				old_index_id);

		let old_index_entries =
			read_index (
				& old_index_path,
				repository.encryption_key (),
			) ?;

		for & (
			ref old_index_bundle_index_header,
			ref _old_index_bundle_info,
		) in old_index_entries.iter () {

			let bundle_id =
				to_array_24 (
					old_index_bundle_index_header.get_id ());

			if seen_bundle_ids.contains (
				& bundle_id) {

				return Err (
					format! (
						"Duplicated bundle id in index: {}",
						bundle_id.to_hex ()));

			}

			seen_bundle_ids.insert (
				bundle_id);

		}

		let old_index_unbalanced_chunks_count =
			old_index_entries.iter ().map (
				|& (_, ref bundle_info)|

				bundle_info.get_chunk_record ().len () as u64

			).filter (
				|& chunk_count|

				chunk_count < minimum_chunk_count

			).sum ();

		if old_index_unbalanced_chunks_count > 0 {

			unbalanced_indexes.push (
				(
					old_index_id,
					old_index_entries,
				)
			);

		}

		unbalanced_chunks_count +=
			old_index_unbalanced_chunks_count;

		read_index_size +=
			old_index_size;

		output.status_progress (
			read_index_size,
			total_index_size);

	}

	output.status_done ();

	* new_bundles_total =
		(unbalanced_chunks_count + arguments.chunks_per_bundle - 1)
			/ arguments.chunks_per_bundle;

	output.message_format (
		format_args! (
			"Found {} chunks to balance into {} bundles",
			unbalanced_chunks_count,
			new_bundles_total));

	Ok (())

}

fn balance_bundles_real (
	output: & Output,
	repository: & Repository,
	temp_files: & mut TempFileManager,
	arguments: & BalanceBundlesArguments,
	minimum_chunk_count: u64,
	unbalanced_indexes: & Vec <(IndexId, Vec <IndexEntry>)>,
	new_bundles_total: u64,
) -> Result <bool, String> {

	output.status (
		"Reading bundles");

	let start_time =
		Instant::now ();

	let checkpoint_time =
		start_time + arguments.checkpoint_time;

	let mut new_bundles_count: u64 = 0;

	let mut balanced_chunks: Vec <(ChunkId, Vec <u8>)> =
		Vec::new ();

	let mut new_index_entries: Vec <IndexEntry> =
		Vec::new ();

	for & (
		ref unbalanced_index_id,
		ref unbalanced_index_entries,
	) in unbalanced_indexes {

		let mut unbalanced_index_entries_iter =
			unbalanced_index_entries.iter ();

		while let Some (& (
			ref unbalanced_index_bundle_header,
			ref unbalanced_index_bundle_info,
		)) = unbalanced_index_entries_iter.next () {

			let unbalanced_bundle_id =
				unbalanced_index_bundle_header.get_id ().to_owned ();

			let unbalanced_bundle_id_hex =
				unbalanced_bundle_id.to_hex ();

			if unbalanced_index_bundle_info.get_chunk_record ().len () as u64
				>= minimum_chunk_count {

				// bundle meets fill factor, nothing to do

				new_index_entries.push (
					(
						unbalanced_index_bundle_header.clone (),
						unbalanced_index_bundle_info.clone (),
					)
				);

			} else {

				// bundle does not meet fill factor, rebundle its contents

				let unbalanced_bundle_path =
					repository.path ()
						.join ("bundles")
						.join (& unbalanced_bundle_id_hex [0 .. 2])
						.join (& unbalanced_bundle_id_hex);

				let unbalanced_bundle =
					read_bundle (
						& unbalanced_bundle_path,
						repository.encryption_key ()
					) ?;

				let mut unbalanced_bundle_iter =
					unbalanced_bundle.into_iter ();

				while let Some ((
					unbalanced_chunk_id,
					unbalanced_chunk_data,
				)) = unbalanced_bundle_iter.next () {

					balanced_chunks.push (
						(
							unbalanced_chunk_id,
							unbalanced_chunk_data,
						)
					);

					if balanced_chunks.len () as u64
						== arguments.chunks_per_bundle {

						output.clear_status ();

						flush_bundle (
							output,
							& repository,
							temp_files,
							& mut balanced_chunks,
							& mut new_index_entries,
							new_bundles_count,
							new_bundles_total,
						) ?;

						new_bundles_count += 1;

						// handle checkpoints

						if checkpoint_time < Instant::now () {

							output.clear_status ();

							// write out remaining chunks from this bundle

							while let Some ((
								unbalanced_chunk_id,
								unbalanced_chunk_data,
							)) = unbalanced_bundle_iter.next () {

								balanced_chunks.push (
									(
										unbalanced_chunk_id,
										unbalanced_chunk_data,
									)
								);

							}

							flush_bundle (
								output,
								& repository,
								temp_files,
								& mut balanced_chunks,
								& mut new_index_entries,
								new_bundles_count,
								new_bundles_total,
							) ?;

							temp_files.delete (
								unbalanced_bundle_path);

							// write out remaining entries from this index

							while let Some (& (
								ref unbalanced_index_bundle_header,
								ref unbalanced_index_bundle_info,
							)) = unbalanced_index_entries_iter.next () {

								new_index_entries.push (
									(
										unbalanced_index_bundle_header.clone (),
										unbalanced_index_bundle_info.clone (),
									)
								);

							}

							temp_files.delete (
								repository.index_path (
									* unbalanced_index_id));

							flush_index (
								output,
								& repository,
								temp_files,
								& mut new_index_entries,
							) ?;

							// commit changes and return

							output.message (
								"Performing checkpoint");

							temp_files.commit () ?;

							return Ok (false);

						}

						output.status (
							"Reading bundles");

					}

				}

				temp_files.delete (
					unbalanced_bundle_path);

			}

		}

		temp_files.delete (
			repository.index_path (
				* unbalanced_index_id));

	}

	output.clear_status ();

	// write final bundle and/or index

	flush_bundle (
		output,
		& repository,
		temp_files,
		& mut balanced_chunks,
		& mut new_index_entries,
		new_bundles_count,
		new_bundles_total,
	) ?;

	flush_index (
		output,
		& repository,
		temp_files,
		& mut new_index_entries,
	) ?;

	temp_files.commit () ?;

	Ok (true)

}

fn flush_bundle (
	output: & Output,
	repository: & Repository,
	temp_files: & mut TempFileManager,
	balanced_chunks: & mut Vec <(ChunkId, Vec <u8>)>,
	new_index_entries: & mut Vec <IndexEntry>,
	new_bundles_count: u64,
	new_bundles_total: u64,
) -> Result <(), String> {

	if balanced_chunks.is_empty () {
		return Ok (())
	}

	output.status_format (
		format_args! (
			"Writing bundle {} of {} ...",
			new_bundles_count + 1,
			new_bundles_total));

	let new_bundle_bytes: Vec <u8> =
		rand::thread_rng ()
			.gen_iter::<u8> ()
			.take (24)
			.collect ();

	let new_bundle_name: String =
		new_bundle_bytes.to_hex ();

	let new_bundle_path =
		repository.path ()
			.join ("bundles")
			.join (& new_bundle_name [0 .. 2])
			.join (& new_bundle_name);

	let new_bundle_file =
		Box::new (
			temp_files.create (
				new_bundle_path,
			) ?
		);

	let total_chunks =
		balanced_chunks.len () as u64;

	let new_index_bundle_info =
		write_bundle (
			new_bundle_file,
			repository.encryption_key (),
			& balanced_chunks,
			|chunks_written| {

				output.status_progress (
					chunks_written,
					total_chunks)

			}
		) ?;

	let mut new_index_bundle_header =
		proto::IndexBundleHeader::new ();

	new_index_bundle_header.set_id (
		new_bundle_bytes);

	new_index_entries.push (
		(
			new_index_bundle_header,
			new_index_bundle_info,
		)
	);

	balanced_chunks.clear ();

	output.status_done ();

	Ok (())

}

fn flush_index (
	output: & Output,
	repository: & Repository,
	temp_files: & mut TempFileManager,
	new_index_entries: & mut Vec <IndexEntry>,
) -> Result <(), String> {

	if new_index_entries.is_empty () {
		return Ok (());
	}

	output.status (
		"Writing index ...");

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
			.join (& new_index_name);

	let new_index_file =
		Box::new (
			temp_files.create (
				new_index_path,
			) ?
		);

	write_index (
		new_index_file,
		repository.encryption_key (),
		& new_index_entries,
	) ?;

	new_index_entries.clear ();

	output.status_done ();

	Ok (())

}

command! (

	name = balance_bundles,
	export = balance_bundles_command,

	arguments = BalanceBundlesArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		chunks_per_bundle: u64,
		fill_factor: u64,
		checkpoint_time: Duration,
		sleep_time: Duration,
	},

	clap_subcommand = {

		clap::SubCommand::with_name ("balance-bundles")
			.about ("rewrites bundles so they are a consistent size")

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
				clap::Arg::with_name ("chunks-per-bundle")

				.long ("chunks-per-bundle")
				.value_name ("CHUNKS-PER-BUNDLE")
				.default_value ("256")
				.help ("Chunks per bundle")

			)

			.arg (
				clap::Arg::with_name ("fill-factor")

				.long ("fill-factor")
				.value_name ("FILL-FACTOR")
				.default_value ("25")
				.help ("Minimum fill factor as percentage")

			)

			.arg (
				clap::Arg::with_name ("checkpoint-time")

				.long ("checkpoint-time")
				.value_name ("CHECKPOINT-TIME")
				.default_value ("10 minutes")
				.help ("Time between checkpoints")

			)

			.arg (
				clap::Arg::with_name ("sleep-time")

				.long ("sleep-time")
				.value_name ("SLEEP-TIME")
				.default_value ("10 seconds")
				.help ("Sleep time on every checkpoint")

			)

	},

	clap_arguments_parse = |clap_matches| {

		let arguments = BalanceBundlesArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

			chunks_per_bundle:
				args::u64_required (
					& clap_matches,
					"chunks-per-bundle"),

			fill_factor:
				args::u64_required (
					& clap_matches,
					"fill-factor"),

			checkpoint_time:
				args::duration_required (
					& clap_matches,
					"checkpoint-time"),

			sleep_time:
				args::duration_required (
					& clap_matches,
					"sleep-time"),

		};

		if arguments.fill_factor > 100 {

			args::error_exit (
				format! (
					"Value of --fill-factor must be between 0 and 100"));

		}

		arguments

	},

);

// ex: noet ts=4 filetype=rust
