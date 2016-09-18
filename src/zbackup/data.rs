use std::sync::Arc;

use zbackup::proto;

pub const KEY_SIZE: usize = 16;
pub const HMAC_SIZE: usize = 20;
pub const IV_SIZE: usize = 16;

pub const BUFFER_SIZE: usize = 0x10000;

pub const WORK_JOBS_TOTAL: usize = 0x1000;
pub const WORK_JOBS_BATCH: usize = 0x100;

pub type BundleId = [u8; 24];
pub type ChunkId = [u8; 24];
pub type IndexId = [u8; 24];

pub type ChunkData = Arc <Vec <u8>>;

pub type EncryptionKey = [u8; KEY_SIZE];

pub type IndexEntry = (
	proto::IndexBundleHeader,
	proto::BundleInfo,
);

// ex: noet ts=4 filetype=rust
