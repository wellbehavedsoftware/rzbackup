use std::mem;
use std::path::PathBuf;

use clap;

use futures;
use futures::BoxFuture;
use futures::Future;

use futures_cpupool::CpuPool;

use num_cpus;

use output::Output;

use rustc_serialize::hex::ToHex;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;
use ::zbackup::data::*;
use ::zbackup::proto;

struct IndexRebuilder <'a> {
	arguments: & 'a RebuildIndexesArguments,
	repository: Repository,
	num_threads: usize,
	cpu_pool: CpuPool,
}

impl <'a> IndexRebuilder <'a> {

	fn new (
		output: & Output,
		arguments: & 'a RebuildIndexesArguments,
	) -> Result <IndexRebuilder <'a>, String> {

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

		// create thread pool

		let num_threads =
			num_cpus::get () * 2;

		let cpu_pool =
			CpuPool::new (
				num_threads);

		// return

		Ok (IndexRebuilder {
			arguments: arguments,
			repository: repository,
			num_threads: num_threads,
			cpu_pool: cpu_pool,
		})

	}

	fn rebuild_indexes (
		& mut self,
		output: & Output,
	) -> Result <bool, String> {

		// begin transaction

		let mut temp_files =
			TempFileManager::new (
				output,
				& self.arguments.repository_path,
				None,
			) ?;

		// get list of bundle files

		let bundle_ids =
			scan_bundle_files (
				output,
				& self.arguments.repository_path,
			) ?;

		output.message_format (
			format_args! (
				"Found {} bundle files",
				bundle_ids.len ()));

		// rebuild indexes

		let mut entries_buffer: Vec <IndexEntry> =
			Vec::new ();

		let mut bundle_count: u64 = 0;
		let bundle_total = bundle_ids.len () as u64;

		let output_job =
			output_job_start! (
				output,
				"Rebuilding indexes");

		type TaskFuture =
			BoxFuture <Option <(BundleId, proto::BundleInfo)>, String>;

		let mut task_futures: Vec <TaskFuture> =
			Vec::new ();

		let mut bundle_ids_iter =
			bundle_ids.into_iter ();

		loop {

			// start bundle load tasks

			while task_futures.len () < self.num_threads {

				if let Some (bundle_id) =
					bundle_ids_iter.next () {

					let repository =
						self.repository.clone ();

					let output_clone =
						output.clone ();

					task_futures.push (
						self.cpu_pool.spawn_fn (move || {

							let output_job =
								output_job_start! (
									output_clone,
									"Reading bundle {}",
									bundle_id.to_hex ());

							let bundle_data =
								read_bundle_info (
									repository.bundle_path (
										bundle_id),
									repository.encryption_key (),
								).map (|bundle_info|
									Some ((bundle_id, bundle_info))
								);

							output_job.remove ();

							bundle_data

						}).boxed ()
					);

				} else {
					break;
				}

			}

			// handle next bundle load

			if task_futures.is_empty () {
				break;
			}

			let (task_result, _index, remaining_task_futures) =
				futures::select_all (
					task_futures,
				).wait ().map_err (
					|(error, _index, _remaining_task_futures)|
					error,
				) ?;

			task_futures = remaining_task_futures;

			if let Some ((bundle_id, bundle_info)) = task_result {

				output_job.progress (
					bundle_count,
					bundle_total);

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

				// write out a new index

				if entries_buffer.len () as u64
					== self.arguments.bundles_per_index {

					let output =
						output.clone ();

					let repository =
						self.repository.clone ();

					let temp_files =
						temp_files.clone ();

					let index_entries =
						mem::replace (
							& mut entries_buffer,
							Vec::new ());

					task_futures.push (
						self.cpu_pool.spawn_fn (move || {

							flush_index_entries (
								& output,
								& repository,
								& temp_files,
								& index_entries,
							).map (|_| None )

						}).boxed ()
					);

				}

				bundle_count += 1;

			}

		}

		// write out final index

		if ! entries_buffer.is_empty () {

			flush_index_entries (
				output,
				& self.repository,
				& mut temp_files,
				& mut entries_buffer,
			) ?;

		}

		output_job.complete ();

		// remove old indexes

		let output_job =
			output_job_start! (
				output,
				"Scanning old index files");

		let old_index_ids =
			scan_index_files (
				& self.arguments.repository_path,
			) ?;

		output_job_update! (
			output_job,
			"Removing {} old index files",
			old_index_ids.len ());

		for old_index_id in old_index_ids {

			temp_files.delete (
				self.repository.index_path (
					old_index_id));

		}

		output_job.complete ();

		// commit changes

		let output_job =
			output_job_start! (
				output,
				"Committing changes");

		temp_files.commit () ?;

		output_job.remove ();

		// clean up and return

		// TODO not sure how to do this

		//self.repository.close (
		//	output);

		Ok (true)

	}

}

command! (

	name = rebuild_indexes,
	export = rebuild_indexes_command,

	arguments = RebuildIndexesArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		bundles_per_index: u64,
	},

	clap_subcommand = {

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

	},

	clap_arguments_parse = |clap_matches| {

		RebuildIndexesArguments {

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
		IndexRebuilder::new (
			output,
			arguments,
		) ?.rebuild_indexes (
			output,
		)
	},

);

// ex: noet ts=4 filetype=rust
