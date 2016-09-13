extern crate libc;
extern crate protobuf;
extern crate rustc_serialize;

#[ macro_use ]
mod misc;

mod compress;
mod server;
mod zbackup;

pub use zbackup::randaccess::RandomAccess;
pub use zbackup::repo::Repository;

pub use server::run_server;

// ex: noet ts=f filetype=rust
