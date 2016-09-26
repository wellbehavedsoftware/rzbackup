use std::error::Error;
use std::io;
use std::io::Write;
use std::fs::File;

fn main () {

	match main_real () {

		Ok (_) => (),

		Err (error) => {

			writeln! (
				& mut io::stderr (),
				"{}",
				error,
			).unwrap_or (());

		},

	};

}

fn main_real (
) -> Result <(), String> {

	try! (

		write_metadata (
		).map_err (
			|error|

			format! (
				"Error writing metadata: {}",
				error)

		)

	);

	Ok (())

}

fn write_metadata (
) -> Result <(), String> {

	let mut file =
		try! (

		File::create (
			"src/zbackup/metadata.rs",
		).map_err (
			|io_error|

			format! (
				"Error creating metadata.rs: {}",
				io_error.description ())

		)

	);

	try! (

		writeln! (
			& mut file,
			"pub const VERSION: & 'static str = \"{}\";\n",
			env! ("CARGO_PKG_VERSION"),
		).map_err (
			|io_error|

			format! (
				"Error writing metadata.rs: {}",
				io_error.description ())

		)

	);

	Ok (())

}

// ex: noet ts=4 filetype=rust
