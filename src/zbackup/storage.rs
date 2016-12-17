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
	filesystem_items: LruCache <String, Arc <FilesystemItem>>,
}

struct FilesystemItem {
	storage_manager: Arc <StorageManagerData>,
	key: String,
	compressed: bool,
	uncompressed_size: usize,
	stored_size: usize,
}

impl StorageManager {

	pub fn new <PathRef: AsRef <Path>> (
		path_ref: PathRef,
		cpu_pool: CpuPool,
		uncompressed_memory_cache_size: usize,
		compressed_memory_cache_size: usize,
		filesystem_cache_size: usize,
	) -> Result <StorageManager, String> {

		// get string from path

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

		// try and create filesystem cache path

		try! (

			fs::create_dir_all (
				& path,
			).map_err (
				|error|

				format! (
					"Error creating filesystem cache path: {}: {}",
					& path,
					error.description ())

			)

		);

		// check we can access filesystem cache path

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
					& path));

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

		if ! self_state.uncompressed_memory_items.contains_key (
			& key,
		) {

			self_state.uncompressed_memory_items.insert (
				key.deref ().to_owned (),
				uncompressed_data.clone ());

		}

		// check if it is compressed

		let in_compressed_memory_cache =
			self_state.compressed_memory_items.contains_key (
				& key);

		let in_filesystem_cache =
			self_state.filesystem_items.contains_key (
				& key);

		if (
			! in_compressed_memory_cache
			|| ! in_filesystem_cache
		) {

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

			if ! in_compressed_memory_cache {

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

			}

			// write out to the filesystem

			if ! in_filesystem_cache {

				let mut output =
					File::create (
						& entry_path,
					).unwrap_or_else (
						|error|

						panic! (
							"Unable to create {}: {}",
							& entry_path,
							error.description ())

					);

				output.write (
					& stored_data,
				).unwrap_or_else (
					|error|

					panic! (
						"Error writing to {}: {}",
						entry_path,
						error.description ())

				);

				// create and store the filesystem item in the index

				let filesystem_item =
					FilesystemItem {

					storage_manager: self.data.clone (),
					key: key.deref ().to_owned (),
					compressed: compressed,
					stored_size: stored_data.len (),
					uncompressed_size: uncompressed_data.len (),

				};

				self_state.filesystem_items.insert (
					key.deref ().to_owned (),
					Arc::new (filesystem_item));

			}

		}

		// return

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
				return Some (
					futures::done (
						Ok (item_data.clone ())
					).boxed (),
				),

			None =>
				(),
		};

		self.get_compressed (
			self_state.deref_mut (),
			key)

	}

	fn get_compressed (
		& self,
		self_state: & mut StorageManagerState,
		key: & str,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

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
				return Some ({

				let self_clone =
					self.clone ();

				let compressed_data =
					compressed_data.clone ();

				let key =
					key.to_owned ();

				self.cpu_pool.spawn_fn (
					move || {

					let uncompressed_data =
						try! (

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

					);

					let mut self_state =
						self_clone.state.lock ().unwrap ();

					self_state.uncompressed_memory_items.insert (
						key,
						uncompressed_data.clone ());

					Ok (uncompressed_data)

				}).boxed ()

			}),

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

		self.get_filesystem (
			self_state,
			key)

	}

	fn get_filesystem (
		& self,
		self_state: & mut StorageManagerState,
		key: & str,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

		match (

			self_state.filesystem_items.get_mut (
				key,
			)

		) {

			Some (filesystem_item) => {

				let self_clone =
					self.clone ();

				let filesystem_item =
					filesystem_item.clone ();

				let key =
					key.to_owned ();

				Some (self.cpu_pool.spawn_fn (
					move || {

					let (uncompressed_data, compressed_data) =
						try! (
							filesystem_item.get ());

					let mut self_state =
						self_clone.state.lock ().unwrap ();

					if compressed_data.is_some () {

						self_state.compressed_memory_items.insert (
							key.clone (),
							MemoryCacheItem::Compressed (
								compressed_data.unwrap (),
								uncompressed_data.len ()));

						self_state.uncompressed_memory_items.insert (
							key,
							uncompressed_data.clone ());

					} else {

						self_state.compressed_memory_items.insert (
							key,
							MemoryCacheItem::Uncompressed (
								uncompressed_data.clone ()));

					}

					Ok (uncompressed_data)

				}).boxed ())

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
	) -> Result <(Arc <Vec <u8>>, Option <Arc <Vec <u8>>>), String> {

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
			Vec::with_capacity (
				self.stored_size);

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

				(
					uncompressed_data,
					Some (Arc::new (stored_data)),
				)

			)

		} else {

			Ok (

				(
					Arc::new (stored_data),
					None,
				)

			)

		}

	}

}

impl Drop for FilesystemItem {

	fn drop (
		& mut self,
	) {

		fs::remove_file (
			self.path (),
		).unwrap_or_else (
			|error|

			panic! (
				"Error removing storage item {}: {}",
				self.key,
				error.description ())

		);

	}

}

// ex: noet ts=4 filetype=rust
