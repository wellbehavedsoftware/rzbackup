use crypto;
use crypto::aessafe::AesSafe128Decryptor;
use crypto::buffer::BufferResult;
use crypto::buffer::ReadBuffer;
use crypto::buffer::RefReadBuffer;
use crypto::buffer::RefWriteBuffer;
use crypto::buffer::WriteBuffer;
use crypto::blockmodes::CbcDecryptor;
use crypto::blockmodes::DecPadding;
use crypto::blockmodes::PkcsPadding;
use crypto::mac::Mac;
use crypto::symmetriccipher::BlockDecryptor;
use crypto::symmetriccipher::Decryptor;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use zbackup::data::*;
use zbackup::proto;

use misc::*;

type DecryptorType =
	CbcDecryptor <
		AesSafe128Decryptor,
		DecPadding <PkcsPadding>,
	>;

/// This provides a reader for an encrypted ZBackup file. It is used internally,
/// but also made available publicly, since it may be useful in some cases.

pub struct CryptoReader {

	input: File,
	eof: bool,

	decryptor: DecryptorType,

	ciphertext_buffer: [u8; BUFFER_SIZE],
	plaintext_buffer: [u8; BUFFER_SIZE],

	ciphertext_start: usize,
	ciphertext_end: usize,

	plaintext_start: usize,
	plaintext_end: usize,

}

impl CryptoReader {

	pub fn open <PathRef: AsRef <Path>> (
		path: PathRef,
		encryption_key: [u8; KEY_SIZE],
	) -> io::Result <CryptoReader> {

		// open file

		let mut file =
			try! (
				File::open (
					path));

		// read first iv

		let mut initialisation_vector: Vec <u8> =
			vec! [0; IV_SIZE];

		try! (
			file.read_exact (
				& mut initialisation_vector));

		// setup decryptor

		let decryptor =
			CbcDecryptor::new (
				AesSafe128Decryptor::new (
					& encryption_key),
				PkcsPadding,
				initialisation_vector);

		// return

		Ok (CryptoReader {

			input: file,
			eof: false,

			decryptor: decryptor,

			ciphertext_buffer: [0u8; BUFFER_SIZE],
			ciphertext_start: 0,
			ciphertext_end: 0,

			plaintext_buffer: [0u8; BUFFER_SIZE],
			plaintext_start: 0,
			plaintext_end: 0,

		})

	}

	fn read_and_decrypt (
		& mut self,
	) -> io::Result <()> {

		assert! (
			! self.eof);

		assert! (
			self.plaintext_start == self.plaintext_end);

		self.plaintext_start = 0;
		self.plaintext_end = 0;

		while ! self.eof {

			// read in some more data

			if self.ciphertext_start == self.ciphertext_end {

				self.ciphertext_start = 0;

				self.ciphertext_end =
					try! (
						self.input.read (
							& mut self.ciphertext_buffer));

				if self.ciphertext_end == 0 {
					self.eof = true;
				}

			}

			// decrypt the data in the buffer

			let mut read_buffer =
				RefReadBuffer::new (
					& mut self.ciphertext_buffer [
						self.ciphertext_start ..
						self.ciphertext_end]);

			let mut write_buffer =
				RefWriteBuffer::new (
					& mut self.plaintext_buffer [
						self.plaintext_end .. ]);

			let decrypt_result =
				try! (

				self.decryptor.decrypt (
					& mut read_buffer,
					& mut write_buffer,
					self.eof,

				).map_err (
					|_|

					io::Error::new (
						io::ErrorKind::InvalidData,
						"Decryption failed")

				)

			);

			self.ciphertext_start +=
				read_buffer.position ();

			self.plaintext_end +=
				write_buffer.position ();

			match decrypt_result {

				BufferResult::BufferUnderflow =>
					continue,

				BufferResult::BufferOverflow =>
					break,

			};

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

		while ! (self.eof && self.plaintext_start == self.plaintext_end)
		&& total_bytes_read < buffer.len () {

			// read more data if appropriate

			if self.plaintext_start == self.plaintext_end {

				try! (
					self.read_and_decrypt ());

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

/// This implements the decryption and verification of a ZBackup encryption key,
/// from the `EncryptionKeyInfo` and the password file. This will normally be
/// called automatically when constructing a `Repository`, but it is made public
/// because it may be useful in some cases.

pub fn decrypt_key (
	password_file_path: & str,
	encryption_key: & proto::EncryptionKeyInfo,
) -> Result <Option <[u8; KEY_SIZE]>, String> {

	// read password from file

	let mut password_file =
		try! (
			io_result (
				File::open (
					password_file_path)));

	let mut password_string =
		String::new ();

	try! (
		io_result (
			password_file.read_to_string (
				& mut password_string)));

	// remove trailing newline

	if password_string.ends_with ("\n") {

		let password_length =
			password_string.len ();

		password_string.truncate (
			password_length - 1);

	}

	let password_bytes =
		password_string.as_bytes ();

	// derive password key from password

	let mut password_hmac =
		crypto::hmac::Hmac::new (
			crypto::sha1::Sha1::new (),
			password_bytes);

	let mut password_result =
		[0u8; KEY_SIZE];

	crypto::pbkdf2::pbkdf2 (
		& mut password_hmac,
		encryption_key.get_salt (),
		encryption_key.get_rounds (),
		& mut password_result);

	// decrypt actual key using password key

	let key_decryptor =
		crypto::aessafe::AesSafe128Decryptor::new (
			& password_result);

	let mut key_result =
		[0u8; KEY_SIZE];

	key_decryptor.decrypt_block (
		& encryption_key.get_encrypted_key (),
		& mut key_result);

	// derive check result to verify password

	let mut check_hmac =
		crypto::hmac::Hmac::new (
			crypto::sha1::Sha1::new (),
			& key_result);

	check_hmac.input (
		encryption_key.get_key_check_input ());

	let mut check_result =
		[0u8; HMAC_SIZE];

	check_hmac.raw_result (
		& mut check_result);

	// return

	if check_result == encryption_key.get_key_check_hmac () {

		Ok (Some (
			key_result
		))

	} else {

		Ok (None)

	}

}

// ex: noet ts=4 filetype=rs
