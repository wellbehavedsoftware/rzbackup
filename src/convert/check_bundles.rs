use std::fs;
use std::path::PathBuf;
use std::slice;

use clap;

use futures::Future;
use futures_cpupool::CpuPool;

use num_cpus;

use output::Output;
use output::OutputJob;

use ::convert::utils::*;
use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::disk_format::*;
use ::zbackup::repository_core::*;

pub fn check_bundles (
	output: & Output,
	arguments: & CheckBundlesArguments,
) -> Result <bool, String> {

	// open repository

	let repository_core =
		string_result_with_prefix (
			|| format! (
				"Error opening repository {}: ",
				arguments.repository_path.to_string_lossy ()),
			RepositoryCore::open (
				& output,
				& arguments.repository_path,
				arguments.password_file_path.clone ()),
		) ?;

	// begin transaction

	let atomic_file_writer =
		AtomicFileWriter::new (
			output,
			& arguments.repository_path,
			None,
		) ?;

	// get a list of index files

	let mut bundle_ids_and_sizes: Vec <(BundleId, u64)> =
		scan_bundle_files_with_sizes (
			& arguments.repository_path,
		) ?.into_iter ().filter (
			|& (bundle_id, _bundle_size)|

			arguments.bundle_name_prefix.is_none ()

			|| bundle_id.to_string ().starts_with (
				arguments.bundle_name_prefix.as_ref ().unwrap ())

		).collect ();

	bundle_ids_and_sizes.sort_by_key (
		|& (bundle_id, _bundle_size)| bundle_id);

	let bundle_total_size: u64 =
		bundle_ids_and_sizes.iter ().map (
			|& (_, bundle_size)|
			bundle_size as u64
		).sum ();

	output.message_format (
		format_args! (
			"Found {} bundle files",
			bundle_ids_and_sizes.len ()));

	// check bundles

	let num_threads =
		(num_cpus::get () - 1) * 5 / 3 + 1;

	let cpu_pool =
		CpuPool::new (
			num_threads);

	let invalid_bundle_count =
		check_bundles_real (
			output,
			& cpu_pool,
			num_threads,
			arguments,
			repository_core.clone (),
			& bundle_ids_and_sizes,
			bundle_total_size,
		) ?;

	// write changes to disk

	atomic_file_writer.commit () ?;

	// return

	Ok (invalid_bundle_count == 0)

}

fn check_bundles_real (
	output: & Output,
	cpu_pool: & CpuPool,
	max_tasks: usize,
	arguments: & CheckBundlesArguments,
	repository_core: RepositoryCore,
	bundle_ids_and_sizes: & Vec <(BundleId, u64)>,
	bundle_total_size: u64,
) -> Result <u64, String> {

	// check bundles

	struct Task {
		bundle_size: u64,
		output_job: OutputJob,
		result: Result <(), String>,
	}

	struct State <'a> {
		bundle_ids_and_sizes_iterator: slice::Iter <'a, (BundleId, u64)>,
		checked_bundle_size: u64,
		invalid_bundle_count: u64,
		output_job: OutputJob,
	}

	let output_job =
		output_job_start! (
			output,
			"Checking bundles");

	let mut state = State {
		output_job: output_job,
		bundle_ids_and_sizes_iterator: bundle_ids_and_sizes.iter (),
		checked_bundle_size: 0,
		invalid_bundle_count: 0,
	};

	// concurrent operation

	concurrent_controller (
		output,
		max_tasks,
		& mut state,

		|state| {

			if let Some (& (bundle_id, bundle_size)) =
				state.bundle_ids_and_sizes_iterator.next () {

				let output =
					output.clone ();

				let repository_core =
					repository_core.clone ();

				let move_broken =
					arguments.move_broken;

				Some (cpu_pool.spawn_fn (move || {

					let output_job =
						output_job_start! (
							output,
							"Checking bundle {}",
							bundle_id);

					Ok (Task {
						bundle_size: bundle_size,
						output_job: output_job,
						result: check_bundle (
							repository_core,
							move_broken,
							bundle_id,
						),
					})

				}).boxed ())

			} else { None }

		},

		|state, task_value| {

			state.checked_bundle_size +=
				task_value.bundle_size;

			state.output_job.progress (
				state.checked_bundle_size,
				bundle_total_size);

			if let Err (error) = task_value.result {

				output.message (
					error);

				state.invalid_bundle_count += 1;

			}

			task_value.output_job.remove ();

			Ok (())

		},

	) ?;

	if state.invalid_bundle_count > 0 {

		output_job_replace! (
			state.output_job,
			"Found {} invalid bundle files",
			state.invalid_bundle_count);

	} else {

		output_job_replace! (
			state.output_job,
			"No problems found");

	}

	Ok (state.invalid_bundle_count)

}

fn check_bundle (
	repository_core: RepositoryCore,
	move_broken: bool,
	bundle_id: BundleId,
) -> Result <(), String> {

	let bundle_path =
		repository_core.bundle_path (
			bundle_id);

	match bundle_read_path (
		& bundle_path,
		repository_core.encryption_key (),
	) {

		Ok (_bundle_chunks) =>
			Ok (()),

		Err (error) => {

			if move_broken {

				let bundles_broken_path =
					repository_core.path ()
						.join ("bundles-broken");

				io_result (
					fs::create_dir_all (
						& bundles_broken_path),
				).map_err (
					|error|

					format! (
						"Error creating {}: {}",
						bundles_broken_path.to_string_lossy (),
						error)

				) ?;

				let bundle_broken_path =
					bundles_broken_path.join (
						bundle_path.file_name ().unwrap ());

				rename_or_copy_and_delete (
					& bundle_path,
					& bundle_broken_path,
				).map_err (
					|error|

					format! (
						"Error moving {} to {}: {}",
						bundle_path.to_string_lossy (),
						bundle_broken_path.to_string_lossy (),
						error)

				) ?;

			}

			Err (error)

		},

	}

}

command! (

	name = check_bundles,
	export = check_bundles_command,

	arguments = CheckBundlesArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		move_broken: bool,
		bundle_name_prefix: Option <String>,
	},

	clap_subcommand = {

		clap::SubCommand::with_name ("check-bundles")
			.about ("Checks bundle files for basic consistency")

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
				clap::Arg::with_name ("move-broken")

				.long ("move-broken")
				.help ("Move broken bundles to bundles-broken directory")

			)

			.arg (
				clap::Arg::with_name ("bundle-name-prefix")

				.long ("bundle-name-prefix")
				.value_name ("BUNDLE-NAME-PREFIX")
				.required (false)
				.help ("Only check bundles whose name start with this")

			)

	},

	clap_arguments_parse = |clap_matches| {

		CheckBundlesArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

			move_broken:
				args::bool_flag (
					& clap_matches,
					"move-broken"),

			bundle_name_prefix:
				args::string_optional (
					& clap_matches,
					"bundle-name-prefix"),

		}

	},

	action = |output, arguments| {
		check_bundles (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
