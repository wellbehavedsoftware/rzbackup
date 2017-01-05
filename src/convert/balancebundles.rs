use std::path::PathBuf;
use std::process;

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

pub fn balance_bundles_command (
) -> Box <Command> {

	Box::new (
		BalanceBundlesCommand {},
	)

}

pub struct BalanceBundlesArguments {
	repository_path: PathBuf,
	password_file_path: Option <PathBuf>,
	chunks_per_bundle: u64,
	fill_factor: u64,
}

pub struct BalanceBundlesCommand {
}

pub fn balance_bundles (
	output: & Output,
	arguments: & BalanceBundlesArguments,
) -> Result <(), String> {

	// open repository

	let repository = match (

		Repository::open (
			& output,
			Repository::default_config (),
			& arguments.repository_path,
			arguments.password_file_path.clone ())

	) {

		Ok (repository) =>
			repository,

		Err (error) => {

			output.message_format (
				format_args! (
					"Error opening repository {}: {}",
					arguments.repository_path.to_string_lossy (),
					error));

			process::exit (1);

		},

	};

	// get list of index files

	let old_indexes = (
		scan_index_files (
			& arguments.repository_path)
	) ?;

	let total_index_size =
		old_indexes.iter ().map (
			|& (_, old_index_size)|
			old_index_size
		).sum ();

	output.message_format (
		format_args! (
			"Found {} index files with total size {}",
			old_indexes.len (),
			total_index_size));

	// read indexes and discard any which are balanced

	output.status (
		"Reading indexes ...");

	let mut unbalanced_indexes: Vec <(String, Vec <IndexEntry>)> =
		Vec::new ();

	let mut read_index_size: u64 = 0;
	let mut unbalanced_chunks_count: u64 = 0;

	let minimum_chunk_count: u64 =
		arguments.chunks_per_bundle * arguments.fill_factor / 100;

	for (old_index_name, old_index_size) in old_indexes {

		let old_index_path =
			arguments.repository_path
				.join ("index")
				.join (& old_index_name);

		let old_index_entries = (
			read_index (
				& old_index_path,
				repository.encryption_key ())
		) ?;

		let old_index_unbalanced_chunks_count =
			old_index_entries.iter ().map (
				|& (_, ref bundle_info)|

				bundle_info.get_chunk_record ().len () as u64

			).filter (
				|& chunk_count|

				chunk_count < minimum_chunk_count

			).sum ();

		if old_index_unbalanced_chunks_count > 0 {

			unbalanced_indexes.push (
				(
					old_index_name,
					old_index_entries,
				)
			);

		}

		unbalanced_chunks_count +=
			old_index_unbalanced_chunks_count;

		read_index_size +=
			old_index_size;

		output.status_progress (
			read_index_size,
			total_index_size);


	}

	output.status_done ();

	let new_bundles_total: u64 =
		(unbalanced_chunks_count + arguments.chunks_per_bundle - 1)
			/ arguments.chunks_per_bundle;

	output.message_format (
		format_args! (
			"Found {} chunks to balance into {} bundles",
			unbalanced_chunks_count,
			new_bundles_total));

	// balance bundles

	output.status (
		"Reading bundles");

	let mut new_bundles_count: u64 = 0;

	let mut balanced_chunks: Vec <(ChunkId, Vec <u8>)> =
		Vec::new ();

	let mut new_index_entries: Vec <IndexEntry> =
		Vec::new ();

	let mut temp_files =
		TempFileManager::new (
			& arguments.repository_path,
		) ?;

	for (unbalanced_index_name, unbalanced_index_entries)
	in unbalanced_indexes {

		for (unbalanced_index_bundle_header, unbalanced_index_bundle_info)
		in unbalanced_index_entries {

			let unbalanced_bundle_id =
				unbalanced_index_bundle_header.get_id ().to_owned ();

			let unbalanced_bundle_id_hex =
				unbalanced_bundle_id.to_hex ();

			if unbalanced_index_bundle_info.get_chunk_record ().len () as u64
				>= minimum_chunk_count {

				// bundle meets fill factor, nothing to do

				new_index_entries.push (
					(
						unbalanced_index_bundle_header,
						unbalanced_index_bundle_info,
					)
				);

			} else {

				// bundle does not meet fill factor, rebundle its contents

				let unbalanced_bundle_path =
					repository.path ()
						.join ("bundles")
						.join (& unbalanced_bundle_id_hex [0 .. 2])
						.join (& unbalanced_bundle_id_hex);

				let unbalanced_bundle =
					read_bundle (
						& unbalanced_bundle_path,
						repository.encryption_key ()
					) ?;

				for (unbalanced_chunk_id, unbalanced_chunk_data)
				in unbalanced_bundle {

					balanced_chunks.push (
						(
							unbalanced_chunk_id,
							unbalanced_chunk_data,
						)
					);

					if balanced_chunks.len () as u64
						== arguments.chunks_per_bundle {

						output.clear_status ();

						flush_bundle (
							output,
							& repository,
							& mut temp_files,
							& mut balanced_chunks,
							& mut new_index_entries,
							new_bundles_count,
							new_bundles_total,
						) ?;

						new_bundles_count += 1;

						output.status (
							"Reading bundles");

					}

				}

				temp_files.delete (
					unbalanced_bundle_path);

			}

		}

		temp_files.delete (
			repository.path ()
				.join ("index")
				.join (unbalanced_index_name));

	}

	output.clear_status ();

	// write final bundle and/or index

	flush_bundle (
		output,
		& repository,
		& mut temp_files,
		& mut balanced_chunks,
		& mut new_index_entries,
		new_bundles_count,
		new_bundles_total,
	) ?;

	process::exit (0);

}

fn flush_bundle (
	output: & Output,
	repository: & Repository,
	temp_files: & mut TempFileManager,
	balanced_chunks: & mut Vec <(ChunkId, Vec <u8>)>,
	new_index_entries: & mut Vec <IndexEntry>,
	new_bundles_count: u64,
	new_bundles_total: u64,
) -> Result <(), String> {

	if ! balanced_chunks.is_empty () {

		output.status_format (
			format_args! (
				"Writing bundle {} of {} ...",
				new_bundles_count + 1,
				new_bundles_total));

	} else {

		output.status (
			"Writing final index");

	}

	// write bundle

	if ! balanced_chunks.is_empty () {

		let new_bundle_bytes: Vec <u8> =
			rand::thread_rng ()
				.gen_iter::<u8> ()
				.take (24)
				.collect ();

		let new_bundle_name: String =
			new_bundle_bytes.to_hex ();

		let new_bundle_path =
			repository.path ()
				.join ("bundles")
				.join (& new_bundle_name [0 .. 2])
				.join (& new_bundle_name);

		let new_bundle_file =
			Box::new (
				temp_files.create (
					new_bundle_path,
				) ?
			);

		let total_chunks =
			balanced_chunks.len () as u64;

		let new_index_bundle_info =
			write_bundle (
				new_bundle_file,
				repository.encryption_key (),
				& balanced_chunks,
				|chunks_written| {

					output.status_progress (
						chunks_written,
						total_chunks)

				}
			) ?;

		let mut new_index_bundle_header =
			proto::IndexBundleHeader::new ();

		new_index_bundle_header.set_id (
			new_bundle_bytes);

		new_index_entries.push (
			(
				new_index_bundle_header,
				new_index_bundle_info,
			)
		);

		balanced_chunks.clear ();

	}

	// write indexes

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
				.join (& new_index_name);

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

		new_index_entries.clear ();

	}

	// commit and return

	temp_files.commit () ?;

	output.status_done ();

	Ok (())

}

impl CommandArguments for BalanceBundlesArguments {

	fn perform (
		& self,
		output: & Output,
	) -> Result <(), String> {

		balance_bundles (
			output,
			self,
		)

	}

}

impl Command for BalanceBundlesCommand {

	fn name (& self) -> & 'static str {
		"balance-bundles"
	}

	fn clap_subcommand <'a: 'b, 'b> (
		& self,
	) -> clap::App <'a, 'b> {

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

	}

	fn clap_arguments_parse (
		& self,
		clap_matches: & clap::ArgMatches,
	) -> Box <CommandArguments> {

		let arguments = BalanceBundlesArguments {

			repository_path:
				args::path_required (
					& clap_matches,
					"repository"),

			password_file_path:
				args::path_optional (
					& clap_matches,
					"password-file"),

			chunks_per_bundle:
				args::u64_required (
					& clap_matches,
					"chunks-per-bundle"),

			fill_factor:
				args::u64_required (
					& clap_matches,
					"fill-factor"),

		};

		if arguments.fill_factor > 100 {

			args::error_exit (
				format! (
					"Value of --fill-factor must be between 0 and 100"));

		}

		Box::new (arguments)

	}

}

// ex: noet ts=4 filetype=rust
