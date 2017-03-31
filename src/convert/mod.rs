// declare modules

#[ macro_use ]
pub mod utils;

mod balance_bundles;
mod balance_indexes;
mod check_backups;
mod check_bundles;
mod check_indexes;
mod gc_bundles;
mod gc_indexes;
mod rebuild_indexes;

// import project dependencies

use ::misc::*;

// import own dependencies

pub use self::balance_bundles::*;
pub use self::balance_indexes::*;
pub use self::check_backups::*;
pub use self::check_bundles::*;
pub use self::check_indexes::*;
pub use self::gc_bundles::*;
pub use self::gc_indexes::*;
pub use self::rebuild_indexes::*;

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
				check_backups_command (),
				check_bundles_command (),
				check_indexes_command (),
				gc_bundles_command (),
				gc_indexes_command (),
				rebuild_indexes_command (),
			],
		)
	)

}

// ex: noet ts=4 filetype=rust
