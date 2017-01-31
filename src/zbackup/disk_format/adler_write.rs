use std::io;
use std::io::Write;

use adler32::RollingAdler32;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;

use misc::*;

pub fn adler_write_hash <
	PrefixFunction: Fn () -> String,
> (
	prefix_function: PrefixFunction,
	adler_write: & mut AdlerWrite,
) -> Result <(), String> {

	// calculate and write hash

	let calculated_hash =
		adler_write.hash ();

	io_result_with_prefix (
		|| format! (
			"{}Error writing adler32 checksum: ",
			prefix_function ()),
		adler_write.write_u32::<LittleEndian> (
			calculated_hash),
	) ?;

	// return ok

	Ok (())

}

pub trait AdlerWrite : CloseableWrite {

	fn byte_count (& self) -> usize;

	fn hash (& self) -> u32;

	fn update (
		& mut self,
		data: & [u8],
	);

}

pub struct AdlerWriter <
	Target: Write,
> {
	target: Target,
	adler: RollingAdler32,
	byte_count: usize,
}

impl <
	Target: Write,
> AdlerWriter <Target> {

	pub fn new (
		target: Target,
	) -> AdlerWriter <Target> {

		AdlerWriter {
			target: target,
			adler: RollingAdler32::new (),
			byte_count: 0,
		}

	}

}

impl <
	Target: CloseableWrite,
> AdlerWrite for AdlerWriter <Target> {

	fn byte_count (& self) -> usize {
		self.byte_count
	}

	fn hash (& self) -> u32 {
		self.adler.hash ()
	}

	fn update (
		& mut self,
		data: & [u8],
	) {

		self.adler.update_buffer (
			data);

		self.byte_count +=
			data.len ();

	}

}

impl <
	Target: CloseableWrite,
> Write for AdlerWriter <Target> {

	fn write (
		& mut self,
		buffer: & [u8],
	) -> Result <usize, io::Error> {

		match self.target.write (
			buffer) {

			Ok (write_size) => {

				self.adler.update_buffer (
					& buffer [0 .. write_size]);

				self.byte_count +=
					write_size;

				Ok (write_size)

			},

			Err (error) =>
				Err (error),

		}

	}

	fn flush (
		& mut self,
	) -> Result <(), io::Error> {
		self.target.flush ()
	}

}

impl <
	Target: CloseableWrite,
> CloseableWrite for AdlerWriter <Target> {

	fn close (
		& mut self,
	) -> Result <(), io::Error> {
		self.target.close ()
	}

}

// ex: noet ts=4 filetype=rust
