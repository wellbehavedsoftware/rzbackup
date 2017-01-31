use protobuf::stream::CodedInputStream;
use protobuf::stream::CodedOutputStream;

use zbackup::disk_format::*;

pub struct DiskFileHeader {
	raw: protobuf_types::FileHeader,
}

impl DiskFileHeader {

	#[ inline ]
	pub fn new (
		version: u32,
	) -> DiskFileHeader {

		let mut raw =
			protobuf_types::FileHeader::new ();

		raw.set_version (
			version);

		DiskFileHeader {
			raw: raw,
		}

	}

	#[ inline ]
	pub fn read (
		coded_input_stream: & mut CodedInputStream,
	) -> Result <DiskFileHeader, String> {

		Ok (DiskFileHeader {
			raw: protobuf_message_read (
				coded_input_stream,
				|| format! (
					"file header"),
			) ?,
		})

	}

	#[ inline ]
	pub fn write (
		& self,
		coded_output_stream: & mut CodedOutputStream,
	) -> Result <(), String> {

		protobuf_message_write (
			|| "file header".to_string (),
			coded_output_stream,
			& self.raw,
		)

	}

	#[ inline ]
	pub fn version (& self) -> u32 {
		self.raw.get_version ()
	}

}

// ex: noet ts=4 filetype=rust
