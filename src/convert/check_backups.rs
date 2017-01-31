use std::collections::HashSet;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use clap;

use rust_crypto::digest::Digest;
use rust_crypto::sha1::Sha1;

use output::Output;

use rustc_serialize::hex::ToHex;

use ::convert::utils::*;
use ::misc::*;
use ::zbackup::data::*;
use zbackup::repository::Repository;

pub fn check_backups (
	output: & Output,
	arguments: & CheckBackupsArguments,
) -> Result <bool, String> {

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

	let _atomic_file_writer =
		AtomicFileWriter::new (
			output,
			& arguments.repository_path,
			None,
		) ?;

	// load indexes

	repository.load_indexes (
		output) ?;

	// get list of backup files

	let backup_names: Vec <PathBuf> =
		scan_backup_files (
			& arguments.repository_path,
		) ?.into_iter ().filter (
			|ref backup_name|

			arguments.backup_name_hash_prefix.is_none () || {

				let mut sha1_digest =
					Sha1::new ();

				sha1_digest.input (
					backup_name.as_os_str ().as_bytes ());

				let mut sha1_sum = [0u8; 20];

				sha1_digest.result (
					& mut sha1_sum);

				sha1_sum.to_hex ().starts_with (
					arguments.backup_name_hash_prefix.as_ref ().unwrap ())

			}

		).collect ();

	if arguments.backup_name_hash_prefix.is_some () {

		output.message_format (
			format_args! (
				"Found {} backup files matching filter",
				backup_names.len ()));

	} else {

		output.message_format (
			format_args! (
				"Found {} backup files",
				backup_names.len ()));

	}

	// check backups

	let output_job =
		output_job_start! (
			output,
			"Checking backups");

	let mut checked_backup_count: u64 = 0;
	let mut error_backup_count: u64 = 0;

	for backup_name in backup_names.iter () {

		output_job.progress (
			checked_backup_count,
			backup_names.len () as u64);

		let backup_path =
			repository.path ()
				.join ("backups")
				.join (backup_name);

		let mut backup_chunks: HashSet <ChunkId> =
			HashSet::new ();

		let backup_expand_error =
			collect_chunks_from_backup (
				& repository,
				& mut backup_chunks,
				& backup_name,
			).err ();

		let missing_chunks: Vec <ChunkId> =
			backup_chunks.iter ().filter (
				|chunk_id|

				! repository.has_chunk (
					chunk_id)

			).map (|c| * c).collect ();

		if let Some (ref error) = backup_expand_error {

			output.message_format (
				format_args! (
					"Backup {} could not be expanded: {}",
					backup_name.to_string_lossy (),
					error));

		} else if ! missing_chunks.is_empty () {

			output.message_format (
				format_args! (
					"Backup {} is missing {} out of {} chunks",
					backup_name.to_string_lossy (),
					missing_chunks.len (),
					backup_chunks.len ()));

		}

		if backup_expand_error.is_some () || ! missing_chunks.is_empty () {

			if arguments.move_broken {

				let backups_broken_path =
					repository.path ()
						.join ("backups-broken");

				let backup_broken_path =
					backups_broken_path.join (
						backup_name);

				io_result (
					fs::create_dir_all (
						backup_broken_path.parent ().unwrap ()),
				) ?;

				io_result (
					fs::rename (
						backup_path,
						backup_broken_path),
				) ?;

			}

			error_backup_count += 1;

		}

		checked_backup_count += 1;

	}

	if error_backup_count > 0 {

		output_job_replace! (
			output_job,
			"{} {} backups with errors out of {} checked",
			if arguments.move_broken { "Moved" } else { "Found" },
			error_backup_count,
			backup_names.len ());

		if ! arguments.move_broken {

			output_message! (
				output,
				"Run with --move-broken to move these to backups-broken \
				directory");

		}

	} else {

		output_job_replace! (
			output_job,
			"All chunks present for {} backups checked",
			backup_names.len ());

	}

	// clean up and return

	repository.close (
		output);

	Ok (error_backup_count == 0)

}

command! (

	name = check_backups,
	export = check_backups_command,

	arguments = CheckBackupsArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
		backup_name_hash_prefix: Option <String>,
		move_broken: bool,
	},

	clap_subcommand = {

		clap::SubCommand::with_name ("check-backups")
			.about ("Checks backups for missing chunks")

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
				.help ("Move broken backups to backups-broken directory")

			)

			.arg (
				clap::Arg::with_name ("backup-name-hash-prefix")

				.long ("backup-name-hash-prefix")
				.value_name ("BACKUP-NAME-HASH-PREFIX")
				.required (false)
				.help ("Only check backups whose name's SHA1 hash start with \
					this")

			)

	},

	clap_arguments_parse = |clap_matches| {

		CheckBackupsArguments {

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

			backup_name_hash_prefix:
				args::string_optional (
					& clap_matches,
					"backup-name-hash-prefix"),

		}

	},

	action = |output, arguments| {
		check_backups (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
