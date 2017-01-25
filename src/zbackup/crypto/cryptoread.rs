use crypto::aessafe::AesSafe128Decryptor;
use crypto::buffer::BufferResult;
use crypto::buffer::ReadBuffer;
use crypto::buffer::RefReadBuffer;
use crypto::buffer::RefWriteBuffer;
use crypto::buffer::WriteBuffer;
use crypto::blockmodes::CbcDecryptor;
use crypto::blockmodes::DecPadding;
use crypto::blockmodes::PkcsPadding;
use crypto::symmetriccipher::Decryptor;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use zbackup::data::*;

type DecryptorType =
	CbcDecryptor <
		AesSafe128Decryptor,
		DecPadding <PkcsPadding>,
	>;

/// This provides a reader for an encrypted ZBackup file. It is used internally,
/// but also made available publicly, since it may be useful in some cases.

pub struct CryptoReader {

	input: File,

	decryptor: DecryptorType,

	ciphertext_buffer: [u8; BUFFER_SIZE],
	ciphertext_start: usize,
	ciphertext_end: usize,
	ciphertext_eof: bool,

	plaintext_buffer: [u8; BUFFER_SIZE],
	plaintext_start: usize,
	plaintext_end: usize,
	plaintext_eof: bool,

}

impl CryptoReader {

	#[ inline ]
	pub fn open <PathRef: AsRef <Path>> (
		path: PathRef,
		encryption_key: [u8; KEY_SIZE],
	) -> io::Result <CryptoReader> {

		Self::open_impl (
			path.as_ref (),
			encryption_key,
		)

	}

	pub fn open_impl (
		path: & Path,
		encryption_key: [u8; KEY_SIZE],
	) -> io::Result <CryptoReader> {

		// open file

		let file =
			File::open (
				path,
			) ?;

		// setup decryptor

		let decryptor =
			CbcDecryptor::new (
				AesSafe128Decryptor::new (
					& encryption_key),
				PkcsPadding,
				[0u8; KEY_SIZE].to_vec ());

		// return

		Ok (CryptoReader {

			input: file,

			decryptor: decryptor,

			ciphertext_buffer: [0u8; BUFFER_SIZE],
			ciphertext_start: 0,
			ciphertext_end: 0,
			ciphertext_eof: false,

			plaintext_buffer: [0u8; BUFFER_SIZE],
			plaintext_start: 0,
			plaintext_end: 0,
			plaintext_eof: false,

		})

	}

	fn read_and_decrypt (
		& mut self,
	) -> io::Result <()> {

		assert! (
			! self.plaintext_eof);

		assert! (
			self.plaintext_start == self.plaintext_end);

		self.plaintext_start = 0;
		self.plaintext_end = 0;

		loop {

			// read in some more data

			if ! self.ciphertext_eof
			&& self.ciphertext_start == self.ciphertext_end {

				self.ciphertext_start = 0;

				self.ciphertext_end =
					self.input.read (
						& mut self.ciphertext_buffer,
					) ?;

				if self.ciphertext_end == 0 {
					self.ciphertext_eof = true;
				}

			}

			// decrypt the data in the buffer

			let mut read_buffer =
				RefReadBuffer::new (
					& mut self.ciphertext_buffer [
						self.ciphertext_start
					..
						self.ciphertext_end
					]);

			let mut write_buffer =
				RefWriteBuffer::new (
					& mut self.plaintext_buffer [
						self.plaintext_end .. ]);

			let decrypt_result =
				self.decryptor.decrypt (
					& mut read_buffer,
					& mut write_buffer,
					self.ciphertext_eof,
				).map_err (
					|_|

					io::Error::new (
						io::ErrorKind::InvalidData,
						"Decryption failed")

				) ?;

			self.ciphertext_start +=
				read_buffer.position ();

			self.plaintext_end +=
				write_buffer.position ();

			if write_buffer.position () == 0 {
				break;
			}

			match decrypt_result {

				BufferResult::BufferUnderflow =>
					continue,

				BufferResult::BufferOverflow =>
					break,

			};

		}

		if self.plaintext_start == self.plaintext_end {
			self.plaintext_eof = true;
		}

		Ok (())

	}

}

impl Read for CryptoReader {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> io::Result <usize> {

		let mut total_bytes_read: usize =
			0;

		// loop to fill buffer while decrypting

		while (
			! (
				self.plaintext_eof
				&& self.plaintext_start == self.plaintext_end
			)
			&& total_bytes_read < buffer.len ()
		) {

			// read more data if appropriate

			if self.plaintext_start == self.plaintext_end {
				self.read_and_decrypt () ?;
			}

			// check available data

			let buffer_remaining =
				& mut buffer [total_bytes_read .. ];

			let available_bytes =
				self.plaintext_end - self.plaintext_start;

			if available_bytes == 0 {
				return Ok (total_bytes_read);
			}

			// if we have enough data, copy that and we're done

			if buffer_remaining.len () <= available_bytes {

				let buffer_remaining_len =
					buffer_remaining.len ();

				buffer_remaining.copy_from_slice (
					& self.plaintext_buffer [
						self.plaintext_start ..
						self.plaintext_start + buffer_remaining_len]);

				self.plaintext_start +=
					buffer_remaining_len;

				total_bytes_read +=
					buffer_remaining_len;

			} else {

				// we have some data but not enough, copy, decrypt and loop

				buffer_remaining [
					0 .. available_bytes
				].copy_from_slice (
					& self.plaintext_buffer [
						self.plaintext_start ..
						self.plaintext_start + available_bytes]
				);

				self.plaintext_start +=
					available_bytes;

				total_bytes_read +=
					available_bytes;

			}

		}

		Ok (total_bytes_read)

	}

}

// ex: noet ts=4 filetype=rs
