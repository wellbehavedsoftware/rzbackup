extern crate output;
extern crate rzbackup;

use std::env;
use std::ffi::OsString;
use std::io;

use rzbackup::Repository;

fn main () {

	let output =
		output::open ();

	let arguments: Vec <OsString> =
		env::args_os ().collect ();

	let repository =
		Repository::open (
			& output,
			Repository::default_config (),
			& arguments [1],
			if ! arguments [2].is_empty () {
				Some (& arguments [2])
			} else { None },
		).unwrap ();

	let stdout =
		io::stdout ();

	let mut stdout_lock =
		stdout.lock ();

	repository.restore (
		& output,
		& arguments [3].to_string_lossy (),
		& mut stdout_lock,
	).unwrap ();

}
 
// ex: noet ts=4 filetype=rust
