use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use protobuf::stream::CodedInputStream;

use misc::*;
use zbackup::disk_format::*;

#[ derive (Clone, Debug) ]
pub struct DiskStorageInfo {
	raw: protobuf_types::StorageInfo,
}

impl DiskStorageInfo {

	#[ inline ]
	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskStorageInfo, String> {

		Ok (DiskStorageInfo {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"storage info"),
			) ?,
		})

	}

	#[ inline ]
	pub fn has_encryption_key (& self) -> bool {
		self.raw.has_encryption_key ()
	}

	#[ inline ]
	pub fn encryption_key (& self) -> DiskEncryptionKeyInfoRef {
		DiskEncryptionKeyInfoRef::new (
			self.raw.get_encryption_key (),
		)
	}

}

#[ inline ]
pub fn storage_info_read <
	InfoPath: AsRef <Path>,
> (
	info_path: InfoPath,
) -> Result <DiskStorageInfo, String> {

	storage_info_read_impl (
		info_path.as_ref (),
	)

}

pub fn storage_info_read_impl (
	info_path: & Path,
) -> Result <DiskStorageInfo, String> {

	let storage_info: DiskStorageInfo;

	// open file

	let source =
		io_result_with_prefix (
			|| format! (
				"Error opening {}: ",
				info_path.to_string_lossy ()),
			File::open (
				info_path),
		) ?;

	let mut source =
		AdlerRead::new (
			Box::new (
				BufReader::new (
					source)));

	{

		let mut coded_input_stream =
			CodedInputStream::from_buffered_reader (
				& mut source);

		// read file header

		let file_header =
			DiskFileHeader::read (
				& mut coded_input_stream,
			) ?;

		if file_header.version () != 1 {

			panic! (
				"Unsupported backup version {}",
				file_header.version ());

		}

		// read storage info

		storage_info =
			DiskStorageInfo::read (
				& mut coded_input_stream,
			) ?;

	}

	// verify checksum

	adler_verify_hash_and_eof (
		|| format! (
			"Error reading {}: ",
			info_path.to_string_lossy ()),
		source,
	) ?;

	// return

	Ok (storage_info)

}

// ex: noet ts=4 filetype=rust
