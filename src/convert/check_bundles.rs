use std::path::PathBuf;

use clap;

use output::Output;

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

	let bundle_ids_and_sizes: Vec <(BundleId, u64)> =
		scan_bundle_files_with_sizes (
			& arguments.repository_path,
		) ?.into_iter ().filter (
			|& (bundle_id, _bundle_size)|

			arguments.bundle_name_prefix.is_none ()

			|| bundle_id.to_string ().starts_with (
				arguments.bundle_name_prefix.as_ref ().unwrap ())

		).collect ();

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

	let mut checked_bundle_size: u64 = 0;

	let output_job =
		output_job_start! (
			output,
			"{} bundles",
			if arguments.repair { "Reparing" } else { "Checking" });

	let mut invalid_bundle_count: u64 = 0;

	for (
		bundle_id,
		bundle_size,
	) in bundle_ids_and_sizes {

		output_job.progress (
			checked_bundle_size,
			bundle_total_size);

		// read the bundle

		let bundle_path =
			repository_core.bundle_path (
				bundle_id);

		match bundle_read_path (
			bundle_path,
			repository_core.encryption_key (),
		) {

			Ok (_bundle_chunks) => {

				// TODO

			},

			Err (error) => {

				output.message (
					error);

				invalid_bundle_count += 1;

			},

		}

		// loop

		checked_bundle_size +=
			bundle_size;

	}

	if invalid_bundle_count > 0 {

		output_job_replace! (
			output_job,
			"Found {} invalid bundle files",
			invalid_bundle_count);

	} else {

		output_job_replace! (
			output_job,
			"No problems found");

	}

	// write changes to disk

	atomic_file_writer.commit () ?;

	// return

	Ok (invalid_bundle_count == 0)

}

command! (

	name = check_bundles,
	export = check_bundles_command,

	arguments = CheckBundlesArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		bundle_name_prefix: Option <String>,
		repair: bool,
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

			bundle_name_prefix:
				args::string_optional (
					& clap_matches,
					"bundle-name-prefix"),

			repair: false,

		}

	},

	action = |output, arguments| {
		check_bundles (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
