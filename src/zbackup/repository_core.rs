use std::path::Path;
use std::path::PathBuf;

use output::Output;

use rustc_serialize::hex::ToHex;

use zbackup::crypto::*;
use zbackup::data::*;
use zbackup::disk_format::*;

pub struct RepositoryCore {
	storage_info: DiskStorageInfo,
	path: PathBuf,
	encryption_key: Option <EncryptionKey>,
}

impl RepositoryCore {

	#[ inline ]
	pub fn open <
		RepositoryPath: AsRef <Path>,
		PasswordFilePath: AsRef <Path>,
	> (
		output: & Output,
		repository_path: RepositoryPath,
		password_file_path: Option <PasswordFilePath>,
	) -> Result <RepositoryCore, String> {

		Self::open_impl (
			output,
			repository_path.as_ref (),
			password_file_path.as_ref ().map (|value| value.as_ref ()),
		)

	}

	fn open_impl (
		output: & Output,
		repository_path: & Path,
		password_file_path: Option <& Path>,
	) -> Result <RepositoryCore, String> {

		let output_job =
			output_job_start! (
				output,
				"Opening repository {}",
				repository_path.to_string_lossy ());

		let storage_info =
			storage_info_read (
				repository_path.join (
					"info"),
			) ?;

		// decrypt encryption key with password

		let encryption_key =
			if storage_info.has_encryption_key () {

			if password_file_path.is_none () {

				output_job.remove ();

				return Err (
					"Required password file not provided".to_string ());

			}

			match (
				decrypt_key (
					password_file_path.unwrap (),
					storage_info.encryption_key (),
				) ?
			) {

				Some (key) =>
					Some (key),

				None => {

					output_job.remove ();

					return Err (
						"Incorrect password".to_string ());

				},

			}

		} else {

			if password_file_path.is_some () {

				output_job.remove ();

				return Err (
					"Unnecessary password file provided".to_string ());

			}

			None

		};

		output_job.complete ();

		Ok (RepositoryCore {
			storage_info: storage_info,
			path: repository_path.to_owned (),
			encryption_key: encryption_key,
		})

	}

	/// Accessor method for the storage info

	#[ inline ]
	pub fn storage_info (& self) -> & DiskStorageInfo {
		& self.storage_info
	}

	/// Accessor method for the path

	#[ inline ]
	pub fn path (& self) -> & Path {
		& self.path
	}

	/// Accessor method for the encryption key

	#[ inline ]
	pub fn encryption_key (& self) -> Option <EncryptionKey> {
		self.encryption_key
	}

	/// Convenience function to return the filesystem path for a backup.

	#[ inline ]
	pub fn backup_path (
		& self,
		backup_name: & str,
	) -> PathBuf {

		if backup_name.chars ().next ().unwrap () == '/' {

			self.backup_path (
				& backup_name [1 .. ])

		} else {

			self.path
				.join ("backups")
				.join (backup_name)

		}

	}

	/// Convenience function to return the filesystem path for a bundle id.

	#[ inline ]
	pub fn bundle_path (
		& self,
		bundle_id: BundleId,
	) -> PathBuf {

		self.path
			.join ("bundles")
			.join (bundle_id.bytes () [0 .. 1].to_hex ())
			.join (bundle_id.to_string ())

	}

	/// Convenience function to return the filesystem path for an index id.

	#[ inline ]
	pub fn index_path (
		& self,
		index_id: IndexId,
	) -> PathBuf {

		self.path
			.join ("index")
			.join (index_id.to_string ())

	}

}

// ex: noet ts=4 filetype=rust
