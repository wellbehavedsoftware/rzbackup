use zbackup::disk_format::*;

pub struct DiskEncryptionKeyInfoRef <'a> {
	raw: & 'a protobuf_types::EncryptionKeyInfo,
}

impl <'a> DiskEncryptionKeyInfoRef <'a> {

	#[ inline ]
	pub fn new (
		raw: & 'a protobuf_types::EncryptionKeyInfo,
	) -> DiskEncryptionKeyInfoRef <'a> {

		DiskEncryptionKeyInfoRef {
			raw: raw,
		}

	}

	#[ inline ]
	pub fn salt (& self) -> & [u8] {
		self.raw.get_salt ()
	}

	#[ inline ]
	pub fn rounds (& self) -> u32 {
		self.raw.get_rounds ()
	}

	#[ inline ]
	pub fn encrypted_key (& self) -> & [u8] {
		self.raw.get_encrypted_key ()
	}

	#[ inline ]
	pub fn key_check_input (& self) -> & [u8] {
		self.raw.get_key_check_input ()
	}

	#[ inline ]
	pub fn key_check_hmac (& self) -> & [u8] {
		self.raw.get_key_check_hmac ()
	}

}

/*
pub struct EncryptionKeyInfo {
    // message fields
    salt: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rounds: ::std::option::Option<u32>,
    encrypted_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key_check_input: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key_check_hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}
*/

// ex: noet ts=4 filetype=rust
