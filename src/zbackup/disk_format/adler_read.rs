use std::io;
use std::io::BufRead;
use std::io::Read;
use std::iter;

use adler32::RollingAdler32;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use ::misc::*;

pub struct AdlerRead <Source: BufRead> {
	source: Source,
	adler: RollingAdler32,
	byte_count: usize,
}

pub fn adler_verify_hash <
	Source: BufRead,
	PrefixFunction: Fn () -> String,
> (
	prefix_function: PrefixFunction,
	adler_read: & mut AdlerRead <Source>,
) -> Result <(), String> {

	// verify hash

	let calculated_hash =
		adler_read.hash ();

	let expected_hash =
		io_result_with_prefix (
			|| format! (
				"{}Error reading adler32 checksum: ",
				prefix_function ()),
			adler_read.read_u32::<LittleEndian> (),
		) ?;

	if calculated_hash != expected_hash {

		return Err (
			format! (
				"{}Adler32 hash calculated {} but expected {}, at position \
				0x{:x}",
				prefix_function (),
				calculated_hash,
				expected_hash,
				adler_read.byte_count - 4));

	}

	// return ok

	Ok (())

}

pub fn adler_verify_hash_and_eof <
	Source: BufRead,
	PrefixFunction: Fn () -> String,
> (
	prefix_function: PrefixFunction,
	mut adler_read: AdlerRead <Source>,
) -> Result <(), String> {

	adler_verify_hash (
		& prefix_function,
		& mut adler_read,
	) ?;

	// verify end of file

	let mut byte_buffer: [u8; 1] = [0u8; 1];

	let bytes_read =
		io_result_with_prefix (
			|| format! (
				"{}Error checking for end of file: ",
				& prefix_function ()),
			adler_read.read (
				& mut byte_buffer),
		) ?;

	if bytes_read != 0 {

		return Err (
			format! (
				"{}Extra data at end of file",
				prefix_function ()));

	}

	// return ok

	Ok (())

}

impl <Source: BufRead> AdlerRead <Source> {

	pub fn new (
		source: Source,
	) -> AdlerRead <Source> {

		AdlerRead {
			source: source,
			adler: RollingAdler32::new (),
			byte_count: 0,
		}

	}

	pub fn hash (& self) -> u32 {
		self.adler.hash ()
	}

	pub fn update (
		& mut self,
		data: & [u8],
	) {

		self.adler.update_buffer (
			data);

		self.byte_count +=
			data.len ();

	}

}

impl <Source: BufRead> Read for AdlerRead <Source> {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> Result <usize, io::Error> {

		match self.source.read (
			buffer) {

			Ok (read_size) => {

				self.adler.update_buffer (
					& buffer [0 .. read_size]);

				self.byte_count +=
					read_size;

				Ok (read_size)

			},

			Err (error) =>
				Err (error),

		}

	}

}

impl <Source: BufRead> BufRead for AdlerRead <Source> {

	fn fill_buf (
		& mut self,
	) -> Result <& [u8], io::Error> {

		self.source.fill_buf ()

	}

	fn consume (
		& mut self,
		amount: usize,
	) {

		let mut buffer: Vec <u8> =
			iter::repeat (0u8)
				.take (amount)
				.collect ();

		self.source.read_exact (
			& mut buffer,
		).unwrap ();

		self.adler.update_buffer (
			& buffer);

		self.byte_count +=
			amount;

	}

}

// ex: noet ts=4 filetype=rust
