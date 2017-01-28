use std::collections::HashSet;
use std::mem;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::vec;

use clap;

use futures;
use futures::BoxFuture;
use futures::Future;
use futures_cpupool::CpuPool;

use num_cpus;

use output::Output;
use output::OutputJob;

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
) -> Result <bool, String> {

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

	// create cpu pool

	let num_threads =
		(num_cpus::get () - 1) * 5 / 3 + 1;

	let cpu_pool =
		CpuPool::new (
			num_threads);

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

			// do nothing if there is only one small bundle

			if count_unbalanced_bundles (
				minimum_chunk_count,
				arguments.chunks_per_bundle,
				& unbalanced_indexes,
			) < 2 {

				output_message! (
					output,
					"Nothing to do");

				break;

			}

			// balance bundles

			if balance_bundles_real (
				output,
				& cpu_pool,
				num_threads,
				& repository,
				& mut temp_files,
				& arguments,
				minimum_chunk_count,
				unbalanced_indexes,
				new_bundles_total,
			) ? {
				break;
			}

		}

		// sleep a while

		if arguments.sleep_time != Duration::from_secs (0) {

			let output_job =
				output_job_start! (
					output,
					"Sleeping");

			thread::sleep (
				arguments.sleep_time);

			output_job.complete ();

		}

	}

	// clean up and return

	repository.close (
		output);

	Ok (true)

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

	let output_job =
		output_job_start! (
			output,
			"Loading indexes");

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

		output_job.progress (
			read_index_size,
			total_index_size);

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
				|| chunk_count > arguments.chunks_per_bundle

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

	}

	* new_bundles_total =
		(unbalanced_chunks_count + arguments.chunks_per_bundle - 1)
			/ arguments.chunks_per_bundle;

	output_job.complete ();

	output_message! (
		output,
		"Found {} chunks to balance into {} bundles",
		unbalanced_chunks_count,
		new_bundles_total);

	Ok (())

}

fn count_unbalanced_bundles (
	minimum_chunk_count: u64,
	maximum_chunk_count: u64,
	unbalanced_indexes: & [(IndexId, Vec <IndexEntry>)],
) -> u64 {

	let unbalanced_bundle_ids: HashSet <BundleId> =
		unbalanced_indexes.iter ().flat_map (
			|& (ref _index_id, ref index_entries)|

			index_entries.iter ().filter (
				|&& (ref _index_bundle_header, ref bundle_info)| {

				let chunk_count =
					bundle_info.get_chunk_record ().len () as u64;

				chunk_count < minimum_chunk_count
				|| chunk_count > maximum_chunk_count

			}).map (
				|& (ref index_bundle_header, ref _bundle_info)|

				to_array_24 (
					index_bundle_header.get_id ())

			)

		).collect ();

	unbalanced_bundle_ids.len () as u64

}

fn balance_bundles_real (
	output: & Output,
	cpu_pool: & CpuPool,
	max_tasks: usize,
	repository: & Repository,
	temp_files: & mut TempFileManager,
	arguments: & BalanceBundlesArguments,
	minimum_chunk_count: u64,
	unbalanced_indexes: Vec <(IndexId, Vec <IndexEntry>)>,
	new_bundles_total: u64,
) -> Result <bool, String> {

	let output_job =
		output_job_start! (
			output,
			"Balancing bundles");

	let start_time =
		Instant::now ();

	let checkpoint_time =
		start_time + arguments.checkpoint_time;

	let mut new_bundles_count: u64 = 0;

	let mut pending_chunks: Vec <(ChunkId, Vec <u8>)> =
		Vec::new ();

	let mut pending_index_entries: Vec <IndexEntry> =
		Vec::new ();

	let mut index_iterator: vec::IntoIter <(IndexId, Vec <IndexEntry>)> =
		unbalanced_indexes.into_iter ();

	let mut index_entry_iterator: vec::IntoIter <IndexEntry> =
		Vec::new ().into_iter ();

	enum Task {

		ReadBundle {
			output_job: OutputJob,
			chunks: Vec <(ChunkId, Vec <u8>)>,
		},

		WriteBundle {
			output_job: OutputJob,
			index_entry: IndexEntry,
		},

	}

	let mut task_futures: Vec <BoxFuture <Task, String>> =
		Vec::new ();

	output.pause ();

	loop {

		let now =
			Instant::now ();

		// write bundles

		while task_futures.len () < max_tasks
		&& pending_chunks.len () >= arguments.chunks_per_bundle as usize {

			let mut bundle_chunks =
				pending_chunks.split_off (
					arguments.chunks_per_bundle as usize);

			mem::swap (
				& mut bundle_chunks,
				& mut pending_chunks);

			let repository = repository.clone ();
			let temp_files = temp_files.clone ();

			let output_job_write_bundle =
				output_job_start! (
					output,
					"Writing bundle {} of {}",
					new_bundles_count + 1,
					new_bundles_total);

			task_futures.push (
				cpu_pool.spawn_fn (move || {

				flush_bundle (
					& output_job_write_bundle,
					& repository,
					& temp_files,
					& bundle_chunks,
				).map (
					|index_entry|

					Task::WriteBundle {
						output_job: output_job_write_bundle,
						index_entry: index_entry
					}

				)

			}).boxed ());

			new_bundles_count += 1;

		}

		// read bundles

		while task_futures.len () < max_tasks
		&& now < checkpoint_time {

			if let Some ((index_bundle_header, bundle_info)) =
				index_entry_iterator.next () {

				let bundle_chunks =
					bundle_info.get_chunk_record ().len () as u64;

				if bundle_chunks >= minimum_chunk_count
					&& bundle_chunks <= arguments.chunks_per_bundle {

					pending_index_entries.push ((
						index_bundle_header,
						bundle_info,
					));

				} else {

					let bundle_id =
						index_bundle_header.get_id ().to_owned ();

					let bundle_id_hex =
						bundle_id.to_hex ();

					let bundle_path =
						repository.path ()
							.join ("bundles")
							.join (& bundle_id_hex [0 .. 2])
							.join (& bundle_id_hex);

					temp_files.delete (
						bundle_path.clone ());

					let encryption_key =
						repository.encryption_key ();

					let output_job_read_bundle =
						output_job_start! (
							output,
							"Reading bundle {}",
							bundle_id.to_hex ());

					task_futures.push (
						cpu_pool.spawn_fn (move || {

						let bundle_chunks =
							read_bundle (
								& bundle_path,
								encryption_key,
							) ?;

						Ok (Task::ReadBundle {
							output_job: output_job_read_bundle,
							chunks: bundle_chunks,
						})

					}).boxed ())

				}

			} else if let Some ((index_id, index_entries)) =
				index_iterator.next () {

				temp_files.delete (
					repository.index_path (
						index_id));

				index_entry_iterator = index_entries.into_iter ();

			} else {

				break;

			}

		}

		// wait for background tasks

		if task_futures.is_empty () {
			break;
		}

		output.unpause ();

		let (task_value, _index, remaining_tasks) =
			futures::select_all (
				task_futures,
			).wait ().map_err (
				|(error, _index, _remaining_tasks)|

				error

			) ?;

		task_futures = remaining_tasks;

		output.pause ();

		// process background task

		match task_value {

			Task::ReadBundle {
				output_job: output_job_read_bundle,
				chunks: bundle_chunks,
			} => {

				output_job_read_bundle.remove ();

				for bundle_chunk in bundle_chunks {
					pending_chunks.push (
						bundle_chunk);
				}

			},

			Task::WriteBundle {
				output_job: output_job_write_bundle,
				index_entry,
			} => {

				output_job_write_bundle.remove ();

				pending_index_entries.push (
					index_entry);

			},

		}

		// end for checkpoint or no more work

		if task_futures.is_empty ()
		&& checkpoint_time < now {
			break;
		}

	}

	output.unpause ();

	// write final bundle

	if new_bundles_count == new_bundles_total - 1 {

		let output_job_final_bundle =
			output_job_start! (
				output,
				"Writing bundle {} of {}",
				new_bundles_count + 1,
				new_bundles_total);

		pending_index_entries.push (
			flush_bundle (
				& output_job_final_bundle,
				& repository,
				& temp_files,
				& pending_chunks,
			) ?
		);

		pending_chunks.clear ();

		output_job_final_bundle.remove ();

		new_bundles_count += 1;

	}

	output_job_replace! (
		output_job,
		"Balanced {} out of {} bundles",
		new_bundles_count,
		new_bundles_total);

	// perform checkpoint

	if new_bundles_count < new_bundles_total {

		if ! pending_chunks.is_empty () {

			let output_job_checkpoint =
				output_job_start! (
					output,
					"Writing remaining chunks for checkpoint");

			pending_index_entries.push (
				flush_bundle (
					& output_job_checkpoint,
					& repository,
					& temp_files,
					& pending_chunks,
				) ?
			);

			output_job_checkpoint.remove ();

		}

		for index_entry in index_entry_iterator {

			pending_index_entries.push (
				index_entry);

		}

	}

	// write index

	flush_index (
		output,
		& repository,
		temp_files,
		& pending_index_entries,
	) ?;

	// commit changes

	{

		let output_job_commit =
			output_job_start! (
				output,
				"Comitting changes");

		temp_files.commit () ?;

		output_job_commit.remove ();

	}

	// return

	Ok (new_bundles_count == new_bundles_total)

}

fn flush_bundle (
	output_job: & OutputJob,
	repository: & Repository,
	temp_files: & TempFileManager,
	bundle_chunks: & Vec <(ChunkId, Vec <u8>)>,
) -> Result <IndexEntry, String> {

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
		bundle_chunks.len () as u64;

	let new_index_bundle_info =
		write_bundle (
			new_bundle_file,
			repository.encryption_key (),
			& bundle_chunks,
			|chunks_written| {

				output_job.progress (
					chunks_written,
					total_chunks)

			}
		) ?;

	let mut new_index_bundle_header =
		proto::IndexBundleHeader::new ();

	new_index_bundle_header.set_id (
		new_bundle_bytes);

	Ok ((
		new_index_bundle_header,
		new_index_bundle_info,
	))

}

fn flush_index (
	output: & Output,
	repository: & Repository,
	temp_files: & TempFileManager,
	new_index_entries: & Vec <IndexEntry>,
) -> Result <(), String> {

	if new_index_entries.is_empty () {
		return Ok (());
	}

	let output_job =
		output_job_start! (
			output,
			"Writing index");

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

	output_job.remove ();

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

	action = |output, arguments| {
		balance_bundles (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
