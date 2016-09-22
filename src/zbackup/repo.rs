#![ allow (unused_parens) ]

use futures;
use futures::BoxFuture;
use futures::Complete;
use futures::Future;

use futures_cpupool::CpuPool;

use lru_cache::LruCache;

use protobuf;
use protobuf::stream::CodedInputStream;

use rustc_serialize::hex::ToHex;

use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;

use misc::*;

use zbackup::crypto::*;
use zbackup::data::*;
use zbackup::proto;
use zbackup::randaccess::*;
use zbackup::read::*;

type MasterIndex = HashMap <BundleId, MasterIndexEntry>;
type ChunkMap = Arc <HashMap <ChunkId, ChunkData>>;
type ChunkCache = LruCache <ChunkId, ChunkData>;

#[ derive (Clone, Copy) ]
pub struct MasterIndexEntryData {
	pub bundle_id: BundleId,
	pub size: u64,
}

pub type MasterIndexEntry = Arc <MasterIndexEntryData>;

#[ derive (Clone, Copy) ]
pub struct RepositoryConfig {
	max_cache_entries: usize,
	max_threads: usize,
}

const DEFAULT_CONFIG: RepositoryConfig =
	RepositoryConfig {
		max_cache_entries: 0x10000,
		max_threads: 4,
	};

struct RepositoryData {
	config: RepositoryConfig,
	path: String,
	storage_info: proto::StorageInfo,
	encryption_key: Option <EncryptionKey>,
}

type BundleWaiter = Complete <Result <ChunkMap, String>>;

struct RepositoryState {
	master_index: Option <MasterIndex>,
	chunk_cache: ChunkCache,
	bundle_waiters: HashMap <BundleId, Vec <BundleWaiter>>,
}

#[ derive (Clone) ]
pub struct Repository {
	data: Arc <RepositoryData>,
	state: Arc <Mutex <RepositoryState>>,
	cpu_pool: CpuPool,
}

impl Repository {

	pub fn default_config () -> RepositoryConfig {
		DEFAULT_CONFIG
	}

	pub fn open (
		repository_config: RepositoryConfig,
		repository_path: & str,
		password_file_path: Option <& str>,
	) -> Result <Repository, String> {

		// load info file

		stderrln! (
			"Loading repository {}",
			repository_path);

		let storage_info =
			try! (
				read_storage_info (
					& format! (
						"{}/info",
						repository_path)));

		// decrypt encryption key with password

		let encryption_key =
			if storage_info.has_encryption_key () {

			if password_file_path.is_none () {

				return Err (
					"Required password file not provided".to_string ());

			}

			match try! (
				decrypt_key (
					password_file_path.unwrap (),
					storage_info.get_encryption_key ())) {

				Some (key) =>
					Some (key),

				None =>
					return Err (
						"Incorrect password".to_string ()),

			}

		} else {

			if password_file_path.is_some () {

				return Err (
					"Unnecessary password file provided".to_string ());

			}

			None

		};

		// create thread pool

		let cpu_pool =
			CpuPool::new (
				repository_config.max_threads);

		// return

		let repository_data =
			RepositoryData {

			config: repository_config,

			path: repository_path.to_string (),
			storage_info: storage_info,
			encryption_key: encryption_key,

		};

		let repository_state =
			RepositoryState {

			master_index:
				None,

			chunk_cache:
				ChunkCache::new (
					repository_config.max_cache_entries),

			bundle_waiters:
				HashMap::new (),

		};

		Ok (Repository {

			data: Arc::new (
				repository_data),

			state: Arc::new (
				Mutex::new (
					repository_state)),

			cpu_pool: cpu_pool,

		})

	}

	pub fn load_indexes (
		& self,
	) -> Result <(), String> {

		let mut self_state =
			self.state.lock ().unwrap ();

		if self_state.master_index.is_some () {
			return Ok (());
		}

		self.load_indexes_real (
			self_state.deref_mut ())

	}

	pub fn reload_indexes (
		& self,
	) -> Result <(), String> {

		let mut self_state =
			self.state.lock ().unwrap ();

		self.load_indexes_real (
			self_state.deref_mut ())

	}

	fn load_indexes_real (
		& self,
		self_state: & mut RepositoryState,
	) -> Result <(), String> {

		struct IndexEntryData {
			chunk_id: ChunkId,
			bundle_id: BundleId,
			size: u64,
		};

		type IndexLoadResult =
			BoxFuture <
				Vec <IndexEntryData>,
				String,
			>;

		stderr! (
			"Loading indexes");

		// start tasks to load each index

		let mut index_futures: Vec <IndexLoadResult> =
			Vec::new ();

		for dir_entry_or_error in try! (
			io_result (
				fs::read_dir (
					format! (
						"{}/index",
						self.data.path)))
		) {

			let dir_entry =
				try! (
					io_result (
						dir_entry_or_error));

			let file_name =
				dir_entry.file_name ();

			let index_name =
				file_name.to_str ().unwrap ().to_owned ();

			let self_clone =
				self.clone ();

			index_futures.push (
				self.cpu_pool.spawn_fn (
					move || {

				let index = try! (

					read_index (
						& format! (
							"{}/index/{}",
							self_clone.data.path,
							index_name),
						self_clone.data.encryption_key,
					).map_err (
						|error|

						format! (
							"Error loading index: {}",
							error)

					)

				);

				let mut entries: Vec <IndexEntryData> =
					Vec::new ();

				for (index_bundle_header, bundle_info) in index {

					for chunk_record in bundle_info.get_chunk_record () {

						entries.push (
							IndexEntryData {

							chunk_id:
								to_array (
									chunk_record.get_id ()),

							bundle_id: 
								to_array (
									index_bundle_header.get_id ()),

							size: 
								chunk_record.get_size () as u64,

						});

					}

				}

				Ok (entries)

			}).boxed ());

		}

		// construct index as they complete

		let mut count: u64 = 0;

		let mut master_index: MasterIndex =
			HashMap::new ();

		for index_future in index_futures {

			let index_entries =
				try! (
					index_future.wait ());

			for index_entry in index_entries {

				master_index.insert (

					index_entry.chunk_id,

					Arc::new (MasterIndexEntryData {
						bundle_id: index_entry.bundle_id,
						size: index_entry.size,
					}),

				);

			}

			if count & 0x3f == 0x3f {
				stderr! (
					".");
			}

			count += 1;

		}

		stderr! (
			"\n");

		// store the result and return

		self_state.master_index =
			Some (
				master_index);

		Ok (())

	}

	pub fn read_and_expand_backup (
		& self,
		backup_name: & str,
	) -> Result <Vec <u8>, String> {

		try! (
			self.load_indexes ());

		// load backup

		stderr! (
			"Loading backup {}",
			backup_name);

		let backup_info =
			try! (
				read_backup_file (
					format! (
						"{}/backups/{}",
						& self.data.path,
						backup_name),
					self.data.encryption_key,
				).or_else (
					|error| {
						stderrln! ("");
						Err (error)
					}
				)
			);

		// expand backup data

		let mut input =
			Cursor::new (
				backup_info.get_backup_data ().to_owned ());

		for _iteration in 0 .. backup_info.get_iterations () {

			let mut temp_output: Cursor <Vec <u8>> =
				Cursor::new (
					Vec::new ());

			try! (
				self.follow_instructions (
					& mut input,
					& mut temp_output,
					& |count| {
						if count & 0xf == 0xf {
							stderr! (".");
						}
					}));

			input =
				Cursor::new (
					temp_output.into_inner ());

		}

		stderr! (
			"\n");

		Ok (input.into_inner ())

	}

	pub fn restore (
		& self,
		backup_name: & str,
		output: & mut Write,
	) -> Result <(), String> {

		let mut input =
			Cursor::new (
				try! (
					self.read_and_expand_backup (
						backup_name)));

		// restore backup

		stderr! (
			"Restoring backup");

		try! (
			self.follow_instructions (
				& mut input,
				output,
				& |count| {
					if count & 0x1ff == 0x1ff {
						stderr! (".");
					}
				}));

		stderr! (
			"\n");

		stderrln! (
			"Restore complete");

		Ok (())

	}

	pub fn restore_test (
		& self,
		backup_name: & str,
		output: & mut Write,
	) -> Result <(), String> {

		stderr! (
			"Loading backup {}",
			backup_name);

		let mut input =
			try! (
				RandomAccess::new (
					self,
					backup_name));

		let mut buffer: Vec <u8> =
			vec! [0u8; BUFFER_SIZE];

		// restore backup

		stderr! (
			"Restoring backup");

		loop {

			let bytes_read =
				try! (
					io_result (
						input.read (
							& mut buffer)));

			if bytes_read == 0 {
				break;
			}

			try! (
				io_result (
					output.write (
						& buffer [
							0 .. bytes_read ])));

		}

		stderrln! (
			"Restore complete");

		Ok (())

	}

	fn follow_instruction_async (
		& self,
		backup_instruction: & proto::BackupInstruction,
	) -> BoxFuture <ChunkData, String> {

		if backup_instruction.has_chunk_to_emit ()
		&& backup_instruction.has_bytes_to_emit () {

			futures::failed::<ChunkData, String> (
				"Instruction with both chunk and bytes".to_string ()
			).boxed ()

		} else if backup_instruction.has_chunk_to_emit () {

			let chunk_id =
				to_array (
					backup_instruction.get_chunk_to_emit ());

			self.get_chunk_async (
				chunk_id,
			)

		} else if backup_instruction.has_bytes_to_emit () {

			futures::done (
				Ok (
					Arc::new (
						backup_instruction.get_bytes_to_emit ().to_vec ())
				)
			).boxed ()

		} else {

			futures::failed::<ChunkData, String> (
				"Instruction with neither chunk or bytes".to_string ()
			).boxed ()

		}

	}

	pub fn follow_instructions (
		& self,
		input: & mut Read,
		output: & mut Write,
		progress: & Fn (u64),
	) -> Result <(), String> {

		let mut coded_input_stream =
			CodedInputStream::new (
				input);

		let mut count: u64 = 0;

		type Job =
			BoxFuture <ChunkData, String>;

		let mut job_list: Vec <Job> =
			vec! [];

		let mut job_position: usize = 0;
		let mut job_count: usize = 0;

		let mut eof = false;

		while (
			! eof
			|| job_count > 0
		) {

			// consume job results

			while (
				(
					job_count > 0
				) && (
					eof
					|| job_count >= WORK_JOBS_TOTAL - WORK_JOBS_BATCH
				)
			) {

				let job_future =
					& mut job_list [job_position];

				let job_result =
					job_future.wait ();

				let job_content =
					try! (
						job_result);

				try! (
					io_result (
						output.write (
							& job_content)));

				progress (
					count);

				count += 1;

				job_position =
					(job_position + 1) % WORK_JOBS_TOTAL;

				job_count -= 1;

			}

			// start jobs

			if (
				! eof
				&& job_count < WORK_JOBS_TOTAL
			) {

				let mut backup_instructions =
					vec! [];

				{

					while (
						! eof
						&& job_count + backup_instructions.len ()
							< WORK_JOBS_TOTAL
					) {

						if (
							try! (
								protobuf_result (
									coded_input_stream.eof ()))
						) {
							eof = true;
							break;
						}

						let instruction_length =
							try! (
								protobuf_result (
									coded_input_stream.read_raw_varint32 ()));

						let instruction_old_limit =
							try! (
								protobuf_result (
									coded_input_stream.push_limit (
										instruction_length)));

						let backup_instruction =
							try! (

							protobuf_result (
								protobuf::core::parse_from::<
									proto::BackupInstruction
								> (
									& mut coded_input_stream,
								),
							)

						);

						backup_instructions.push (
							backup_instruction);

						coded_input_stream.pop_limit (
							instruction_old_limit);

					}

				}

				for backup_instruction in backup_instructions {

					let new_job_future = (

						self.follow_instruction_async (
							& backup_instruction)

					).boxed ();

					let new_job_position =
						(job_position + job_count) % WORK_JOBS_TOTAL;

					if new_job_position == job_list.len () {

						job_list.push (
							new_job_future);

					} else {

						job_list [new_job_position] =
							new_job_future;

					}

					job_count += 1;

				}

			}

		}

		Ok (())

	}

	pub fn get_chunk (
		& self,
		chunk_id: ChunkId,
	) -> Result <ChunkData, String> {

		self.get_chunk_async (
			chunk_id,
		).wait ()

	}

	pub fn get_chunk_async (
		& self,
		chunk_id: ChunkId,
	) -> BoxFuture <ChunkData, String> {

		let bundle_id;

		{

			let mut self_state =
				self.state.lock ().unwrap ();

			if self_state.master_index.is_none () {

				match self.load_indexes () {

					Ok (_) => (),

					Err (error) =>
						return futures::failed (
							error,
						).boxed (),

				}

			}

			// lookup in cache

			match (

				self_state.chunk_cache.get_mut (
					& chunk_id)

			) {

				Some (chunk_data) => {

					return futures::done (
						Ok (
							chunk_data.clone ()
						)
					).boxed ();

				},

				None => (),

			}

			// get bundle id

			bundle_id = match (

				self_state.master_index.as_ref ().unwrap ().get (
					& chunk_id,
				).clone ()

			) {

				Some (index_entry) =>
					index_entry.bundle_id,

				None => {

					return futures::failed (
						format! (
							"Missing chunk: {}",
							chunk_id.to_hex ()),
					).boxed ();

				},

			};

		}

		// load bundle

		let get_bundle_future =
			self.get_bundle (
				bundle_id);

		get_bundle_future.and_then (
			move |chunk_map|

			chunk_map.get (
				& chunk_id,
			).ok_or (

				format! (
					"Expected to find chunk {} in bundle {}",
					chunk_id.to_hex (),
					bundle_id.to_hex ())

			).map (
				|chunk_data|

				chunk_data.clone ()

			)

		).boxed ()

	}

	fn get_bundle (
		& self,
		bundle_id: BundleId,
	) -> BoxFuture <ChunkMap, String> {

		let bundle_path =
			format! (
				"{}/bundles/{}/{}",
				self.data.path,
				& bundle_id.to_hex () [0 .. 2],
				bundle_id.to_hex ());

		let mut self_state =
			self.state.lock ().unwrap ();


		if (

			self_state.bundle_waiters.contains_key (
				& bundle_id)

		) {

			let (complete, future) =
				futures::oneshot ();

			self_state.bundle_waiters.get_mut (
				& bundle_id,
			).unwrap ().push (
				complete,
			);

			future.map_err (
				|_|

				"Cancelled".to_owned ()

			).and_then (
				|chunk_maps_result| {

				chunk_maps_result

			}).boxed ()

		} else {

			let self_clone_1 =
				self.clone ();

			let self_clone_2 =
				self.clone ();

			self_state.bundle_waiters.insert (
				bundle_id,
				Vec::new ());

			self.cpu_pool.spawn_fn (
				move || {

				let chunk_map_result = (

					read_bundle (
						bundle_path,
						self_clone_1.data.encryption_key)

				).map_err (
					|original_error| {

					format! (
						"Error reading bundle {}: {}",
						bundle_id.to_hex (),
						original_error)

				}).map (
					move |bundle_data| {

					let mut chunk_map =
						HashMap::new ();

					for (found_chunk_id, found_chunk_data) in bundle_data {

						chunk_map.insert (
							found_chunk_id,
							Arc::new (
								found_chunk_data));

					}

					Arc::new (chunk_map)

				});

				let mut self_state =
					self_clone_2.state.lock ().unwrap ();

				for (chunk_id, chunk_data)
				in chunk_map_result.as_ref ().unwrap_or (
					& Arc::new (HashMap::new ()),
				).iter () {

					self_state.chunk_cache.insert (
						chunk_id.clone (),
						chunk_data.clone ());

				}

				let bundle_waiters =
					self_state.bundle_waiters.remove (
						& bundle_id,
					).unwrap ();

				for bundle_waiter in bundle_waiters {

					bundle_waiter.complete (
						chunk_map_result.clone ());

				}

				chunk_map_result

			}).boxed ()

		}

	}

	pub fn get_index_entry (
		& self,
		chunk_id: ChunkId,
	) -> Result <MasterIndexEntry, String> {

		let self_state =
			self.state.lock ().unwrap ();

		if self_state.master_index.is_none () {

			try! (
				self.load_indexes ());

		}

		match (

			self_state.master_index.as_ref ().unwrap ().get (
				& chunk_id,
			).clone ()

		) {

			Some (value) =>
				Ok (value.clone ()),

			None =>
				Err (
					format! (
						"Missing chunk: {}",
						chunk_id.to_hex ())
				),

		}

	}

	pub fn open_backup (
		& self,
		backup_name: & str,
	) -> Result <RandomAccess, String> {

		RandomAccess::new (
			self,
			backup_name)

	}

	pub fn config (
		& self,
	) -> & RepositoryConfig {
		& self.data.config
	}

	pub fn storage_info (
		& self,
	) -> & proto::StorageInfo {
		& self.data.storage_info
	}

	pub fn encryption_key (
		& self,
	) -> Option <[u8; KEY_SIZE]> {
		self.data.encryption_key
	}

}
