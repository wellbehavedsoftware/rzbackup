#[ macro_use] 
mod stderr;

mod io;
mod protobuf;

pub use self::io::*;
pub use self::protobuf::*;
pub use self::stderr::*;

pub fn to_array (
	slice: & [u8],
) -> [u8; 24] {

	[
		slice [0],  slice [1],  slice [2],  slice [3],  slice [4],  slice [5],
		slice [6],  slice [7],  slice [8],  slice [9],  slice [10], slice [11],
		slice [12], slice [13], slice [14], slice [15], slice [16], slice [17],
		slice [18], slice [19], slice [20], slice [21], slice [22], slice [23],
	]

}

// ex: noet ts=4 filetype=rust
