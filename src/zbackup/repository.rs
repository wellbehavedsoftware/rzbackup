#![ allow (unused_parens) ]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use rust_crypto::digest::Digest;
use rust_crypto::sha1::Sha1;
use rust_crypto::sha2::Sha256;

use futures;
use futures::BoxFuture;
use futures::Future;

use futures_cpupool::CpuPool;

use num_cpus;

use output;
use output::Output;

use protobuf::stream::CodedInputStream;

use rustc_serialize::hex::ToHex;

use misc::*;
use zbackup::bundle_loader::*;
use zbackup::data::*;
use zbackup::disk_format::*;
use zbackup::index_cache::*;
use zbackup::randaccess::*;
use zbackup::repository_core::*;
use zbackup::storage::*;

/// This is the main struct which implements the ZBackup restore functionality.
/// It is multi-threaded, using a cpu pool internally, and it is fully thread
/// safe.

#[ derive (Clone) ]
pub struct Repository {
	data: Arc <RepositoryData>,
	state: Arc <Mutex <RepositoryState>>,
	cpu_pool: CpuPool,
	bundle_loader: BundleLoader,
	storage_manager: StorageManager <ChunkId>,
}

type ChunkMap = Arc <HashMap <ChunkId, ChunkData>>;

/// This controls the configuration of a repository, and is passed to the `open`
/// constructor.

#[ derive (Clone) ]
pub struct RepositoryConfig {
	pub max_uncompressed_memory_cache_entries: usize,
	pub max_compressed_memory_cache_entries: usize,
	pub max_compressed_filesystem_cache_entries: usize,
	pub max_threads: usize,
	pub filesystem_cache_path: String,
	pub work_jobs_total: usize, // deprecated and ignored
	pub work_jobs_batch: usize, // deprecated and ignored
}

struct RepositoryData {
	config: RepositoryConfig,
	core: Arc <RepositoryCore>,
}

pub struct RepositoryStatus {
	pub bundle_loader: BundleLoaderStatus,
	pub storage_manager: StorageManagerStatus,
}

// bundle future

type ChunkFuture =
	BoxFuture <ChunkData, String>;

type ChunkDoubleFuture =
	BoxFuture <ChunkFuture, String>;

struct RepositoryState {
	index_cache: IndexCache,
	bundles_needed: HashSet <BundleId>,
}

impl Repository {

	/// Provides a default configuration for a Repository. This may be useful
	/// for some users of the library, although normally a custom configuration
	/// will be a better option.

	pub fn default_config () -> RepositoryConfig {

		RepositoryConfig {

			max_uncompressed_memory_cache_entries:
				MAX_UNCOMPRESSED_MEMORY_CACHE_ENTRIES,

			max_compressed_memory_cache_entries:
				MAX_COMPRESSED_MEMORY_CACHE_ENTRIES,

			max_compressed_filesystem_cache_entries:
				MAX_COMPRESSED_FILESYSTEM_CACHE_ENTRIES,

			max_threads:
				num_cpus::get () * 2,

			filesystem_cache_path:
				FILESYSTEM_CACHE_PATH.to_owned (),

			work_jobs_total: 0, // deprecated and ignored
			work_jobs_batch: 0, // deprecated and ignored

		}

	}

	/// Constructs a new Repository from a configuration and a path, and an
	/// optional password file path.
	///
	/// This will read the repositories info file, and decrypt the encryption
	/// key using the password, if provided.

	#[ inline ]
	pub fn open <
		RepositoryPath: AsRef <Path>,
		PasswordFilePath: AsRef <Path>,
	> (
		output: & Output,
		repository_config: RepositoryConfig,
		repository_path: RepositoryPath,
		password_file_path: Option <PasswordFilePath>,
	) -> Result <Repository, String> {

		Self::open_impl (
			output,
			repository_config,
			repository_path.as_ref (),
			password_file_path.as_ref ().map (|path| path.as_ref ()),
		)

	}

	fn open_impl (
		output: & Output,
		repository_config: RepositoryConfig,
		repository_path: & Path,
		password_file_path: Option <& Path>,
	) -> Result <Repository, String> {

		// load repository core

		let repository_core =
			Arc::new (
				RepositoryCore::open (
					output,
					repository_path,
					password_file_path,
				) ?
			);

		// create thread pool

		let cpu_pool =
			CpuPool::new (
				repository_config.max_threads);

		// create bundle loader

		let bundle_loader =
			BundleLoader::new (
				repository_core.clone (),
				repository_config.max_threads);

		// create storage manager

		let storage_manager =
			StorageManager::new (
				repository_config.filesystem_cache_path.clone (),
				repository_config.max_threads,
				repository_config.max_uncompressed_memory_cache_entries,
				repository_config.max_compressed_memory_cache_entries,
				repository_config.max_compressed_filesystem_cache_entries * 7/8,
				repository_config.max_compressed_filesystem_cache_entries * 1/8,
				true,
			) ?;

		// create data

		let repository_data =
			Arc::new (RepositoryData {
				config: repository_config,
				core: repository_core.clone (),
			});

		// create state

		let repository_state =
			Arc::new (Mutex::new (RepositoryState {
				index_cache: IndexCache::new (
					repository_core.clone (),
				),
				bundles_needed: HashSet::new (),
			}));

		// return

		Ok (Repository {
			data: repository_data,
			state: repository_state,
			cpu_pool: cpu_pool,
			bundle_loader: bundle_loader,
			storage_manager: storage_manager,
		})

	}

	/// Load the index files. This is not done automatically, but it will be
	/// done lazily when they are first needed. This function also implements a
	/// lazy loading pattern, and so no index files will be reloaded if it is
	/// called more than ones.
	///
	/// Apart from being used internally, this function is designed to be used
	/// by library users who want to eagerly load the indexes so that restore
	/// operations can begin more quickly. This would also allow errors when
	/// reading the index files to be caught more quickly and deterministically.

	pub fn load_indexes (
		& self,
		output: & Output,
	) -> Result <(), String> {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.index_cache.load_if_not_loaded (
			output,
		)

	}

	/// Reload the index files. This forces the indexes to be reloaded, even if
	/// they have already been loaded. This should be called if new backups have
	/// been added to an already-open repository.

	pub fn reload_indexes (
		& self,
		output: & Output,
	) -> Result <(), String> {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.index_cache.reload (
			output,
		)

	}

	/// This will load a backup entirely into memory. The use of this function
	/// should probably be discouraged for most use cases, since backups could
	/// be extremely large.

	pub fn read_and_expand_backup (
		& self,
		output: & Output,
		backup_name: & str,
	) -> Result <(Vec <u8>, [u8; 32]), String> {

		self.load_indexes (
			output,
		) ?;

		// load backup

		let output_job =
			output_job_start! (
				output,
				"Loading backup {}",
				backup_name);

		let backup_info =
			backup_read_path (
				self.data.core.backup_path (
					backup_name),
				self.data.core.encryption_key (),
			) ?;

		// expand backup data

		let mut input =
			Cursor::new (
				backup_info.backup_data ().to_owned ());

		for _iteration in 0 .. backup_info.iterations () {

			let mut temp_output: Cursor <Vec <u8>> =
				Cursor::new (
					Vec::new ());

			let mut sha1_digest =
				Sha1::new ();

			self.follow_instructions (
				& mut input,
				& mut temp_output,
				& mut sha1_digest,
				& |count| {
					if count & 0xf == 0xf {
						output_job.tick ();
					}
				},
			) ?;

			input =
				Cursor::new (
					temp_output.into_inner ());

		}

		output_job.complete ();

		Ok (
			(
				input.into_inner (),
				backup_info.sha256 (),
			)
		)

	}

	/// This function will restore a named backup, writing it to the provided
	/// implementation of the `Write` trait.

	pub fn restore (
		& self,
		output: & Output,
		backup_name: & str,
		target: & mut Write,
	) -> Result <(), String> {

		if backup_name.is_empty () {

			return Err (
				"Backup name must not be empty".to_string ());

		}

		if backup_name.chars ().next ().unwrap () != '/' {

			return Err (
				"Backup name must begin with '/'".to_string ());

		}

		let (input_bytes, checksum) =
			self.read_and_expand_backup (
				output,
				backup_name,
			) ?;

		let mut input =
			Cursor::new (
				input_bytes);

		let output_job =
			output_job_start! (
				output,
				"Restoring {}",
				backup_name);

		// restore backup

		let mut sha256_sum =
			Sha256::new ();

		self.follow_instructions (
			& mut input,
			target,
			& mut sha256_sum,
			& |count| {
				if count & 0x7f == 0x00 {
					output_job.tick ();
				}
			},
		) ?;

		// verify checksum

		let mut sha256_sum_bytes: [u8; 32] =
			[0u8; 32];

		sha256_sum.result (
			& mut sha256_sum_bytes);

		if checksum != sha256_sum_bytes {

			return Err (
				format! (
					"Expected sha256 checksum {} but calculated {}",
					checksum.to_hex (),
					sha256_sum_bytes.to_hex ()));

		}

		// done

		output_job.complete ();

		Ok (())

	}

	#[ doc (hidden) ]
	pub fn restore_test (
		& self,
		output: & Output,
		backup_name: & str,
		target: & mut Write,
	) -> Result <(), String> {

		let output_job =
			output_job_start! (
				output,
				"Restoring {}",
				backup_name);

		let mut input =
			RandomAccess::new (
				output,
				self,
				backup_name,
			) ?;

		let mut buffer: Vec <u8> =
			vec! [0u8; BUFFER_SIZE];

		// restore backup

		loop {

			let bytes_read =
				io_result (
					input.read (
						& mut buffer),
				) ?;

			if bytes_read == 0 {
				break;
			}

			io_result (
				target.write (
					& buffer [
						0 .. bytes_read ]),
			) ?;

		}

		output_job.complete ();

		Ok (())

	}

	fn follow_instruction_async_async (
		& self,
		debug: & Output,
		backup_instruction: & DiskBackupInstruction,
	) -> BoxFuture <BoxFuture <ChunkData, String>, String> {

		if backup_instruction.has_chunk_to_emit ()
		&& backup_instruction.has_bytes_to_emit () {

			let chunk_id =
				backup_instruction.chunk_to_emit ();

			let backup_instruction_bytes_to_emit =
				backup_instruction.bytes_to_emit ().to_vec ();

			self.get_chunk_async_async_debug (
				debug,
				chunk_id,
			).map (
				move |chunk_data_future|

				chunk_data_future.map (
					move |chunk_data|

					Arc::new (
						chunk_data.iter ().map (
							move |& value| value
						).chain (
							backup_instruction_bytes_to_emit.into_iter ()
						).collect ())

				).boxed ()

			).boxed ()

		} else if backup_instruction.has_chunk_to_emit () {

			let chunk_id =
				backup_instruction.chunk_to_emit ();

			self.get_chunk_async_async_debug (
				debug,
				chunk_id,
			)

		} else if backup_instruction.has_bytes_to_emit () {

			futures::done (Ok (
				futures::done (Ok (

				Arc::new (
					backup_instruction.bytes_to_emit ().to_vec ())

				)).boxed ()
			)).boxed ()

		} else {

			futures::failed::<BoxFuture <ChunkData, String>, String> (
				"Instruction with neither chunk or bytes".to_string ()
			).boxed ()

		}

	}

	#[ doc (hidden) ]
	pub fn follow_instructions (
		& self,
		input: & mut Read,
		target: & mut Write,
		digest: & mut Digest,
		progress: & Fn (u64),
	) -> Result <(), String> {

		self.follow_instructions_debug (
			& output::null (),
			input,
			target,
			digest,
			progress,
		)

	}

	#[ doc (hidden) ]
	pub fn follow_instructions_debug (
		& self,
		debug: & Output,
		input: & mut Read,
		target: & mut Write,
		digest: & mut Digest,
		progress: & Fn (u64),
	) -> Result <(), String> {

		let mut coded_input_stream =
			CodedInputStream::new (
				input);

		let mut count: u64 = 0;

		enum JobTarget {
			Chunk (ChunkData),
			FutureChunk (BoxFuture <ChunkData, String>),
		}

		type Job = BoxFuture <JobTarget, String>;

		let mut current_chunk_job: Option <Job> =
			None;

		let mut next_chunk_jobs: LinkedList <Job> =
			LinkedList::new ();

		let mut future_chunk_job: Option <Job> =
			None;

		let mut eof = false;

		loop {

			// load next instruction, if we have room

			if future_chunk_job.is_none () && ! eof {

				if (
					protobuf_result (
						coded_input_stream.eof (),
					) ?
				) {

					eof = true;

				} else {

					let backup_instruction =
						DiskBackupInstruction::read (
							& mut coded_input_stream,
						) ?;

					future_chunk_job = Some (

						self.follow_instruction_async_async (
							debug,
							& backup_instruction,
						).map (
							|future_chunk_data|

							JobTarget::FutureChunk (
								future_chunk_data)

						).boxed ()

					);

				}

			}

			// wait for something to happen

			if current_chunk_job.is_none () {

				current_chunk_job =
					next_chunk_jobs.pop_front ();

			}

			let have_current_chunk_job =
				current_chunk_job.is_some ();

			let have_future_chunk_job =
				future_chunk_job.is_some ();

			if (
				! have_current_chunk_job
				&& ! have_future_chunk_job
			) {
				break;
			}

			let completed_job_target =
				match futures::select_all (vec! [

				current_chunk_job.unwrap_or_else (
					|| futures::empty ().boxed ()),

				future_chunk_job.unwrap_or_else (
					|| futures::empty ().boxed ()),

			]).wait () {

				Ok ((value, 0, remaining_future)) => {

					future_chunk_job =
						if have_future_chunk_job {

							Some (
								remaining_future.into_iter ()
									.next ()
									.unwrap ()
									.boxed ()
							)

						} else { None };

					current_chunk_job = None;

					value

				},

				Ok ((value, 1, remaining_future)) => {

					current_chunk_job =
						if have_current_chunk_job {

							Some (
								remaining_future.into_iter ()
									.next ()
									.unwrap ()
									.boxed ()
							)

						} else { None };

					future_chunk_job = None;

					value

				},

				Ok ((_, _, _)) =>
					panic! ("Not possible"),

				Err ((error, _, _)) =>
					return Err (error),

			};

			// handle the something that happened

			match completed_job_target {

				JobTarget::Chunk (chunk_data) => {

					digest.input (
						& chunk_data);

					io_result (
						target.write (
							& chunk_data)
					) ?;

					progress (
						count);

					count += 1;

				},

				JobTarget::FutureChunk (future_chunk) => {

					next_chunk_jobs.push_back (

						future_chunk.map (
							|chunk_data|

							JobTarget::Chunk (
								chunk_data)

						).boxed ()

					);

				},

			};

		}

		Ok (())

	}

	/// This will load a single chunk from the repository. It can be used to
	/// create advanced behaviours, and is used, for example, by the
	/// `RandomAccess` struct.

	pub fn get_chunk (
		& self,
		chunk_id: ChunkId,
	) -> Result <ChunkData, String> {

		self.get_chunk_debug (
			& output::null (),
			chunk_id,
		)

	}

	#[ doc (hidden) ]
	pub fn get_chunk_debug (
		& self,
		debug: & Output,
		chunk_id: ChunkId,
	) -> Result <ChunkData, String> {

		self.get_chunk_async_debug (
			debug,
			chunk_id,
		).wait ()

	}

	/// This will load a single chunk from the repository, returning immediately
	/// with a future which can later be waited for. The chunk will be loaded in
	/// the background using the cpu pool.

	pub fn get_chunk_async (
		& self,
		chunk_id: ChunkId,
	) -> BoxFuture <ChunkData, String> {

		self.get_chunk_async_debug (
			& output::null (),
			chunk_id,
		)

	}

	#[ doc (hidden) ]
	pub fn get_chunk_async_debug (
		& self,
		debug: & Output,
		chunk_id: ChunkId,
	) -> BoxFuture <ChunkData, String> {

		self.get_chunk_async_async_debug (
			debug,
			chunk_id,
		).and_then (
			|future|

			future.wait ()

		).boxed ()

	}

	/// This will load a single chunk from the repository, returning immediately
	/// with a future which will complete immediately if the chunk is in cache,
	/// with a future which will complete immediately with the chunk data.
	///
	/// If the chunk is not in cache, the returned future will wait until there
	/// is an available thread to start loading the bundle containing the
	/// chunk's data. It will then complete with a future which will in turn
	/// complete when the bundle has been loaded.
	///
	/// This double-asynchronicity allows consumers to efficiently use all
	/// available threads while blocking when none are available. This should
	/// significantly reduce worst-case memory usage.

	pub fn get_chunk_async_async (
		& self,
		chunk_id: ChunkId,
	) -> BoxFuture <BoxFuture <ChunkData, String>, String> {

		self.get_chunk_async_async_debug (
			& output::null (),
			chunk_id,
		)

	}

	#[ doc (hidden) ]
	pub fn get_chunk_async_async_debug (
		& self,
		debug: & Output,
		chunk_id: ChunkId,
	) -> BoxFuture <BoxFuture <ChunkData, String>, String> {

		let mut self_state =
			self.state.lock ().unwrap ();

		if ! self_state.index_cache.loaded () {

			panic! (
				"Must load indexes before getting chunks");

		}

		// lookup via storage manager

		let debug_clone =
			debug.clone ();

		if let Some (chunk_data_future) =
			self.storage_manager.get (
				debug,
				& chunk_id,
			) {

			let self_clone =
				self.clone ();

			return futures::done (
				Ok (chunk_data_future),
			).or_else (
				move |_error: String| {

				let mut self_state =
					self_clone.state.lock ().unwrap ();

				self_clone.load_chunk_async_async (
					& debug_clone,
					self_state.deref_mut (),
					chunk_id)

			}).boxed ();

		}

		// load bundle if chunk is not available

		self.load_chunk_async_async (
			debug,
			self_state.deref_mut (),
			chunk_id)

	}

	fn load_chunk_async_async (
		& self,
		debug: & Output,
		self_state: & mut RepositoryState,
		chunk_id: ChunkId,
	) -> BoxFuture <BoxFuture <ChunkData, String>, String> {

		match self_state.index_cache.get (
			& chunk_id,
		) {

			Some (index_entry) =>
				self.load_chunk_async_async_impl (
					debug,
					self_state,
					chunk_id,
					index_entry.bundle_id (),
				),

			None =>
				futures::failed (
					format! (
						"Missing chunk: {}",
						chunk_id),
				).boxed (),

		}

	}

	fn load_chunk_async_async_impl (
		& self,
		_debug: & Output,
		self_state: & mut RepositoryState,
		chunk_id: ChunkId,
		bundle_id: BundleId,
	) -> ChunkDoubleFuture {

		self_state.bundles_needed.insert (
			bundle_id);

		let self_clone =
			self.clone ();

		self.bundle_loader.load_bundle_async_async (
			& output::null (),
			bundle_id,
		).map (
			move |chunk_map_future: BoxFuture <ChunkMap, String>|

			chunk_map_future.then (
				move |chunk_map_result: Result <ChunkMap, String>| {

				chunk_map_result.map (
					move |chunk_map| {

					let mut self_state =
						self_clone.state.lock ().unwrap ();

					if self_state.bundles_needed.remove (
						& bundle_id) {

						for (chunk_id, chunk_data)
						in chunk_map.iter () {

							self_clone.storage_manager.insert (
								* chunk_id,
								chunk_data.clone (),
								false,
							) ?;

						}

					}

					if let Some (chunk_data) =
						chunk_map.get (& chunk_id) {

						self_clone.storage_manager.insert (
							chunk_id,
							chunk_data.clone (),
							true,
						) ?;

						Ok (chunk_data.clone ())

					} else {

						Err (
							format! (
								"Chunk not found: {}",
								chunk_id)
						)

					}

				}).and_then (
					|result| result
				)

			}).boxed ()

		).boxed ()

	}

	/// This will load a single index entry from the repository. It returns this
	/// as a `MasterIndexEntry`, which includes the index entry and the header
	/// from the index file, since both are generally needed to do anything
	/// useful.
	///
	/// It can be used to create advanced behaviours, and is used, for example,
	/// by the `RandomAccess` struct.

	pub fn get_index_entry (
		& self,
		chunk_id: ChunkId,
	) -> Result <IndexEntry, String> {

		let self_state =
			self.state.lock ().unwrap ();

		if ! self_state.index_cache.loaded () {

			panic! (
				"Must load indexes before getting index entries");

		}

		match self_state.index_cache.get (
			& chunk_id,
		) {

			Some (value) =>
				Ok (value.clone ()),

			None =>
				Err (
					format! (
						"Missing chunk: {}",
						chunk_id)
				),

		}

	}

	/// Returns true if a chunk is present in the loaded indexes

	pub fn has_chunk (
		& self,
		chunk_id: & ChunkId,
	) -> bool {

		let self_state =
			self.state.lock ().unwrap ();

		if ! self_state.index_cache.loaded () {

			panic! (
				"Must load indexes before getting index entries");

		}

		self_state.index_cache.get (
			chunk_id,
		).is_some ()

	}

	/// This is a convenience method to construct a `RandomAccess` struct. It
	/// simply calls the `RandomAccess::new` constructor.

	pub fn open_backup (
		& self,
		output: & Output,
		backup_name: & str,
	) -> Result <RandomAccess, String> {

		RandomAccess::new (
			output,
			self,
			backup_name)

	}

	/// This method closes the repository, removing all temporary files

	pub fn close (
		self,
		output: & Output,
	) {

		let output_job =
			output_job_start! (
				output,
				"Closing repository");

		drop (self);

		output_job.complete ();

	}

	/// This is an accessor method to access the `RepositoryConfig` struct which
	/// was used to construct this `Repository`.

	#[ inline ]
	pub fn config (
		& self,
	) -> & RepositoryConfig {
		& self.data.config
	}

	/// This is an accessor method for the `RepositoryCore` which holds the most
	/// basic details about a repository

	#[ inline ]
	pub fn core (& self) -> & RepositoryCore {
		& self.data.core
	}

	/// This is an accessor method to access the repository's `path`

	#[ inline ]
	pub fn path (& self) -> & Path {
		& self.data.core.path ()
	}

	/// This is an accessor method to access the `StorageInfo` protobug struct
	/// which was loaded from the repository's index file.

	#[ inline ]
	pub fn storage_info (& self) -> & DiskStorageInfo {
		& self.data.core.storage_info ()
	}

	/// This is an accessor method to access the decrypted encryption key which
	/// was stored in the repository's info file and decrypted using the
	/// provided password.

	#[ inline ]
	pub fn encryption_key (& self) -> Option <EncryptionKey> {
		self.data.core.encryption_key ()
	}

	/// Convenience function to return the filesystem path for an index id.

	#[ inline ]
	pub fn index_path (
		& self,
		index_id: IndexId,
	) -> PathBuf {

		self.data.core.index_path (
			index_id,
		)

	}

	/// Convenience function to return the filesystem path for a bundle id.

	#[ inline ]
	pub fn bundle_path (
		& self,
		bundle_id: BundleId,
	) -> PathBuf {

		self.data.core.bundle_path (
			bundle_id,
		)

	}

	pub fn status (
		& self,
	) -> RepositoryStatus {

		RepositoryStatus {

			bundle_loader:
				self.bundle_loader.status (),

			storage_manager:
				self.storage_manager.status (),

		}

	}

}

