use std::collections::HashSet;
use std::mem;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::vec;

use clap;

use futures::Future;
use futures_cpupool::CpuPool;

use num_cpus;

use output::Output;
use output::OutputJob;

use convert::utils::*;
use misc::*;
use misc::args::ClapSubCommandRzbackupArgs;
use zbackup::data::*;
use zbackup::disk_format::*;
use zbackup::repository::*;
use zbackup::repository_core::*;

pub fn balance_bundles (
	output: & Output,
	arguments: & BalanceBundlesArguments,
) -> Result <bool, String> {

	let minimum_chunk_count: u64 =
		arguments.chunks_per_bundle * arguments.fill_factor / 100;

	let repository_core;
	let backup_chunk_ids;

	if arguments.cluster_backups {

		// open repository

		let repository =
			string_result_with_prefix (
				|| format! (
					"Error opening repository {}: ",
					arguments.repository_path.to_string_lossy ()),
				Repository::open (
					& output,
					arguments.repository_config.clone (),
					& arguments.repository_path,
					arguments.password_file_path.clone ()),
			) ?;

		repository_core =
			repository.core ().clone ();

		// load indexes

		repository.load_indexes (
			output,
		) ?;

		// get list of backup files

		let backup_files =
			scan_backups (
				output,
				& arguments.repository_path,
			) ?;

		// get a list of chunks used by backups

		backup_chunk_ids =
			get_recursive_chunks (
				output,
				& repository,
				& backup_files,
			) ?;

	} else {

		// open repository

		repository_core =
			string_result_with_prefix (
				|| format! (
					"Error opening repository {}: ",
					arguments.repository_path.to_string_lossy ()),
				RepositoryCore::open (
					& output,
					& arguments.repository_path,
					arguments.password_file_path.clone ()),
			) ?;

		backup_chunk_ids =
			HashSet::new ()

	};

	// create cpu pool

	let num_threads =
		(num_cpus::get () - 1) * 5 / 3 + 1;

	let cpu_pool =
		CpuPool::new (
			num_threads);

	loop {

		{

			// begin transaction

			let atomic_file_writer =
				AtomicFileWriter::new (
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

			let mut unbalanced_indexes: Vec <(IndexId, Vec <RawIndexEntry>)> =
				Vec::new ();

			let mut new_bundles_total: u64 = 0;

			read_indexes_find_unbalanced (
				output,
				& repository_core,
				& arguments,
				minimum_chunk_count,
				& old_index_ids_and_sizes,
				& backup_chunk_ids,
				& mut unbalanced_indexes,
				& mut new_bundles_total,
			) ?;

			// do nothing if there is only one unbalanced bundle

			if count_unbalanced_bundles (
				minimum_chunk_count,
				arguments.chunks_per_bundle,
				& backup_chunk_ids,
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
				& repository_core,
				& atomic_file_writer,
				& arguments,
				minimum_chunk_count,
				& backup_chunk_ids,
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

	// return

	Ok (true)

}

fn read_indexes_find_unbalanced (
	output: & Output,
	repository_core: & RepositoryCore,
	arguments: & BalanceBundlesArguments,
	minimum_chunk_count: u64,
	old_index_ids_and_sizes: & Vec <(IndexId, u64)>,
	backup_chunk_ids: & HashSet <ChunkId>,
	unbalanced_indexes: & mut Vec <(IndexId, Vec <RawIndexEntry>)>,
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
			repository_core.index_path (
				old_index_id);

		let old_index_entries =
			index_read_path (
				& old_index_path,
				repository_core.encryption_key (),
			) ?;

		for & RawIndexEntry {
			index_bundle_header: ref old_index_bundle_header,
			..
		} in old_index_entries.iter () {

			if seen_bundle_ids.contains (
				& old_index_bundle_header.bundle_id ()) {

				return Err (
					format! (
						"Duplicated bundle id in index: {}",
						old_index_bundle_header.bundle_id ()));

			}

			seen_bundle_ids.insert (
				old_index_bundle_header.bundle_id (),
			);

		}

		let old_index_unbalanced_chunks_count =
			old_index_entries.iter ().map (
				|& RawIndexEntry {
					index_bundle_header: ref _old_index_bundle_header,
					bundle_info: ref old_index_bundle_info,
				}| {

				let old_index_backup_chunk_ids: HashSet <ChunkId> =
					old_index_bundle_info.chunks ().map (
						|old_index_bundle_info_chunk|

						old_index_bundle_info_chunk.chunk_id ()

					).collect ();

				let old_index_num_backup_chunks =
					old_index_backup_chunk_ids.intersection (
						backup_chunk_ids,
					).count () as u64;

				(
					old_index_num_backup_chunks,
					old_index_bundle_info.num_chunks ()
						- old_index_num_backup_chunks,
				)

			}).filter (
				|& (
					old_index_backup_chunk_count,
					old_index_non_backup_chunk_count,
				)| {

				let old_index_total_chunk_count =
					old_index_backup_chunk_count
						+ old_index_non_backup_chunk_count;

				(

					old_index_backup_chunk_count > 0
					&& old_index_non_backup_chunk_count > 0

				) || (

					old_index_total_chunk_count
						< minimum_chunk_count

					|| old_index_total_chunk_count
						> arguments.chunks_per_bundle

				)

			}).map (
				|(backup_chunk_count, non_backup_chunk_count)|

				backup_chunk_count + non_backup_chunk_count

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
		"Found {} chunks in {} {} to balance into {} bundles",
		unbalanced_chunks_count,
		unbalanced_indexes.len (),
		if unbalanced_indexes.len () == 1 { "index" } else { "indexes" },
		new_bundles_total);

	Ok (())

}

fn count_unbalanced_bundles (
	minimum_chunk_count: u64,
	maximum_chunk_count: u64,
	backup_chunk_ids: & HashSet <ChunkId>,
	unbalanced_indexes: & [(IndexId, Vec <RawIndexEntry>)],
) -> u64 {

	let unbalanced_bundle_ids: HashSet <BundleId> =
		unbalanced_indexes.iter ().flat_map (
			|& (ref _index_id, ref index_entries)|

			index_entries.iter ().filter (
				|&& RawIndexEntry {
					ref bundle_info,
					..
				}| {

				let bundle_backup_chunk_ids: HashSet <ChunkId> =
					bundle_info.chunks ().map (
						|bundle_info_chunk|

						bundle_info_chunk.chunk_id ()

					).collect ();

				let bundle_num_backup_chunks =
					bundle_backup_chunk_ids.intersection (
						backup_chunk_ids,
					).count () as u64;

				let bundle_num_non_backup_chunks =
					bundle_info.num_chunks ()
						- bundle_num_backup_chunks;

				(

					bundle_num_backup_chunks > 0
					&& bundle_num_non_backup_chunks > 0

				) || (

					bundle_info.num_chunks () < minimum_chunk_count
					|| bundle_info.num_chunks () > maximum_chunk_count

				)

			}).map (
				|& RawIndexEntry {
					ref index_bundle_header,
					..
				}|

				index_bundle_header.bundle_id ()

			)

		).collect ();

	unbalanced_bundle_ids.len () as u64

}

fn balance_bundles_real (
	output: & Output,
	cpu_pool: & CpuPool,
	max_tasks: usize,
	repository_core: & RepositoryCore,
	atomic_file_writer: & AtomicFileWriter,
	arguments: & BalanceBundlesArguments,
	minimum_chunk_count: u64,
	backup_chunk_ids: & HashSet <ChunkId>,
	unbalanced_indexes: Vec <(IndexId, Vec <RawIndexEntry>)>,
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

	enum Task {

		ReadBundle {
			output_job: OutputJob,
			chunks: Vec <(ChunkId, Vec <u8>)>,
		},

		WriteBundle {
			output_job: OutputJob,
			index_entry: RawIndexEntry,
		},

	}

	struct State {
		new_bundles_count: u64,
		pending_backup_chunks: Vec <(ChunkId, Vec <u8>)>,
		pending_non_backup_chunks: Vec <(ChunkId, Vec <u8>)>,
		pending_index_entries: Vec <RawIndexEntry>,
		index_iterator: vec::IntoIter <(IndexId, Vec <RawIndexEntry>)>,
		index_entry_iterator: vec::IntoIter <RawIndexEntry>,
	}

	let mut state = State {
		new_bundles_count: 0,
		pending_backup_chunks: Vec::new (),
		pending_non_backup_chunks: Vec::new (),
		pending_index_entries: Vec::new (),
		index_iterator: unbalanced_indexes.into_iter (),
		index_entry_iterator: Vec::new ().into_iter (),
	};

	// concurrent operation

	concurrent_controller (
		output,
		max_tasks,
		& mut state,
		|state| {

			// write bundles

			if (

				state.pending_backup_chunks.len ()
					>= arguments.chunks_per_bundle as usize

				|| state.pending_non_backup_chunks.len ()
					>= arguments.chunks_per_bundle as usize

			) {

				let mut bundle_chunks;

				if state.pending_backup_chunks.len ()
					>= arguments.chunks_per_bundle as usize {

					bundle_chunks =
						state.pending_backup_chunks.split_off (
							arguments.chunks_per_bundle as usize);

					mem::swap (
						& mut bundle_chunks,
						& mut state.pending_backup_chunks);

				} else {

					bundle_chunks =
						state.pending_non_backup_chunks.split_off (
							arguments.chunks_per_bundle as usize);

					mem::swap (
						& mut bundle_chunks,
						& mut state.pending_non_backup_chunks);

				};

				let repository_core = repository_core.clone ();
				let atomic_file_writer = atomic_file_writer.clone ();

				let output_job_write_bundle =
					output_job_start! (
						output,
						"Writing bundle {} of {}",
						state.new_bundles_count + 1,
						new_bundles_total);

				let task = (
					cpu_pool.spawn_fn (move || {

					flush_bundle (
						& output_job_write_bundle,
						& repository_core,
						atomic_file_writer,
						& bundle_chunks,
					).map (
						|index_entry|

						Task::WriteBundle {
							output_job: output_job_write_bundle,
							index_entry: index_entry
						}

					)

				}).boxed ());

				state.new_bundles_count += 1;

				return Some (task);

			}

			// read bundles

			if checkpoint_time <= Instant::now () {
				return None;
			}

			loop {

				if let Some (RawIndexEntry {
					index_bundle_header,
					bundle_info,
				}) = state.index_entry_iterator.next () {

					let bundle_backup_chunk_ids: HashSet <ChunkId> =
						bundle_info.chunks ().map (
							|bundle_info_chunk|

							bundle_info_chunk.chunk_id ()

						).collect ();

					let bundle_num_backup_chunks =
						bundle_backup_chunk_ids.intersection (
							& backup_chunk_ids,
						).count () as u64;

					let bundle_num_non_backup_chunks =
						bundle_info.num_chunks ()
							- bundle_num_backup_chunks;

					if (

						bundle_num_backup_chunks == 0
						|| bundle_num_non_backup_chunks == 0

					) && (

						bundle_info.num_chunks ()
							>= minimum_chunk_count

						&& bundle_info.num_chunks ()
							<= arguments.chunks_per_bundle

					) {

						state.pending_index_entries.push (
							RawIndexEntry {
								index_bundle_header: index_bundle_header,
								bundle_info: bundle_info,
							}
						);

					} else {

						let bundle_path =
							repository_core.bundle_path (
								index_bundle_header.bundle_id ());

						atomic_file_writer.delete (
							bundle_path.clone ());

						let encryption_key =
							repository_core.encryption_key ();

						let output_job_read_bundle =
							output_job_start! (
								output,
								"Reading bundle {}",
								index_bundle_header.bundle_id ());

						return Some (
							cpu_pool.spawn_fn (move || {

							let bundle_chunks =
								bundle_read_path (
									& bundle_path,
									encryption_key,
								) ?;

							Ok (Task::ReadBundle {
								output_job: output_job_read_bundle,
								chunks: bundle_chunks,
							})

						}).boxed ());

					}

				} else if let Some ((index_id, index_entries)) =
					state.index_iterator.next () {

					atomic_file_writer.delete (
						repository_core.index_path (
							index_id));

					state.index_entry_iterator =
						index_entries.into_iter ();

				} else {

					return None;

				}

			}

		},

		|state, task_value| {

			// process background task

			match task_value {

				Task::ReadBundle {
					output_job: output_job_read_bundle,
					chunks: bundle_chunks,
				} => {

					output_job_read_bundle.remove ();

					for (
						bundle_chunk_id,
						bundle_chunk_data,
					) in bundle_chunks {

						if backup_chunk_ids.contains (
							& bundle_chunk_id,
						) {

							state.pending_backup_chunks.push (
								(
									bundle_chunk_id,
									bundle_chunk_data,
								),
							);

						} else {

							state.pending_non_backup_chunks.push (
								(
									bundle_chunk_id,
									bundle_chunk_data,
								),
							);

						}

					}

				},

				Task::WriteBundle {
					output_job: output_job_write_bundle,
					index_entry,
				} => {

					output_job_write_bundle.remove ();

					state.pending_index_entries.push (
						index_entry);

				},

			}

			Ok (())

		},

	) ?;

	output.unpause ();

	// write final bundle

	let mut pending_chunks: Vec <(ChunkId, Vec <u8>)> =
		state.pending_backup_chunks.into_iter ().chain (
			state.pending_non_backup_chunks,
		).collect ();

	if state.new_bundles_count == new_bundles_total - 1 {

		let output_job_final_bundle =
			output_job_start! (
				output,
				"Writing bundle {} of {}",
				state.new_bundles_count + 1,
				new_bundles_total);

		state.pending_index_entries.push (
			flush_bundle (
				& output_job_final_bundle,
				& repository_core,
				atomic_file_writer.clone (),
				& pending_chunks,
			) ?
		);

		pending_chunks.clear ();

		output_job_final_bundle.remove ();

		state.new_bundles_count += 1;

	}

	output_job_replace! (
		output_job,
		"Balanced {} out of {} bundles",
		state.new_bundles_count,
		new_bundles_total);

	// perform checkpoint

	if state.new_bundles_count < new_bundles_total {

		if ! pending_chunks.is_empty () {

			let output_job_checkpoint =
				output_job_start! (
					output,
					"Writing remaining chunks for checkpoint");

			state.pending_index_entries.push (
				flush_bundle (
					& output_job_checkpoint,
					& repository_core,
					atomic_file_writer.clone (),
					& pending_chunks,
				) ?
			);

			output_job_checkpoint.remove ();

		}

		for index_entry in state.index_entry_iterator {

			state.pending_index_entries.push (
				index_entry);

		}

	}

	// write index

	flush_index (
		output,
		& repository_core,
		& atomic_file_writer,
		& state.pending_index_entries,
	) ?;

	// commit changes

	{

		let output_job_commit =
			output_job_start! (
				output,
				"Comitting changes");

		atomic_file_writer.commit () ?;

		output_job_commit.remove ();

	}

	// return

	Ok (state.new_bundles_count == new_bundles_total)

}

fn flush_bundle (
	output_job: & OutputJob,
	repository_core: & RepositoryCore,
	atomic_file_writer: AtomicFileWriter,
	bundle_chunks: & Vec <(ChunkId, Vec <u8>)>,
) -> Result <RawIndexEntry, String> {

	let new_bundle_id =
		BundleId::random ();

	let new_bundle_path =
		repository_core.bundle_path (
			new_bundle_id);

	let mut new_bundle_file =
		atomic_file_writer.create (
			new_bundle_path,
		) ?;

	let total_chunks =
		bundle_chunks.len () as u64;

	let new_index_bundle_info =
		bundle_write_direct (
			& mut new_bundle_file,
			repository_core.encryption_key (),
			& bundle_chunks,
			move |chunks_written| {

				output_job.progress (
					chunks_written,
					total_chunks)

			}
		) ?;

	let new_index_bundle_header =
		DiskIndexBundleHeader::new (
			new_bundle_id);

	Ok (RawIndexEntry {
		index_bundle_header: new_index_bundle_header,
		bundle_info: new_index_bundle_info,
	})

}

fn flush_index (
	output: & Output,
	repository_core: & RepositoryCore,
	atomic_file_writer: & AtomicFileWriter,
	new_index_entries: & Vec <RawIndexEntry>,
) -> Result <(), String> {

	if new_index_entries.is_empty () {
		return Ok (());
	}

	let output_job =
		output_job_start! (
			output,
			"Writing index");

	let new_index_id =
		IndexId::random ();

	let new_index_path =
		repository_core.index_path (
			new_index_id);

	let mut new_index_file =
		atomic_file_writer.create (
			new_index_path,
		) ?;

	index_write_direct (
		& mut new_index_file,
		repository_core.encryption_key (),
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
		repository_config: RepositoryConfig,
		chunks_per_bundle: u64,
		fill_factor: u64,
		checkpoint_time: Duration,
		sleep_time: Duration,
		cluster_backups: bool,
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

			.arg (
				clap::Arg::with_name ("cluster-backups")

				.long ("cluster-backups")
				.help ("Cluster chunks required to expand backups")

			)

			.repository_config_args ()

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

			repository_config:
				args::repository_config (
					clap_matches),

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

			cluster_backups:
				args::bool_flag (
					& clap_matches,
					"cluster-backups"),

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
