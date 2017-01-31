pub mod args;

#[ macro_use ]
mod command;

mod atomic_file_writer;
mod cloning_shared_future;
mod error;
mod io;
mod protobuf;

pub use self::atomic_file_writer::*;
pub use self::cloning_shared_future::*;
pub use self::command::*;
pub use self::error::*;
pub use self::io::*;
pub use self::protobuf::*;
//pub use self::task_queue::*;

pub fn to_array_16 (
	slice: & [u8],
) -> [u8; 16] {

	[
		slice [0],  slice [1],  slice [2],  slice [3],
		slice [4],  slice [5],  slice [6],  slice [7],
		slice [8],  slice [9],  slice [10], slice [11],
		slice [12], slice [13], slice [14], slice [15],
	]

}

pub fn to_array_32 (
	slice: & [u8],
) -> [u8; 32] {

	[
		slice [0],  slice [1],  slice [2],  slice [3],
		slice [4],  slice [5],  slice [6],  slice [7],
		slice [8],  slice [9],  slice [10], slice [11],
		slice [12], slice [13], slice [14], slice [15],
		slice [16], slice [17], slice [18], slice [19],
		slice [20], slice [21], slice [22], slice [23],
		slice [24], slice [25], slice [26], slice [27],
		slice [28], slice [29], slice [30], slice [31],
	]

}

// ex: noet ts=4 filetype=rust
