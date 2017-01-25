// declare modules

mod client;
mod clientexit;
mod clientreindex;
mod clientrestore;
mod clientstatus;

// import project dependencies

use ::misc::*;

// import own dependencies

pub use self::client::*;
pub use self::clientexit::*;
pub use self::clientreindex::*;
pub use self::clientrestore::*;
pub use self::clientstatus::*;

// commands

pub fn client_command (
) -> Box <Command> {

	Box::new (
		ParentCommand::new (
			"client",
			"Connects to a server and performs various operations",
			vec! [
				client_exit_command (),
				client_reindex_command (),
				client_restore_command (),
				client_status_command (),
			],
		)
	)

}

// ex: noet ts=4 filetype=rust
