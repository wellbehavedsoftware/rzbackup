use zbackup::proto;

pub const KEY_SIZE: usize =
	16;

pub type IndexEntry = (
	proto::IndexBundleHeader,
	proto::BundleInfo,
);

// ex: noet ts=4 filetype=rust
