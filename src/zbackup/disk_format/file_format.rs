use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use rand;
use rand::Rng;

use misc::*;
use zbackup::crypto::*;
use zbackup::data::*;
use zbackup::disk_format::*;

#[ inline ]
pub fn file_open_with_crypto_and_adler <
	PathRef: AsRef <Path>
> (
	path: PathRef,
	encryption_key: Option <EncryptionKey>,
) -> io::Result <AdlerRead <Box <BufRead>>> {

	file_open_with_crypto_and_adler_impl (
		path.as_ref (),
		encryption_key,
	)

}

fn file_open_with_crypto_and_adler_impl (
	path: & Path,
	encryption_key: Option <EncryptionKey>,
) -> io::Result <AdlerRead <Box <BufRead>>> {

	Ok (match encryption_key {

		Some (encryption_key) => {

			let mut crypto_reader =
				CryptoReader::open (
					path,
					encryption_key,
				) ?;

			let mut initialisation_vector =
				[0u8; IV_SIZE];

			crypto_reader.read_exact (
				& mut initialisation_vector,
			) ?;

			let crypto_buf_reader: Box <BufRead> =
				Box::new (
					BufReader::new (
						crypto_reader));

			let mut adler_read =
				AdlerRead::new (
					crypto_buf_reader);

			adler_read.update (
				& initialisation_vector);

			adler_read

		},

		None => {

			let file =
				File::open (
					path,
				) ?;

			let file_buf_reader: Box <BufRead> =
				Box::new (
					BufReader::new (
						file));

			let adler_read =
				AdlerRead::new (
					file_buf_reader);

			adler_read

		},

	})

}

pub fn writer_wrap_with_crypto_and_adler <'a> (
	target: & 'a mut Write,
	encryption_key: Option <EncryptionKey>,
) -> Result <AdlerWriter <Box <CloseableWrite + 'a>>, io::Error> {

	Ok (match encryption_key {

		Some (encryption_key) => {

			let mut crypto_writer: Box <CloseableWrite + 'a> =
				Box::new (
					CryptoWriter::wrap (
						target,
						encryption_key,
					) ?
				);

			let initialisation_vector: Vec <u8> =
				rand::thread_rng ()
					.gen_iter::<u8> ()
					.take (IV_SIZE)
					.collect ();

			crypto_writer.write (
				& initialisation_vector,
			) ?;

			let mut adler_write =
				AdlerWriter::new (
					crypto_writer);

			adler_write.update (
				& initialisation_vector);

			adler_write

		},

		None =>
			AdlerWriter::new (
				Box::new (
					CloseableWriter::wrap (
						target))),

	})

}

// ex: noet ts=4 filetype=rust
