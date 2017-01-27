use std::collections::HashSet;
use std::path::PathBuf;

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

pub fn gc_indexes (
	output: & Output,
	arguments: & GcIndexesArguments,
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

	let temp_files =
		TempFileManager::new (
			output,
			& arguments.repository_path,
			None,
		) ?;

	// load indexes

	repository.load_indexes (
		output,
	) ?;

	// get list of index files

	let output_job =
		output_job_start! (
			output,
			"Scanning indexes");

	let old_index_ids_and_sizes =
		scan_index_files_with_sizes (
			& arguments.repository_path,
		) ?;

	let total_index_size: u64 =
		old_index_ids_and_sizes.iter ().map (
			|& (_, old_index_size)|
			old_index_size
		).sum ();

	output_job_replace! (
		output_job,
		"Found {} index files",
		old_index_ids_and_sizes.len ());

	// get list of backup files

	let output_job =
		output_job_start! (
			output,
			"Scanning backups");

	let backup_files =
		scan_backup_files (
			& arguments.repository_path,
		) ?;

	output_job_replace! (
		output_job,
		"Found {} backup files",
		backup_files.len ());

	// get a list of chunks used by backups

	let output_job =
		output_job_start! (
			output,
			"Reading backups");

	let mut backup_chunk_ids: HashSet <ChunkId> =
		HashSet::new ();

	let mut backup_count: u64 = 0;
	let backup_total = backup_files.len () as u64;

	for backup_file in backup_files {

		output_job.progress (
			backup_count,
			backup_total);

		collect_chunks_from_backup (
			& repository,
			& mut backup_chunk_ids,
			& backup_file,
		) ?;

		backup_count += 1;

	}

	output_job_replace! (
		output_job,
		"Found {} chunks referenced by backups",
		backup_chunk_ids.len ());

	// process indexes

	let output_job =
		output_job_start! (
			output,
			"Garbage collecting indexes");

	let mut old_index_progress: u64 = 0;

	let mut chunks_removed: u64 = 0;
	let mut indexes_modified: u64 = 0;
	let mut indexes_removed: u64 = 0;

	for (
		old_index_id,
		old_index_size,
	) in old_index_ids_and_sizes {

		output_job.progress (
			old_index_progress,
			total_index_size);

		let old_index_path =
			repository.index_path (
				old_index_id);

		let old_index_entries =
			read_index (
				& old_index_path,
				repository.encryption_key (),
			) ?;

		// skip this index if all chunks are still referenced

		if ! old_index_entries.iter ().any (
			|& (
				ref _old_index_bundle_header,
				ref old_index_bundle_info,
			)|

			old_index_bundle_info.get_chunk_record ().iter ().any (
				|ref old_chunk_record| {

				! backup_chunk_ids.contains (
					old_chunk_record.get_id ())

			})

		) {

			old_index_progress += 1;

			continue;

		}

		// rewrite the index

		let mut new_index_entries: Vec <IndexEntry> =
			Vec::new ();

		for & (
			ref old_index_bundle_header,
			ref old_index_bundle_info,
		) in old_index_entries.iter () {

			// skip this bundle if there are no referenced chunks

			if ! old_index_bundle_info.get_chunk_record ().iter ().any (
				|ref old_chunk_record| {

				backup_chunk_ids.contains (
					old_chunk_record.get_id ())

			}) {

				chunks_removed +=
					old_index_bundle_info.get_chunk_record ().len () as u64;

				continue;

			}

			// create a new index entry for this bundle

			let new_index_bundle_header =
				old_index_bundle_header.clone ();

			let mut new_index_bundle_info =
				proto::BundleInfo::new ();

			for old_chunk_record
			in old_index_bundle_info.get_chunk_record ().iter () {

				if backup_chunk_ids.contains (
					old_chunk_record.get_id ()) {

					new_index_bundle_info.mut_chunk_record ().push (
						old_chunk_record.clone ());

				} else {

					chunks_removed += 1;

				}

			}

			new_index_entries.push (
				(
					new_index_bundle_header,
					new_index_bundle_info,
				)
			);

		}

		temp_files.delete (
			old_index_path);

		if ! new_index_entries.is_empty () {

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
					.join (new_index_name);

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

			indexes_modified += 1;

		} else {

			indexes_removed += 1;

		}

		old_index_progress +=
			old_index_size;

	}

	output_job_replace! (
		output_job,
		"Removed {} chunks from {} modified and {} deleted indexes",
		chunks_removed,
		indexes_modified,
		indexes_removed);

	// commit changes

	let output_job =
		output_job_start! (
			output,
			"Committing changes");

	temp_files.commit () ?;

	output_job.remove ();

	// clean up and return

	repository.close (
		output);

	Ok (true)

}

command! (

	name = gc_indexes,
	export = gc_indexes_command,

	arguments = GcIndexesArguments {
		repository_path: PathBuf,
		password_file_path: Option <PathBuf>,
	},

	clap_subcommand = {

		clap::SubCommand::with_name ("gc-indexes")
			.about ("Removes index entries which are not referenced by any \
				backup")

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

	},

	clap_arguments_parse = |clap_matches| {

		GcIndexesArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

		}

	},

	action = |output, arguments| {
		gc_indexes (output, arguments)
	},

);

// ex: noet ts=4 filetype=rust
