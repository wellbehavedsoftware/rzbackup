use std::io;
use std::io::Write;

use rust_crypto::aessafe::AesSafe128Encryptor;
use rust_crypto::buffer::BufferResult;
use rust_crypto::buffer::RefReadBuffer;
use rust_crypto::buffer::RefWriteBuffer;
use rust_crypto::buffer::WriteBuffer;
use rust_crypto::blockmodes::CbcEncryptor;
use rust_crypto::blockmodes::EncPadding;
use rust_crypto::blockmodes::PkcsPadding;
use rust_crypto::symmetriccipher::Encryptor;

use ::misc::*;
use ::zbackup::data::*;

type EncryptorType =
	CbcEncryptor <
		AesSafe128Encryptor,
		EncPadding <PkcsPadding>,
	>;

pub struct CryptoWriter <Target: Write> {

	target: Target,

	encryptor: EncryptorType,

	plaintext_eof: bool,

	ciphertext_buffer: [u8; BUFFER_SIZE],
	ciphertext_eof: bool,

}

impl <Target: Write> CryptoWriter <Target> {

	pub fn wrap (
		target: Target,
		encryption_key: [u8; KEY_SIZE],
	) -> io::Result <CryptoWriter <Target>> {

		// setup encryptor

		let encryptor =
			CbcEncryptor::new (
				AesSafe128Encryptor::new (
					& encryption_key),
				PkcsPadding,
				[0u8; KEY_SIZE].to_vec ());

		// return

		Ok (CryptoWriter {

			target: target,

			encryptor: encryptor,

			plaintext_eof: false,

			ciphertext_buffer: [0u8; BUFFER_SIZE],
			ciphertext_eof: false,

		})

	}

	fn encrypt_and_write (
		& mut self,
		plaintext_buffer: & [u8],
	) -> io::Result <()> {

		assert! (
			! self.ciphertext_eof);

		let mut read_buffer =
			RefReadBuffer::new (
				plaintext_buffer);

		loop {

			// encrypt data in the buffer

			let bytes_out;
			let encrypt_result;

			{

				let mut write_buffer =
					RefWriteBuffer::new (
						& mut self.ciphertext_buffer);

				encrypt_result =
					self.encryptor.encrypt (
						& mut read_buffer,
						& mut write_buffer,
						self.plaintext_eof,
					).map_err (
						|_|

						io::Error::new (
							io::ErrorKind::InvalidData,
							"Encryption failed")

					) ?;

				bytes_out =
					write_buffer.position ();

			}

			// write out encrypted data

			self.target.write (
				& self.ciphertext_buffer [
					0 .. bytes_out],
			) ?;

			// continue as appropriate

			match encrypt_result {

				BufferResult::BufferUnderflow =>
					break,

				BufferResult::BufferOverflow =>
					continue,

			};

		}

		if self.plaintext_eof {
			self.ciphertext_eof = true;
		}

		Ok (())

	}

}

impl <Target: Write> Write for CryptoWriter <Target> {

	fn write (
		& mut self,
		buffer: & [u8],
	) -> io::Result <usize> {

		if self.plaintext_eof {

			panic! (
				"Attempt to write to closed CryptoWriter");

		}

		self.encrypt_and_write (
			buffer,
		) ?;

		Ok (buffer.len ())

	}

	fn flush (
		& mut self,
	) -> Result <(), io::Error> {

		Ok (())

	}

}

impl <Target: Write> CloseableWrite for CryptoWriter <Target> {

	fn close (
		& mut self,
	) -> io::Result <()> {

		self.plaintext_eof = true;

		self.encrypt_and_write (
			& [0u8; 0],
		) ?;

		self.target.flush ()

	}

}

// ex: noet ts=4 filetype=rs
