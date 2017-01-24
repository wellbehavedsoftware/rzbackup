use std::fs;
use std::fs::File;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use errno;

use libc;

use output::Output;

use rand;
use rand::Rng;

use ::misc::*;

pub struct TempFileManager {
	lock_fd: libc::c_int,
	temp_dir_path: PathBuf,
	temp_files: Vec <(String, PathBuf)>,
	delete_files: Vec <PathBuf>,
}

impl TempFileManager {

	pub fn new (
		output: & Output,
		repository_path: & Path,
		sleep_time: Option <Duration>,
	) -> Result <TempFileManager, String> {

		// create or open lock file

		let lock_path =
			repository_path.join ("lock");

		let lock_path_c_str: Vec <u8> =
			lock_path
				.as_os_str ().as_bytes ()
				.clone ().into_iter ()
				.chain (b"\0")
				.map (|&c| c)
				.collect ();

		let lock_fd = unsafe {
			libc::open (
				& lock_path_c_str [0]
					as * const u8
					as * const i8,
				libc::O_CREAT | libc::O_WRONLY | libc::O_TRUNC,
				0o0600,
			)
		};

		if lock_fd < 0 {

			output.clear_status ();

			return Err (
				format! (
					"Error creating lock file {}: {}",
					lock_path.to_string_lossy (),
					errno::errno ()));

		}

		// obtain lock

		match sleep_time {

			Some (sleep_time) =>
				Self::lock_non_blocking (
					output,
					sleep_time,
					lock_fd),

			None =>
				Self::lock_blocking (
					output,
					lock_fd),

		}.map_err (|error| {

			unsafe {
				libc::close (lock_fd);
			}

			format! (
				"Error obtaining lock on {}: {}",
				lock_path.to_string_lossy (),
				error)

		}) ?;

		// create tmp directory

		let temp_dir_path =
			repository_path.join ("tmp");

		if ! temp_dir_path.exists () {

			io_result_with_prefix (
				|| format! (
					"Error creating tmp directory {}: ",
					temp_dir_path.clone ().to_string_lossy ()),
				fs::create_dir (
					temp_dir_path.clone (),
				),
			).map_err (
				|error| {

				unsafe {
					libc::close (lock_fd);
				}

				error

			}) ?;

		}

		Ok (TempFileManager {
			lock_fd: lock_fd,
			temp_dir_path: temp_dir_path,
			temp_files: Vec::new (),
			delete_files: Vec::new (),
		})

	}

	fn lock_non_blocking (
		output: & Output,
		sleep_time: Duration,
		lock_fd: libc::c_int,
	) -> Result <(), String> {

		output.status (
			"Waiting for repository lock ...");

		// lock with flock

		loop {

			let flock_result = unsafe {
				libc::flock (
					lock_fd,
					libc::LOCK_EX | libc::LOCK_NB,
				)
			};

			if flock_result != 0 {

				if errno::errno () == errno::Errno (libc::EWOULDBLOCK) {

					thread::sleep (
						sleep_time);

					continue;

				}

				output.clear_status ();

				return Err (
					format! (
						"{}",
						errno::errno ()));

			}

			break;

		}

		// lock with fcntl

		let mut fcntl_flock =
			libc::flock {
				l_type: F_WRLCK,
				l_whence: libc::SEEK_SET as i16,
				l_start: 0,
				l_len: 0,
				l_pid: 0,
			};

		let fcntl_result = unsafe {
			libc::fcntl (
				lock_fd,
				libc::F_SETLKW,
				& mut fcntl_flock
					as * mut libc::flock,
			)
		};

		if fcntl_result != 0 {

			output.clear_status ();

			return Err (
				format! (
					"{}",
					errno::errno ()));

		}

		// return

		output.clear_status ();

		Ok (())

	}

	fn lock_blocking (
		output: & Output,
		lock_fd: libc::c_int,
	) -> Result <(), String> {

		output.status (
			"Waiting for repository lock ...");

		// lock with flock

		let flock_result = unsafe {
			libc::flock (
				lock_fd,
				libc::LOCK_EX,
			)
		};

		if flock_result != 0 {

			output.clear_status ();

			return Err (
				format! (
					"{}",
					errno::errno ()));

		}

		// lock with fcntl

		let mut fcntl_flock =
			libc::flock {
				l_type: F_WRLCK,
				l_whence: libc::SEEK_SET as i16,
				l_start: 0,
				l_len: 0,
				l_pid: 0,
			};

		let fcntl_result = unsafe {
			libc::fcntl (
				lock_fd,
				libc::F_SETLKW,
				& mut fcntl_flock
					as * mut libc::flock,
			)
		};

		if fcntl_result != 0 {

			output.clear_status ();

			return Err (
				format! (
					"{}",
					errno::errno ()));

		}

		// return

		output.clear_status ();

		Ok (())

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
				|| format! (
					"Error creating temporary file {}: ",
					temp_file_path.to_string_lossy ()),
				File::create (
					& temp_file_path),
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
					|| format! (
						"Error syncing temp file {}: ",
						temp_file_name),
					File::open (
						self.temp_dir_path.join (
							temp_file_name)))
			) ?;

			io_result_with_prefix (
				|| format! (
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
				|| format! (
					"Error creating target directory {}: ",
					parent_dir.to_string_lossy ()),
				fs::create_dir_all (
					parent_dir),
			) ?;

			string_result_with_prefix (
				|| format! (
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
				|| format! (
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

	pub fn changes (
		& self,
	) -> bool {

		! self.delete_files.is_empty ()
		|| ! self.temp_files.is_empty ()

	}

}

impl Drop for TempFileManager {

	fn drop (
		& mut self,
	) {

		// remove temporary files and directory

		for & (ref temp_file_name, _)
		in self.temp_files.iter () {

			fs::remove_file (
				temp_file_name,
			).unwrap_or (
				() // do nothing
			);

		}

		fs::remove_dir (
			& self.temp_dir_path,
		).unwrap_or (
			() // do nothing
		);

		// release lock

		unsafe {

			libc::close (self.lock_fd);

		}

	}

}

fn rename_or_copy_and_delete <
	SourcePath: AsRef <Path>,
	TargetPath: AsRef <Path>,
> (
	source_path: SourcePath,
	target_path: TargetPath,
) -> Result <(), String> {

	rename_or_copy_and_delete_impl (
		source_path.as_ref (),
		target_path.as_ref (),
	)

}

fn rename_or_copy_and_delete_impl (
	source_path: & Path,
	target_path: & Path,
) -> Result <(), String> {

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
			).map_err (|error|
				format! (
					"Error opening source file: {}",
					error)
			) ?;

		let target_temp_path =
			target_path.with_extension (
				"temp");

		let mut target_temp =
			File::create (
				& target_temp_path,
			).map_err (|error|
				format! (
					"Error opening target temp file {}: {}",
					target_temp_path.to_string_lossy (),
					error)
			) ?;

		io::copy (
			& mut source,
			& mut target_temp,
		).map_err (|error|
			format! (
				"Error copying contents to target temp file {}: {}",
				target_temp_path.to_string_lossy (),
				error)
		) ?;

		target_temp.sync_all (
		).map_err (|error|
			format! (
				"Error syncing target temp file {}: {}",
				target_temp_path.to_string_lossy (),
				error)
		) ?;

		fs::rename (
			& target_temp_path,
			target_path,
		).map_err (|error|
			format! (
				"Error renaming temp file {}: {}",
				target_temp_path.to_string_lossy (),
				error)
		) ?;

	}

	// delete the original

	fs::remove_file (
		source_path,
	).map_err (|error|
		format! (
			"Error removing source file: {}",
			error)
	) ?;

	Ok (())

}

const F_WRLCK: libc::c_short = 1;

// ex: noet ts=4 filetype=rust
