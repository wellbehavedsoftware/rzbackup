//! RZBackup is a library, and a collection of binaries, implementing a partial
//! clone of [ZBackup](http://zbackup.org).
//!
//! The main class is `Repository`, which has static methods for opening and
//! accessing data from a ZBackup repository. The `restore` method will restore
//! a backup to a provided `Writer`.
//!
//! `Repository` implements `Clone` and is fully thread-safe. It performs
//! parallel decompression operations using a background thread pool and it has
//! a three-layer cache. The parameters for cache sizes and number of threads
//! are fully configurable.
//!
//! There is also a `RandomAccess` class which implements `Seek` and `Read`, and
//! can be constructed from a `Repository` and the name of a backup.

#![ allow (unused_parens) ]

#[ macro_use ]
extern crate lazy_static;

#[ macro_use ]
extern crate output;

extern crate adler32;
extern crate byteorder;
extern crate clap;
extern crate crypto as rust_crypto;
extern crate errno;
extern crate futures;
extern crate futures_cpupool;
extern crate libc;
extern crate lru_cache;
extern crate minilzo;
extern crate num_cpus;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;

#[ doc (hidden) ]
#[ macro_use ]
pub mod misc;

#[ doc (hidden) ]
pub mod client;

#[ doc (hidden) ]
pub mod commands;

#[ doc (hidden) ]
pub mod convert;

#[ doc (hidden) ]
pub mod server;

mod compress;
mod zbackup;

pub use ::misc::AtomicFileWriter;
pub use zbackup::crypto;
pub use zbackup::data::*;
pub use zbackup::disk_format;
pub use zbackup::metadata::*;
pub use zbackup::randaccess::RandomAccess;
pub use zbackup::repository::Repository as ZBackupRepository;
pub use zbackup::repository::RepositoryConfig as ZBackupRepositoryConfig;

pub use server::run_server;

// ex: noet ts=f filetype=rust
