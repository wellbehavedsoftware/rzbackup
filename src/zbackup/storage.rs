#![ allow (unused_parens) ]

use futures;
use futures::BoxFuture;
use futures::Future;

use futures_cpupool::CpuPool;

use lru_cache::LruCache;

use minilzo;

use std::error::Error;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

#[ derive (Clone) ]
pub struct StorageManager {
	data: Arc <StorageManagerData>,
	state: Arc <Mutex <StorageManagerState>>,
	cpu_pool: CpuPool,
}

#[ derive (Clone) ]
struct StorageManagerData {
	path: String,
}

enum MemoryCacheItem {
	Compressed (Arc <Vec <u8>>, usize),
	Uncompressed (Arc <Vec <u8>>),
}

struct StorageManagerState {
	uncompressed_memory_items: LruCache <String, Arc <Vec <u8>>>,
	compressed_memory_items: LruCache <String, MemoryCacheItem>,
	filesystem_items: LruCache <String, FilesystemItem>,
}

struct FilesystemItem {
	storage_manager: Arc <StorageManagerData>,
	key: String,
	compressed: bool,
	uncompressed_size: usize,
}

impl StorageManager {

	pub fn new <PathRef: AsRef <Path>> (
		path_ref: PathRef,
		cpu_pool: CpuPool,
		uncompressed_memory_cache_size: usize,
		compressed_memory_cache_size: usize,
		filesystem_cache_size: usize,
	) -> Result <StorageManager, String> {

		let path =
			try! (

			path_ref.as_ref ().to_str (
			).ok_or_else (
				||

				format! (
					"Invalid path: {}",
					path_ref.as_ref ().to_string_lossy ())

			)

		).to_owned ();

		let metadata =
			try! (

			fs::metadata (
				& path,
			).map_err (
				|_|

				format! (
					"Unable to access {}",
					& path)

			)

		);

		if ! metadata.is_dir () {

			return Err (
				format! (
					"Not a directory: {}",
					path));

		}

		// TODO create a test file to check permissions

		Ok (StorageManager {

			data: Arc::new (
				StorageManagerData {

				path:
					path,

			}),

			state: Arc::new (
				Mutex::new (
					StorageManagerState {

				uncompressed_memory_items:
					LruCache::new (
						uncompressed_memory_cache_size),

				compressed_memory_items:
					LruCache::new (
						compressed_memory_cache_size),

				filesystem_items:
					LruCache::new (
						filesystem_cache_size),

			})),

			cpu_pool:
				cpu_pool,

		})

	}

	pub fn insert (
		& mut self,
		key: String,
		uncompressed_data: Arc <Vec <u8>>,
	) -> Result <(), String> {

		let entry_path =
			format! (
				"{}/{}",
				self.data.path,
				key);

		let mut self_state =
			self.state.lock ().unwrap ();

		// store in memory cache

		self_state.uncompressed_memory_items.insert (
			key.deref ().to_owned (),
			uncompressed_data.clone ());

		// try and compress the data

		let (compressed, stored_data) =
			match (

			minilzo::compress (
				& uncompressed_data)

		) {

			Ok (compressed_data) =>
				(true, Arc::new (compressed_data)),

			Err (minilzo::Error::NotCompressible) =>
				(false, uncompressed_data.clone ()),

			Err (error) =>
				return Err (
					format! (
						"Error during compression: {:?}",
						error)),

		};

		// store in compressed memory cache

		self_state.compressed_memory_items.insert (
			key.deref ().to_owned (),
			if compressed {

				MemoryCacheItem::Compressed (
					stored_data.clone (),
					uncompressed_data.len ())

			} else {

				MemoryCacheItem::Uncompressed (
					stored_data.clone ())

			}
		);

		// write out to the filesystem

		let mut output =
			try! (

			File::create (
				& entry_path,
			).map_err (
				|error|

				format! (
					"Unable to create {}: {}",
					& entry_path,
					error.description ())
			)

		);

		try! (

			output.write (
				& stored_data,
			).map_err (
				|error|

				format! (
					"Error writing to {}: {}",
					entry_path,
					error.description ())
			)

		);

		// create and store the filesystem item in the index

		let filesystem_item =
			FilesystemItem {

			storage_manager: self.data.clone (),
			key: key.deref ().to_owned (),
			compressed: compressed,
			uncompressed_size: uncompressed_data.len (),

		};

		self_state.filesystem_items.insert (
			key.deref ().to_owned (),
			filesystem_item);

		Ok (())

	}

	pub fn get (
		& self,
		key: & str,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

		let mut self_state =
			self.state.lock ().unwrap ();

		// try in uncompressed memory cache

		match (

			self_state.uncompressed_memory_items.get_mut (
				key,
			)

		) {

			Some (item_data) =>
{
stderr! ("-");
				return Some (
					futures::done (
						Ok (item_data.clone ())
					).boxed (),
//				),
)},

			None =>
				(),
		};

		// try in compressed memory cache

		match (

			self_state.compressed_memory_items.get_mut (
				key,
			)

		) {

			Some (
				& mut MemoryCacheItem::Compressed (
					ref compressed_data,
					uncompressed_size)
			) =>
{
stderr! ("x");
				return Some ({

				futures::done (

					minilzo::decompress (
						& compressed_data,
						uncompressed_size,
					).map (
						|uncompressed_data|

						Arc::new (
							uncompressed_data)

					).map_err (
						|error|

						format! (
							"Decompression failed: {:?}",
							error)

					)

				).boxed ()

//			}),
})},

			Some (
				& mut MemoryCacheItem::Uncompressed (
					ref uncompressed_data)
			) =>
				return Some (
					futures::done (
						Ok (uncompressed_data.clone ())
					).boxed (),
				),

			None =>
				(),

		};

		// try in compressed filesystem cache

		Self::get_filesystem (
			self_state.deref_mut (),
			key)

	}

	fn get_filesystem (
		self_state: & mut StorageManagerState,
		key: & str,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

stderr! ("O");

		match (

			self_state.filesystem_items.get_mut (
				key,
			)

		) {

			Some (filesystem_item) => {

				let uncompressed_data =
					match (

					filesystem_item.get ()

				) {

					Ok (data) =>
						data,

					Err (error) =>
						return Some (
							futures::failed (
								error,
							).boxed ()
						),

				};

				self_state.uncompressed_memory_items.insert (
					key.to_owned (),
					uncompressed_data.clone ());

				Some (futures::done (
					Ok (uncompressed_data)
				).boxed ())

			},

			None =>
				None,

		}

	}

}

impl FilesystemItem {

	fn path (
		& self,
	) -> String {

		format! (
			"{}/{}",
			self.storage_manager.path,
			self.key)

	}

	fn get (
		& self,
	) -> Result <Arc <Vec <u8>>, String> {

		let mut file =
			try! (

			File::open (
				self.path (),
			).map_err (
				|error|

				format! (
					"Error loading storage item {}: {}",
					self.key,
					error.description ())

			)

		);

		let mut stored_data =
			Vec::new ();

		try! (

			file.read_to_end (
				& mut stored_data,
			).map_err (
				|error|

				format! (
					"Error loading storage item {}: {}",
					self.key,
					error.description ())

			)

		);

		if self.compressed {

			let uncompressed_data =
				Arc::new (
					try! (

				minilzo::decompress (
					& stored_data,
					self.uncompressed_size,
				).map_err (
					|error|

					format! (
						"Error decompressing stored data: {:?}",
						error)

				)

			));

			Ok (
				uncompressed_data
			)

		} else {

			Ok (

				Arc::new (
					stored_data)

			)

		}

	}

}

impl Drop for FilesystemItem {

	fn drop (
		& mut self,
	) {

		match (

			fs::remove_file (
				self.path (),
			)

		) {

			Ok (_) => (),

			Err (error) => {

				stderrln! (
					"Error removing storage item {}: {}",
					self.key,
					error.description ());

			}

		}

	}

}

// ex: noet ts=4 filetype=rust
