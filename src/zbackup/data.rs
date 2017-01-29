use std::sync::Arc;

use rand;
use rand::Rng;

use rustc_serialize::hex::FromHex;

use ::misc::*;
use zbackup::proto;

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
pub type BundleId = [u8; 24];

pub fn bundle_id_generate () -> BundleId {
	generic_id_generate ()
}

pub fn bundle_id_parse (string: & str) -> Result <BundleId, String> {
	generic_id_parse (string, "bundle id")
}

/// A ZBackup chunk ID
pub type ChunkId = [u8; 24];

pub fn chunk_id_generate () -> ChunkId {
	generic_id_generate ()
}

pub fn chunk_id_parse (string: & str) -> Result <ChunkId, String> {
	generic_id_parse (string, "chunk id")
}

/// A ZBackup index ID
pub type IndexId = [u8; 24];

pub fn index_id_generate () -> IndexId {
	generic_id_generate ()
}

pub fn index_id_parse (string: & str) -> Result <IndexId, String> {
	generic_id_parse (string, "index id")
}

fn generic_id_generate (
) -> [u8; 24] {

	let id_bytes: Vec <u8> =
		rand::thread_rng ()
			.gen_iter::<u8> ()
			.take (24)
			.collect ();

	to_array_24 (
		& id_bytes)

}

fn generic_id_parse (
	string: & str,
	type_name: & str,
) -> Result <[u8; 24], String> {

	if string.len () != 48 {

		return Err (
			format! (
				"Invalid {}: {}",
				type_name,
				string));

	}

	string.from_hex ().map (
		|hex_vector|

		to_array_24 (
			& hex_vector)

	).or_else (
		|_hex_error|

		Err (
			format! (
				"Invalid {}: {}",
				type_name,
				string))

	)

}

/// A ZBackup chunk's data
pub type ChunkData = Arc <Vec <u8>>;

/// A ZBackup encryption key
pub type EncryptionKey = [u8; KEY_SIZE];

/// A ZBackup index entry. This combined the header with the bundle info, since
/// generally both of these are needed to make use of the data.
pub type RawIndexEntry = (
	proto::IndexBundleHeader,
	proto::BundleInfo,
);

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
