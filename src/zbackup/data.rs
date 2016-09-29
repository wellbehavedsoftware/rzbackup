use std::sync::Arc;

use zbackup::proto;

/// The size of a ZBackup encryption key, in bytes
pub const KEY_SIZE: usize = 16;

/// The size of a ZBackup HMAC, in bytes
pub const HMAC_SIZE: usize = 20;

/// The size of a ZBackup initialisation vector, in bytes
pub const IV_SIZE: usize = 16;

/// The size of the buffers used when reading and decompressing ZBackup data
pub const BUFFER_SIZE: usize = 0x2000;

/// The default number of total jobs to queue up during a restore operation.
/// Since many chunks can be stored in a single bundle, and we can only use a
/// single thread to decompress one bundle, this needs to be high enough to
/// guarantee that we will always have enough jobs in the queue to make use of
/// all of the threads available.
pub const WORK_JOBS_TOTAL: usize = 0x200;

/// The batch size for adding and removing jobs from the work queue. By doing
/// this in batches, we can avoid a certain amount of unnecessary locking.
pub const WORK_JOBS_BATCH: usize = 0x20;

/// A ZBackup bundle ID
pub type BundleId = [u8; 24];

/// A ZBackup chunk ID
pub type ChunkId = [u8; 24];

/// A ZBackup index ID
pub type IndexId = [u8; 24];

/// A ZBackup chunk's data
pub type ChunkData = Arc <Vec <u8>>;

/// A ZBackup encryption key
pub type EncryptionKey = [u8; KEY_SIZE];

/// A ZBackup index entry. This combined the header with the bundle info, since
/// generally both of these are needed to make use of the data.
pub type IndexEntry = (
	proto::IndexBundleHeader,
	proto::BundleInfo,
);

/// The default number of uncompressed memory cache entries. This number of
/// chunks will be kept in memory uncompressed, in an LRU cache.
pub const MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES: usize = 0x100;

/// The default number of uncompressed memory cache entries. This number of
/// chunks will be kept in memory after LZO-compression, in an LRU cache.
pub const MAX_COMPRESSED_MEMORY_CACHE_ENTRIES: usize = 0x800;

/// The default number of compressed filesystem cache entries. This number of
/// chunks will be stored in a temporary directory after LZO-compression.
pub const MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES: usize = 0x4000;

/// The default location for the filesystem cache.
pub const FILESYSTEM_CACHE_PATH: & 'static str = "/tmp/rzbackup-cache";

// ex: noet ts=4 filetype=rust
