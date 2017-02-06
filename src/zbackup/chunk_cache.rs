#![ allow (unused_parens) ]

use std::error::Error;
use std::hash::Hash;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use futures;
use futures::BoxFuture;
use futures::Future;

use futures_cpupool::CpuPool;

use lru_cache::LruCache;

use minilzo;

use output::Output;

use rand;
use rand::Rng;

/// The chunk cache provides a tiered cache for individual chunks. These are
/// stored in memory, compressed and uncompressed, and in the filesyste,
/// compressed. The compression is LZO for speed and the cache uses a simple
/// least-recently-used policy.
///
/// The disk cache is split into two, with one typically large area reserved for
/// chunks which have actually been used, and another smaller one for those
/// which have been uncompressed but have not been used. The large area for used
/// chunks greatly improves efficiency over time, while the smaller cache for
/// unused chunks improves decompression when the cache is cold or when
/// restoring new backups.

#[ derive (Clone) ]
pub struct ChunkCache <Key: ChunkCacheKey> {
	data: Arc <ChunkCacheData>,
	state: Arc <Mutex <ChunkCacheState <Key>>>,
	cpu_pool: CpuPool,
}

pub trait ChunkCacheKey: Clone + Eq + Hash + Send + Sync + 'static {
}

impl <Type> ChunkCacheKey for Type
where Type: Clone + Eq + Hash + Send + Sync + 'static {
}

#[ derive (Clone) ]
struct ChunkCacheData {
	path: PathBuf,
	debug: bool,
}

#[ derive (Clone) ]
enum MemoryCacheItem {
	Compressed (Arc <Vec <u8>>, usize),
	Uncompressed (Arc <Vec <u8>>),
}

struct ChunkCacheState <Key: ChunkCacheKey> {

	uncompressed_memory_items: LruCache <Key, Arc <Vec <u8>>>,
	compressed_memory_items: LruCache <Key, MemoryCacheItem>,
	live_filesystem_items: LruCache <Key, Arc <FilesystemItem>>,
	dead_filesystem_items: LruCache <Key, Arc <FilesystemItem>>,

	uncompressed_memory_hits: u64,
	compressed_memory_hits: u64,
	live_filesystem_hits: u64,
	dead_filesystem_hits: u64,
	misses: u64,

}

struct FilesystemItem {
	storage_manager: Arc <ChunkCacheData>,
	filename: String,
	compressed: bool,
	uncompressed_size: usize,
	stored_size: usize,
}

pub struct ChunkCacheStatus {

	pub uncompressed_memory_items: u64,
	pub compressed_memory_items: u64,
	pub live_filesystem_items: u64,
	pub dead_filesystem_items: u64,

	pub uncompressed_memory_hits: u64,
	pub compressed_memory_hits: u64,
	pub live_filesystem_hits: u64,
	pub dead_filesystem_hits: u64,
	pub misses: u64,

}

impl <Key: ChunkCacheKey> ChunkCache <Key> {

	#[ inline ]
	pub fn new <PathRef: AsRef <Path>> (
		path_ref: PathRef,
		num_threads: usize,
		uncompressed_memory_cache_size: usize,
		compressed_memory_cache_size: usize,
		live_filesystem_cache_size: usize,
		dead_filesystem_cache_size: usize,
		debug: bool,
	) -> Result <ChunkCache <Key>, String> {

		Self::new_impl (
			path_ref.as_ref (),
			num_threads,
			uncompressed_memory_cache_size,
			compressed_memory_cache_size,
			live_filesystem_cache_size,
			dead_filesystem_cache_size,
			debug,
		)

	}

	fn new_impl (
		path: & Path,
		num_threads: usize,
		uncompressed_memory_cache_size: usize,
		compressed_memory_cache_size: usize,
		live_filesystem_cache_size: usize,
		dead_filesystem_cache_size: usize,
		debug: bool,
	) -> Result <ChunkCache <Key>, String> {

		// try and create filesystem cache path

		fs::create_dir_all (
			path,
		).map_err (
			|error|

			format! (
				"Error creating filesystem cache path: {}: {}",
				path.to_string_lossy (),
				error.description ())

		) ?;

		// check we can access filesystem cache path

		let metadata =
			fs::metadata (
				path,
			).map_err (
				|_|

				format! (
					"Unable to access: {}",
					path.to_string_lossy ())

			) ?;

		if ! metadata.is_dir () {

			return Err (
				format! (
					"Not a directory: {}",
					path.to_string_lossy ()));

		}

		// TODO create a test file to check permissions

		Ok (ChunkCache {

			data: Arc::new (
				ChunkCacheData {
					path: path.to_owned (),
					debug: debug,
				}),

			state: Arc::new (
				Mutex::new (
					ChunkCacheState {

				uncompressed_memory_items:
					LruCache::new (
						uncompressed_memory_cache_size),

				compressed_memory_items:
					LruCache::new (
						compressed_memory_cache_size),

				live_filesystem_items:
					LruCache::new (
						live_filesystem_cache_size),

				dead_filesystem_items:
					LruCache::new (
						dead_filesystem_cache_size),

				uncompressed_memory_hits: 0,
				compressed_memory_hits: 0,
				live_filesystem_hits: 0,
				dead_filesystem_hits: 0,
				misses: 0,

			})),

			cpu_pool:
				CpuPool::new (
					num_threads),

		})

	}

	pub fn insert (
		& self,
		key: Key,
		uncompressed_data: Arc <Vec <u8>>,
		live: bool,
	) -> Result <(), String> {

		let entry_filename =
			rand::thread_rng ()
				.gen_ascii_chars ()
				.take (16)
				.collect ();

		let entry_path =
			self.data.path.join (
				& entry_filename);

		let mut self_state =
			self.state.lock ().unwrap ();

		// store in uncompressed memory cache

		if ! self_state.uncompressed_memory_items.contains_key (
			& key,
		) {

			self_state.uncompressed_memory_items.insert (
				key.clone (),
				uncompressed_data.clone ());

		}

		// check if it is compressed

		let in_live_filesystem_cache =
			self_state.live_filesystem_items.contains_key (
				& key);

		let in_dead_filesystem_cache =
			self_state.live_filesystem_items.contains_key (
				& key);

		if (
			! in_live_filesystem_cache
			&& ! in_dead_filesystem_cache
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

			if live {

				self_state.compressed_memory_items.insert (
					key.clone (),
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

			// store in compressed filesystem cache

			if ! in_live_filesystem_cache && ! in_dead_filesystem_cache {

				let mut output =
					File::create (
						& entry_path,
					).unwrap_or_else (
						|error|

						panic! (
							"Unable to create {}: {}",
							entry_path.to_string_lossy (),
							error.description ())

					);

				output.write (
					& stored_data,
				).unwrap_or_else (
					|error|

					panic! (
						"Error writing to {}: {}",
						entry_path.to_string_lossy (),
						error.description ())

				);

				// create and store the filesystem item

				let filesystem_item =
					FilesystemItem {

					storage_manager: self.data.clone (),
					filename: entry_filename,
					compressed: compressed,
					stored_size: stored_data.len (),
					uncompressed_size: uncompressed_data.len (),

				};

				if live {

					if in_dead_filesystem_cache {

						self_state.dead_filesystem_items.remove (
							& key);

					}

					self_state.live_filesystem_items.insert (
						key,
						Arc::new (filesystem_item));

				} else if ! in_live_filesystem_cache {

					self_state.dead_filesystem_items.insert (
						key,
						Arc::new (filesystem_item));

				}

			}

		}

		// return

		Ok (())

	}

	pub fn get (
		& self,
		debug: & Output,
		key: & Key,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

		let mut self_state =
			self.state.lock ().unwrap ();

		// try in uncompressed memory cache

		match (
			self_state.uncompressed_memory_items.get_mut (
				& key,
			).map (|item| item.clone ())
		) {

			Some (item_data) => {

				// freshen caches

				self_state.compressed_memory_items.get_mut (
					& key);

				let in_live =
					self_state.live_filesystem_items.get_mut (
						& key,
					).is_some ();

				if ! in_live {

					let filesystem_item =
						self_state.dead_filesystem_items.get_mut (
							& key,
						).unwrap ().clone ();

					if self.data.debug {

						output_message! (
							debug,
							"Promote to live: {}",
							filesystem_item.filename);

					}

					self_state.dead_filesystem_items.remove (
						& key);

					self_state.live_filesystem_items.insert (
						key.to_owned (),
						filesystem_item);

				}

				// update hits

				self_state.uncompressed_memory_hits += 1;

				// return

				Some (
					futures::done (
						Ok (item_data)
					).boxed (),
				)

			},

			None =>
				self.get_compressed (
					debug,
					self_state.deref_mut (),
					key),

		}

	}

	fn get_compressed (
		& self,
		debug: & Output,
		self_state: & mut ChunkCacheState <Key>,
		key: & Key,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

		// try in compressed memory cache

		match (

			self_state.compressed_memory_items.get_mut (
				& key,
			).map (|item| item.clone ())

		) {

			Some (
				MemoryCacheItem::Compressed (
					ref compressed_data,
					uncompressed_size)
			) => Some ({

				// freshen caches

				let in_live =
					self_state.live_filesystem_items.get_mut (
						& key,
					).is_some ();

				if ! in_live {

					let filesystem_item =
						self_state.dead_filesystem_items.get_mut (
							& key,
						).unwrap ().clone ();

					if self.data.debug {

						output_message! (
							debug,
							"Promote to live: {}",
							filesystem_item.filename);

					}

					self_state.live_filesystem_items.insert (
						key.to_owned (),
						filesystem_item);

					self_state.dead_filesystem_items.remove (
						& key);

				}

				// decompress and return future

				let self_clone =
					self.clone ();

				let compressed_data =
					compressed_data.clone ();

				let key =
					key.to_owned ();

				self.cpu_pool.spawn_fn (
					move || {

					let uncompressed_data =
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

						) ?;

					let mut self_state =
						self_clone.state.lock ().unwrap ();

					self_state.uncompressed_memory_items.insert (
						key,
						uncompressed_data.clone ());

					// update hits

					self_state.compressed_memory_hits += 1;

					// return

					Ok (uncompressed_data)

				}).boxed ()

			}),

			Some (
				MemoryCacheItem::Uncompressed (
					ref uncompressed_data)
			) => Some ({

				// freshen caches

				let in_live =
					self_state.live_filesystem_items.get_mut (
						& key,
					).is_some ();

				if ! in_live {

					let filesystem_item =
						self_state.dead_filesystem_items.get_mut (
							& key,
						).unwrap ().clone ();

					if self.data.debug {

						output_message! (
							debug,
							"Promote to live: {}",
							filesystem_item.filename);

					}

					self_state.live_filesystem_items.insert (
						key.clone (),
						filesystem_item);

					self_state.dead_filesystem_items.remove (
						& key);

				}

				// update hits

				self_state.compressed_memory_hits += 1;

				// return data directly

				futures::done (
					Ok (uncompressed_data.clone ())
				).boxed ()

			}),

			None =>
				self.get_filesystem (
					self_state,
					key),

		}

	}

	fn get_filesystem (
		& self,
		self_state: & mut ChunkCacheState <Key>,
		key: & Key,
	) -> Option <BoxFuture <Arc <Vec <u8>>, String>> {

		let live_filesystem_item: Option <Arc <FilesystemItem>> =
			self_state.live_filesystem_items.get_mut (
				key,
			).map (|filesystem_item|
				filesystem_item.clone ()
			);

		let is_live =
			live_filesystem_item.is_some ();

		let dead_filesystem_item =
			if is_live {
				None
			} else {
				self_state.dead_filesystem_items.get_mut (
					key,
				).map (|filesystem_item|
					filesystem_item.clone ()
				)
			};

		if let Some (filesystem_item) =
			live_filesystem_item.or (
				dead_filesystem_item) {

			// move to live

			if ! is_live {

				self_state.dead_filesystem_items.remove (
					key);

				self_state.live_filesystem_items.insert (
					key.to_owned (),
					filesystem_item.clone ());

			}

			// load and return

			let key =
				key.to_owned ();

			let self_clone =
				self.clone ();

			Some (self.cpu_pool.spawn_fn (
				move || {

				let (uncompressed_data, compressed_data) =
					filesystem_item.get () ?;

				// try and insert in memory cache

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

				// update hits

				if is_live {
					self_state.live_filesystem_hits += 1;
				} else {
					self_state.dead_filesystem_hits += 1;
				}

				// return

				Ok (uncompressed_data)

			}).boxed ())

		} else {

			self_state.misses += 1;

			None

		}

	}

	pub fn status (
		& self,
	) -> ChunkCacheStatus {

		let self_state =
			self.state.lock ().unwrap ();

		ChunkCacheStatus {

			uncompressed_memory_items:
				self_state.uncompressed_memory_items.len () as u64,

			compressed_memory_items:
				self_state.compressed_memory_items.len () as u64,

			live_filesystem_items:
				self_state.live_filesystem_items.len () as u64,

			dead_filesystem_items:
				self_state.dead_filesystem_items.len () as u64,

			uncompressed_memory_hits:
				self_state.uncompressed_memory_hits,

			compressed_memory_hits:
				self_state.compressed_memory_hits,

			live_filesystem_hits:
				self_state.live_filesystem_hits,

			dead_filesystem_hits:
				self_state.dead_filesystem_hits,

			misses:
				self_state.misses,

		}

	}

}

impl FilesystemItem {

	#[ inline ]
	fn path (
		& self,
	) -> PathBuf {

		self.storage_manager.path.join (
			& self.filename)

	}

	fn get (
		& self,
	) -> Result <(Arc <Vec <u8>>, Option <Arc <Vec <u8>>>), String> {

		let mut file =
			File::open (
				self.path (),
			).map_err (
				|error|

				format! (
					"Error loading storage item {}: {}",
					self.filename,
					error.description ())

			) ?;

		let mut stored_data =
			Vec::with_capacity (
				self.stored_size);

		file.read_to_end (
			& mut stored_data,
		).map_err (
			|error|

			format! (
				"Error loading storage item {}: {}",
				self.filename,
				error.description ())

		) ?;

		if self.compressed {

			let uncompressed_data =
				Arc::new (

				minilzo::decompress (
					& stored_data,
					self.uncompressed_size,
				).map_err (
					|error|

					format! (
						"Error decompressing stored data: {:?}",
						error)

				) ?

			);

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
				self.filename,
				error.description ())

		);

	}

}

// ex: noet ts=4 filetype=rust
