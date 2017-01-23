use std::path::PathBuf;

use clap;

use output::Output;

use rustc_serialize::hex::ToHex;

use ::Repository;
use ::TempFileManager;
use ::convert::utils::*;
use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::read::*;

pub fn check_bundles (
	output: & Output,
	arguments: & CheckBundlesArguments,
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

	// begin transaction

	let mut temp_files =
		TempFileManager::new (
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

			|| bundle_id.to_hex ().starts_with (
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

	output.status_format (
		format_args! (
			"{} bundles ...",
			if arguments.repair { "Reparing" } else { "Checking" }));

	let mut invalid_bundle_count: u64 = 0;

	for (
		bundle_id,
		bundle_size,
	) in bundle_ids_and_sizes {

		output.status_progress (
			checked_bundle_size,
			bundle_total_size);

		// read the bundle

		let bundle_path =
			repository.bundle_path (
				bundle_id);

		match read_bundle (
			bundle_path,
			repository.encryption_key (),
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

	output.status_done ();

	if invalid_bundle_count > 0 {

		output.message_format (
			format_args! (
				"Found {} invalid bundle files",
				invalid_bundle_count));

	} else {

		output.message (
			"No problems found");

	}

	// commit changes

	output.status (
		"Committing changes ...");

	temp_files.commit () ?;

	output.status_done ();

	Ok (())

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

);

// ex: noet ts=4 filetype=rust
