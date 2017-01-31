use std::path::Path;

use protobuf::stream::CodedInputStream;

use misc::*;
use zbackup::data::*;
use zbackup::disk_format::*;
use zbackup::disk_format::protobuf_types as raw;

pub struct DiskBackupInfo {
	raw: raw::BackupInfo,
}

impl DiskBackupInfo {

	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskBackupInfo, String> {

		Ok (DiskBackupInfo {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"backup info"),
			) ?,
		})

	}

	pub fn sha256 (& self) -> [u8; 32] {
		to_array_32 (self.raw.get_sha256 ())
	}

	pub fn backup_data (& self) -> & [u8] {
		& self.raw.get_backup_data ()
	}

	pub fn iterations (& self) -> u32 {
		self.raw.get_iterations ()
	}

}

#[ inline ]
pub fn backup_read_path <
	BackupPath: AsRef <Path>,
> (
	backup_path: BackupPath,
	encryption_key: Option <EncryptionKey>,
) -> Result <DiskBackupInfo, String> {

	backup_read_path_impl (
		backup_path.as_ref (),
		encryption_key,
	)

}

pub fn backup_read_path_impl (
	backup_path: & Path,
	encryption_key: Option <[u8; KEY_SIZE]>,
) -> Result <DiskBackupInfo, String> {

	let backup_info: DiskBackupInfo;

	// open file

	let mut source =
		io_result_with_prefix (
			|| format! (
				"Error reading {}: ",
				backup_path.to_string_lossy ()),
			file_open_with_crypto_and_adler (
				backup_path,
				encryption_key),
		) ?;

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read file header

		let file_header: DiskFileHeader =
			DiskFileHeader::read (
				& mut coded_input_stream,
			) ?;

		if file_header.version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.version ());

		}

		// read backup info

		backup_info =
			DiskBackupInfo::read (
				& mut coded_input_stream,
			) ?;

	}

	// verify checksum

	adler_verify_hash_and_eof (
		|| format! (
			"Error reading {}: ",
			backup_path.to_string_lossy ()),
		source,
	) ?;

	// return

	Ok (backup_info)

}

// ex: noet ts=4 filetype=rust
