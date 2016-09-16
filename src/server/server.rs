use std::error::Error;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub use server::handler::handle_client;

use ::Repository;

use ::misc::*;

pub fn run_server (
	repository: Repository,
	bind_address: & str,
) -> Result <(), String> {

	let listener =
		try! (
			io_result (
				TcpListener::bind (
					bind_address)));

	let repository =
		Arc::new (
			Mutex::new (
				repository));
	
	for stream
	in listener.incoming () {

		match stream {

			Ok (stream) => {

				let repository_copy =
					repository.clone ();

				thread::spawn (
					move || {

						handle_client (
							repository_copy,
							stream)

					}
				);
			},

			Err (error) => {

				println! (
					"Connection failed: {}",
					error.description ());

			},

		}

	};

	Ok (())

}

// ex: noet ts=4 filetype=rust
