use std::path::PathBuf;

use clap;

use output::Output;

use rand;
use rand::Rng;

use rustc_serialize::hex::ToHex;

use convert::utils::*;
use misc::*;
use zbackup::data::*;
use zbackup::disk_format::*;
use zbackup::repository::*;

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

	let atomic_file_writer =
		AtomicFileWriter::new (
			output,
			& arguments.repository_path,
			None,
		) ?;

	// load indexes

	repository.load_indexes (
		output,
	) ?;

	// get list of index files

	scan_indexes_with_sizes! (
		output,
		arguments.repository_path,
		old_index_ids_and_sizes,
		total_index_size,
	);

	// get list of backup files

	let backup_files =
		scan_backups (
			output,
			& arguments.repository_path,
		) ?;

	// get a list of chunks used by backups

	let backup_chunk_ids =
		get_recursive_chunks (
			output,
			& repository,
			& backup_files,
		) ?;

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
			index_read_path (
				& old_index_path,
				repository.encryption_key (),
			) ?;

		// skip this index if all chunks are still referenced

		if ! old_index_entries.iter ().any (
			|& RawIndexEntry {
				index_bundle_header: ref _old_index_bundle_header,
				bundle_info: ref old_index_bundle_info,
			}|

			old_index_bundle_info.chunks ().any (
				|ref old_chunk_record| {

				! backup_chunk_ids.contains (
					& old_chunk_record.chunk_id (),
				)

			})

		) {

			old_index_progress += 1;

			continue;

		}

		// rewrite the index

		let mut new_index_entries: Vec <RawIndexEntry> =
			Vec::new ();

		for RawIndexEntry {
			index_bundle_header: old_index_bundle_header,
			bundle_info: old_index_bundle_info,
		} in old_index_entries.into_iter () {

			// skip this bundle if there are no referenced chunks

			if ! old_index_bundle_info.chunks ().any (
				|ref old_chunk_record| {

				backup_chunk_ids.contains (
					& old_chunk_record.chunk_id ())

			}) {

				chunks_removed +=
					old_index_bundle_info.num_chunks ();

				continue;

			}

			// create a new index entry for this bundle

			let mut new_bundle_chunks =
				Vec::new ();

			for old_bundle_chunk
			in old_index_bundle_info.chunks () {

				if backup_chunk_ids.contains (
					& old_bundle_chunk.chunk_id ()) {

					new_bundle_chunks.push (
						old_bundle_chunk.clone ());

				} else {

					chunks_removed += 1;

				}

			}

			new_index_entries.push (
				RawIndexEntry {

					index_bundle_header:
						old_index_bundle_header,

					bundle_info:
						DiskBundleInfo::new (
							new_bundle_chunks),

				}
			);

		}

		atomic_file_writer.delete (
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

			let mut new_index_file =
				atomic_file_writer.create (
					new_index_path,
				) ?;

			index_write_direct (
				& mut new_index_file,
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

	atomic_file_writer.commit () ?;

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
