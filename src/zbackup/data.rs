use zbackup::proto;

pub const KEY_SIZE: usize = 16;
pub const HMAC_SIZE: usize = 20;
pub const IV_SIZE: usize = 16;
pub const BLOCK_SIZE: usize = 16;

pub const PAGE_SIZE: usize = 0x1000;

pub type IndexEntry = (
	proto::IndexBundleHeader,
	proto::BundleInfo,
);

// ex: noet ts=4 filetype=rust
