use std::collections::LinkedList;
use std::path::PathBuf;

use clap;

use futures::BoxFuture;
use futures::Future;

use futures_cpupool::CpuPool;

use num_cpus;

use output::Output;

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

		output.status (
			"Rebuilding indexes");

		type BundleFuture =
			BoxFuture <(BundleId, proto::BundleInfo), String>;

		let mut bundle_futures: LinkedList <BundleFuture> =
			LinkedList::new ();

		let mut bundle_ids_iter =
			bundle_ids.into_iter ();

		loop {

			// start bundle load tasks

			while bundle_futures.len () < self.num_threads {

				if let Some (bundle_id) =
					bundle_ids_iter.next () {

					let repository =
						self.repository.clone ();

					bundle_futures.push_back (
						self.cpu_pool.spawn_fn (move ||
							read_bundle_info (
								repository.bundle_path (
									bundle_id),
								repository.encryption_key (),
							).map (|bundle_info|
								(bundle_id, bundle_info)
							)
						).boxed ()
					);

				} else {
					break;
				}

			}

			// handle next bundle load

			// TODO use select for better parallelism

			if let Some (bundle_future) =
				bundle_futures.pop_front () {

				output.status_progress (
					bundle_count,
					bundle_total);

				let (bundle_id, bundle_info) =
					bundle_future.wait () ?;

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

					flush_index_entries (
						& self.repository,
						& mut temp_files,
						& mut entries_buffer,
					) ?;

				}

				bundle_count += 1;

			} else {
				break;
			}

		}

		// write out final index

		if ! entries_buffer.is_empty () {

			flush_index_entries (
				& self.repository,
				& mut temp_files,
				& mut entries_buffer,
			) ?;

		}

		output.status_done ();

		// remove old indexes

		let old_index_ids =
			scan_index_files (
				& self.arguments.repository_path,
			) ?;

		output.message_format (
			format_args! (
				"Removing {} old index files",
				old_index_ids.len ()));

		for old_index_id in old_index_ids {

			temp_files.delete (
				self.repository.index_path (
					old_index_id));

		}

		// commit changes and return

		output.status (
			"Committing changes ...");

		temp_files.commit () ?;

		output.status_done ();

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
