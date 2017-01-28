use std::mem;
use std::path::PathBuf;

use clap;

use futures;
use futures::BoxFuture;
use futures::Future;

use futures_cpupool::CpuPool;

use num_cpus;

use output::Output;
use output::OutputJob;

use rustc_serialize::hex::ToHex;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::read::*;
use ::zbackup::data::*;
use ::zbackup::proto;
use ::zbackup::write::*;

enum TaskResult {

	ReadBundle {
		output_job: OutputJob,
		bundle_id: BundleId,
		bundle_info: proto::BundleInfo,
	},

	WriteIndex {
		output_job: OutputJob,
	},

}

type TaskFuture =
	BoxFuture <TaskResult, String>;

struct IndexRebuilder <'a> {
	arguments: & 'a RebuildIndexesArguments,
	repository: Repository,
	max_tasks: usize,
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
			num_cpus::get ();

		let cpu_pool =
			CpuPool::new (
				num_threads);

		// return

		Ok (IndexRebuilder {
			arguments: arguments,
			repository: repository,
			max_tasks: num_threads,
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

		let output_job_main =
			output_job_start! (
				output,
				"Rebuilding indexes");

		let mut task_futures: Vec <TaskFuture> =
			Vec::new ();

		let mut bundle_ids_iter =
			bundle_ids.into_iter ();

		output.pause ();

		loop {

			// start bundle load tasks

			while task_futures.len () < self.max_tasks {

				if let Some (bundle_id) =
					bundle_ids_iter.next () {

					let repository =
						self.repository.clone ();

					let output_job_bundle =
						output_job_start! (
							output,
							"Reading bundle {}",
							bundle_id.to_hex ());

					task_futures.push (
						self.cpu_pool.spawn_fn (move || {

							let bundle_info =
								read_bundle_info (
									repository.bundle_path (
										bundle_id),
									repository.encryption_key (),
								) ?;

							Ok (TaskResult::ReadBundle {
								output_job: output_job_bundle,
								bundle_id: bundle_id,
								bundle_info: bundle_info,
							})

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

			output.unpause ();

			let (task_result, _index, remaining_task_futures) =
				futures::select_all (
					task_futures,
				).wait ().map_err (
					|(error, _index, _remaining_task_futures)|
					error,
				) ?;

			output.pause ();

			task_futures = remaining_task_futures;

			match task_result {

				TaskResult::ReadBundle {
					output_job: output_job_bundle,
					bundle_id,
					bundle_info,
				} => {

					output_job_bundle.remove ();

					output_job_main.progress (
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

						let index_id =
							index_id_generate ();

						let output_job_write_index =
							output_job_start! (
								output,
								"Writing index {}",
								index_id.to_hex ());

						task_futures.push (
							self.cpu_pool.spawn_fn (move || {

								write_index_with_id (
									& repository,
									& temp_files,
									index_id,
									& index_entries,
								) ?;

								Ok (TaskResult::WriteIndex {
									output_job: output_job_write_index,
								})

							}).boxed ()
						);

					}

					bundle_count += 1;

				},

				TaskResult::WriteIndex {
					output_job: output_job_write_index,
				} => {

					output_job_write_index.remove ();

				},

			};

		}

		output.unpause ();

		// write out final index

		if ! entries_buffer.is_empty () {

			flush_index_entries (
				output,
				& self.repository,
				& mut temp_files,
				& mut entries_buffer,
			) ?;

		}

		// remove old indexes

		let output_job_remove_indexes =
			output_job_start! (
				output,
				"Scanning old index files");

		let old_index_ids =
			scan_index_files (
				& self.arguments.repository_path,
			) ?;

		output_job_update! (
			output_job_remove_indexes,
			"Removing {} old index files",
			old_index_ids.len ());

		for old_index_id in old_index_ids {

			temp_files.delete (
				self.repository.index_path (
					old_index_id));

		}

		output_job_remove_indexes.complete ();

		// commit changes

		let output_job_commit =
			output_job_start! (
				output,
				"Committing changes");

		temp_files.commit () ?;

		output_job_commit.remove ();

		// clean up and return

		output_job_main.complete ();

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
