use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use rand;
use rand::Rng;

use ::misc::*;

pub struct TempFileManager {
	temp_dir_path: PathBuf,
	temp_files: Vec <(String, PathBuf)>,
	delete_files: Vec <PathBuf>,
}

impl TempFileManager {

	pub fn new (
		repository_path: & Path,
	) -> Result <TempFileManager, String> {

		let temp_dir_path =
			repository_path.join ("tmp");

		if ! temp_dir_path.exists () {

			io_result_with_prefix (
				format! (
					"Error creating tmp directory {}: ",
					temp_dir_path.clone ().to_string_lossy ()),
				fs::create_dir (
					temp_dir_path.clone (),
				),
			) ?;

		}

		Ok (TempFileManager {
			temp_dir_path: temp_dir_path,
			temp_files: Vec::new (),
			delete_files: Vec::new (),
		})

	}

	pub fn create (
		& mut self,
		target_path: PathBuf,
	) -> Result <File, String> {

		let temp_file_name: String =
			rand::thread_rng ()
				.gen_ascii_chars ()
				.take (16)
				.collect ();

		let temp_file_path =
			self.temp_dir_path.join (
				& temp_file_name);

		let temp_file =
			io_result_with_prefix (
				format! (
					"Error creating temporary file {}: ",
					temp_file_path.to_string_lossy ()),
				File::create (
					temp_file_path),
			) ?;

		self.temp_files.push (
			(
				temp_file_name,
				target_path,
			),
		);

		Ok (temp_file)

	}

	pub fn delete (
		& mut self,
		delete_path: PathBuf,
	) {

		self.delete_files.push (
			delete_path)

	}

	pub fn commit (
		& mut self
	) -> Result <(), String> {

		// sync all temp files

		for & (ref temp_file_name, _)
		in self.temp_files.iter () {

			let temp_file = (
				io_result_with_prefix (
					format! (
						"Error syncing temp file {}: ",
						temp_file_name),
					File::open (
						self.temp_dir_path.join (
							temp_file_name)))
			) ?;

			io_result_with_prefix (
				format! (
					"Error syncing temp file {}: ",
					temp_file_name),
				temp_file.sync_all ()
			) ?;

		}

		// rename temp files

		for & (ref temp_file_name, ref target_path)
		in self.temp_files.iter () {

			let parent_dir =
				target_path.parent ().unwrap ();

			io_result_with_prefix (
				format! (
					"Error creating target directory {}: ",
					parent_dir.to_string_lossy ()),
				fs::create_dir_all (
					parent_dir),
			) ?;

			io_result_with_prefix (
				format! (
					"Error renaming temp file {} to {}: ",
					temp_file_name,
					target_path.to_string_lossy ()),
				rename_or_copy_and_delete (
					self.temp_dir_path.join (
						temp_file_name),
					target_path)
			) ?;

		}

		self.temp_files.clear ();

		// delete files

		for delete_file_name in self.delete_files.iter () {

			io_result_with_prefix (
				format! (
					"Error deleting {}: ",
					delete_file_name.to_string_lossy ()),
				fs::remove_file (
					delete_file_name),
			) ?;

		}

		self.delete_files.clear ();

		// return

		Ok (())

	}

}

fn rename_or_copy_and_delete <
	SourcePath: AsRef <Path>,
	TargetPath: AsRef <Path>,
> (
	source_path: SourcePath,
	target_path: TargetPath,
) -> Result <(), io::Error> {

	let source_path = source_path.as_ref ();
	let target_path = target_path.as_ref ();

	// try a simple rename

	if let Ok (()) = (
		fs::rename (
			source_path,
			target_path)
	) {
		return Ok (());
	}

	// copy the file contents

	{

		let mut source =
			File::open (
				source_path,
			) ?;

		let mut target =
			File::create (
				target_path,
			) ?;

		io::copy (
			& mut source,
			& mut target,
		) ?;

		target.sync_all () ?;

	}

	// delete the original

	fs::remove_file (
		source_path,
	) ?;

	Ok (())

}

// ex: noet ts=4 filetype=rust
