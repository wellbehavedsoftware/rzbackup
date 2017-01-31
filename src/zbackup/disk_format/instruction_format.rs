use protobuf::stream::CodedInputStream;

use zbackup::data::*;
use zbackup::disk_format::*;

pub struct DiskBackupInstruction {
	raw: protobuf_types::BackupInstruction,
}

impl DiskBackupInstruction {

	#[ inline ]
	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskBackupInstruction, String> {

		Ok (DiskBackupInstruction {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"backup instruction"),
			) ?,
		})

	}

	#[ inline ]
	pub fn has_bytes_to_emit (& self) -> bool {
		self.raw.has_bytes_to_emit ()
	}

	#[ inline ]
	pub fn bytes_to_emit (& self) -> & [u8] {
		self.raw.get_bytes_to_emit ()
	}

	#[ inline ]
	pub fn has_chunk_to_emit (& self) -> bool {
		self.raw.has_chunk_to_emit ()
	}

	#[ inline ]
	pub fn chunk_to_emit (& self) -> ChunkId {
		ChunkId::from_slice (
			self.raw.get_chunk_to_emit ()
		).unwrap ()
	}

}

// ex: noet ts=4 filetype=rust
