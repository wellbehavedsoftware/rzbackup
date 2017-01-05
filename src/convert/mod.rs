// declare modules

pub mod utils;

mod balancebundles;
mod balanceindexes;
mod checkindexes;
mod gcbundles;
mod gcindexes;
mod rebuildindexes;

// import project dependencies

use ::misc::*;

// import own dependencies

pub use self::balancebundles::*;
pub use self::balanceindexes::*;
pub use self::checkindexes::*;
pub use self::gcbundles::*;
pub use self::gcindexes::*;
pub use self::rebuildindexes::*;

// commands

pub fn convert_command (
) -> Box <Command> {

	Box::new (
		ParentCommand::new (
			"convert",
			"Performs various operations on ZBackup repositories",
			vec! [
				balance_bundles_command (),
				balance_indexes_command (),
				check_indexes_command (),
				gc_bundles_command (),
				gc_indexes_command (),
				rebuild_indexes_command (),
			],
		)
	)

}

// ex: noet ts=4 filetype=rust
