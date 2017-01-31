use std::error::Error;
use std::io;
use std::io::Write;

pub fn io_result <Type> (
	result: Result <Type, io::Error>,
) -> Result <Type, String> {

	result.map_err (
		|io_error|

		io_error.description ().to_string ()

	)

}

pub fn io_result_with_prefix <
	PrefixFunction: Fn () -> String,
	Type,
> (
	prefix_function: PrefixFunction,
	result: Result <Type, io::Error>,
) -> Result <Type, String> {

	result.map_err (
		|io_error|

		format! (
			"{}{}",
			prefix_function (),
			io_error.description ())

	)

}

pub trait CloseableWrite: Write {

	fn close (
		& mut self,
	) -> Result <(), io::Error>;

}

pub struct CloseableWriter <
	Target: Write,
> {
	target: Target,
}

impl <Target: Write> CloseableWriter <Target> {

	pub fn wrap (
		target: Target,
	) -> CloseableWriter <Target> {

		CloseableWriter {
			target: target,
		}

	}

}

impl <Target: Write> Write for CloseableWriter <Target> {

    fn write (
		& mut self,
		buffer: & [u8],
    ) -> io::Result <usize> {

		self.target.write (
			buffer,
		)

    }

    fn flush (
		& mut self,
    ) -> io::Result <()> {

		self.target.flush ()

    }

}

impl <Target: Write> CloseableWrite for CloseableWriter <Target> {

	fn close (
		& mut self,
	) -> Result <(), io::Error> {
		self.flush ()
	}

}

impl <
	CloseableWriteSized: CloseableWrite + ?Sized
> CloseableWrite for Box <CloseableWriteSized> {

	fn close (
		& mut self,
	) -> Result <(), io::Error> {
		(** self).close ()
	}

}

// ex: noet ts=4 filetype=rust
