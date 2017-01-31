use std::fs::File;
use std::io::Read;
use std::path::Path;

use rust_crypto;
use rust_crypto::mac::Mac;
use rust_crypto::symmetriccipher::BlockDecryptor;

use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::disk_format::*;

/// This implements the decryption and verification of a ZBackup encryption key,
/// from the `EncryptionKeyInfo` and the password file. This will normally be
/// called automatically when constructing a `Repository`, but it is made public
/// because it may be useful in some cases.

#[ inline ]
pub fn decrypt_key <
	PasswordFilePath: AsRef <Path>,
> (
	password_file_path: PasswordFilePath,
	encryption_key: DiskEncryptionKeyInfoRef,
) -> Result <Option <EncryptionKey>, String> {

	decrypt_key_impl (
		password_file_path.as_ref (),
		encryption_key,
	)

}

fn decrypt_key_impl (
	password_file_path: & Path,
	encryption_key_info: DiskEncryptionKeyInfoRef,
) -> Result <Option <EncryptionKey>, String> {

	// read password from file

	let mut password_file =
		io_result (
			File::open (
				password_file_path),
		) ?;

	let mut password_string =
		String::new ();

	io_result (
		password_file.read_to_string (
			& mut password_string),
	) ?;

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
		rust_crypto::hmac::Hmac::new (
			rust_crypto::sha1::Sha1::new (),
			password_bytes);

	let mut password_result =
		[0u8; KEY_SIZE];

	rust_crypto::pbkdf2::pbkdf2 (
		& mut password_hmac,
		encryption_key_info.salt (),
		encryption_key_info.rounds (),
		& mut password_result);

	// decrypt actual key using password key

	let key_decryptor =
		rust_crypto::aessafe::AesSafe128Decryptor::new (
			& password_result);

	let mut key_result =
		[0u8; KEY_SIZE];

	key_decryptor.decrypt_block (
		& encryption_key_info.encrypted_key (),
		& mut key_result);

	// derive check result to verify password

	let mut check_hmac =
		rust_crypto::hmac::Hmac::new (
			rust_crypto::sha1::Sha1::new (),
			& key_result);

	check_hmac.input (
		encryption_key_info.key_check_input ());

	let mut check_result =
		[0u8; HMAC_SIZE];

	check_hmac.raw_result (
		& mut check_result);

	// return

	if check_result == encryption_key_info.key_check_hmac () {

		Ok (Some (
			key_result
		))

	} else {

		Ok (None)

	}

}

// ex: noet ts=4 filetype=rust
