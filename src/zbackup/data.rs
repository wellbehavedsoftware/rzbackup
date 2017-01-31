use std::fmt;
use std::sync::Arc;

use rand;
use rand::Rng;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;

use zbackup::disk_format::*;

/// The size of a ZBackup encryption key, in bytes
pub const KEY_SIZE: usize = 16;

/// The size of a ZBackup HMAC, in bytes
pub const HMAC_SIZE: usize = 20;

/// The size of a ZBackup initialisation vector, in bytes
pub const IV_SIZE: usize = 16;

/// The size of the buffers used when reading and decompressing ZBackup data
pub const BUFFER_SIZE: usize = 0x2000;

#[ doc (hidden) ]
pub const WORK_JOBS_TOTAL: usize = 0;

#[ doc (hidden) ]
pub const WORK_JOBS_BATCH: usize = 0;

/// A ZBackup bundle ID

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BundleId {
	bytes: [u8; 24],
}

impl BundleId {

	#[ inline ]
	pub fn random () -> BundleId {
		BundleId {
			bytes: random_array_24 (),
		}
	}

	#[ inline ]
	pub fn from_slice (
		bytes: & [u8],
	) -> Result <BundleId, String> {

		Ok (BundleId {
			bytes: to_array_24 (
				bytes.iter ().map (|& byte| byte),
			) ?,
		})

	}

	#[ inline ]
	pub fn parse <
		BundleIdString: AsRef <str>,
	> (
		bundle_id_string: BundleIdString,
	) -> Result <BundleId, String> {

		Ok (BundleId {
			bytes: parse_array_24 (
				"bundle id",
				bundle_id_string.as_ref (),
			) ?,
		})

	}

	#[ inline ]
	pub fn bytes (& self) -> & [u8] {
		& self.bytes
	}

	#[ inline ]
	pub fn into_vec (self) -> Vec <u8> {
		self.bytes.to_vec ()
	}

	#[ inline ]
	pub fn to_string (& self) -> String {
		self.bytes.to_hex ()
	}

}

impl fmt::Display for BundleId {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> Result <(), fmt::Error> {

		formatter.write_str (
			& self.bytes.to_hex (),
		)

	}

}

/// A ZBackup chunk ID

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct ChunkId {
	bytes: [u8; 24],
}

impl ChunkId {

	#[ inline ]
	pub fn random () -> ChunkId {
		ChunkId {
			bytes: random_array_24 (),
		}
	}

	#[ inline ]
	pub fn from_slice (
		bytes: & [u8],
	) -> Result <ChunkId, String> {

		Ok (ChunkId {
			bytes: to_array_24 (
				bytes.iter ().map (|& byte| byte),
			) ?,
		})

	}

	#[ inline ]
	pub fn parse <
		ChunkIdString: AsRef <str>,
	> (
		chunk_id_string: ChunkIdString,
	) -> Result <ChunkId, String> {

		Ok (ChunkId {
			bytes: parse_array_24 (
				"chunk id",
				chunk_id_string.as_ref (),
			) ?,
		})

	}

	#[ inline ]
	pub fn bytes (& self) -> & [u8] {
		& self.bytes
	}

	#[ inline ]
	pub fn into_vec (self) -> Vec <u8> {
		self.bytes.to_vec ()
	}

	#[ inline ]
	pub fn to_string (& self) -> String {
		self.bytes.to_hex ()
	}

}

impl From <[u8; 24]> for ChunkId {

	fn from (
		bytes: [u8; 24],
	) -> ChunkId {

		ChunkId {
			bytes: bytes,
		}

	}

}

impl fmt::Display for ChunkId {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> Result <(), fmt::Error> {

		formatter.write_str (
			& self.bytes.to_hex (),
		)

	}

}

/// A ZBackup index ID

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct IndexId {
	bytes: [u8; 24],
}

impl IndexId {

	#[ inline ]
	pub fn random () -> IndexId {
		IndexId {
			bytes: random_array_24 (),
		}
	}

	#[ inline ]
	pub fn parse <
		IndexIdString: AsRef <str>,
	> (
		index_id_string: IndexIdString,
	) -> Result <IndexId, String> {

		Ok (IndexId {
			bytes: parse_array_24 (
				"index id",
				index_id_string.as_ref (),
			) ?,
		} )

	}

	#[ inline ]
	pub fn into_vec (self) -> Vec <u8> {
		self.bytes.to_vec ()
	}

	#[ inline ]
	pub fn to_string (& self) -> String {
		self.bytes.to_hex ()
	}

}

impl From <[u8; 24]> for IndexId {

	fn from (
		bytes: [u8; 24],
	) -> IndexId {

		IndexId {
			bytes: bytes,
		}

	}

}

impl fmt::Display for IndexId {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> Result <(), fmt::Error> {

		formatter.write_str (
			& self.bytes.to_hex (),
		)

	}

}

pub fn to_array_24 <
	ItemsIntoIter: IntoIterator <Item = u8>,
> (
	items_into_iter: ItemsIntoIter,
) -> Result <[u8; 24], String> {

	let mut items_iter =
		items_into_iter.into_iter ();

	let array = [
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
		items_iter.next ().ok_or_else (|| "Not enough data".to_string ()) ?,
	];

	if items_iter.next ().is_some () {
		return Err ("Too much data".to_string ());
	}

	Ok (array)

}

pub fn parse_array_24 (
	name: & str,
	string: & str,
) -> Result <[u8; 24], String> {

	if string.len () != 48 {

		Err (
			format! (
				"Invalid {}: {}",
				name,
				string))

	} else {

		Ok (to_array_24 (
			string.from_hex ().map_err (|_|

				format! (
					"Invalid {}: {}",
					name,
					string)

			) ?.into_iter (),
		) ?)

	}

}

pub fn random_array_24 () -> [u8; 24] {

	to_array_24 (
		rand::thread_rng ()
			.gen_iter::<u8> ()
			.take (24)
	).unwrap ()

}

/// A ZBackup chunk's data

pub type ChunkData = Arc <Vec <u8>>;

/// A ZBackup encryption key

pub type EncryptionKey = [u8; KEY_SIZE];

/// A ZBackup index entry. This combined the header with the bundle info, since
/// generally both of these are needed to make use of the data.

pub struct RawIndexEntry {
	pub index_bundle_header: DiskIndexBundleHeader,
	pub bundle_info: DiskBundleInfo,
}

/// The default number of uncompressed memory cache entries. This number of
/// chunks will be kept in memory uncompressed, in an LRU cache.
pub const MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES: usize = 0x10;

/// The default number of uncompressed memory cache entries. This number of
/// chunks will be kept in memory after LZO-compression, in an LRU cache.
pub const MAX_COMPRESSED_MEMORY_CACHE_ENTRIES: usize = 0x100;

/// The default number of compressed filesystem cache entries. This number of
/// chunks will be stored in a temporary directory after LZO-compression.
pub const MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES: usize = 0x4000;

/// The default location for the filesystem cache.
pub const FILESYSTEM_CACHE_PATH: & 'static str = "/tmp/rzbackup-cache";

// ex: noet ts=4 filetype=rust
