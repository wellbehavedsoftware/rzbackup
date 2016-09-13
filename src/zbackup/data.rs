use zbackup::proto;

pub type IndexEntry = (
	proto::IndexBundleHeader,
	proto::BundleInfo,
);

// ex: noet ts=4 filetype=rust
