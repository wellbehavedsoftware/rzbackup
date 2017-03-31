use std::collections::HashMap;
use std::collections::LinkedList;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use futures::BoxFuture;
use futures::Future;
use futures::Sink;
use futures::Stream;
use futures::future;
use futures::sync::mpsc;
use futures::sync::oneshot;
use futures_cpupool::CpuFuture;
use futures_cpupool::CpuPool;

use output::Output;

use ::misc::*;
use ::zbackup::data::*;
use ::zbackup::disk_format::*;
use ::zbackup::repository_core::*;

type ChunkMap =
	Arc <HashMap <ChunkId, ChunkData>>;

type ChunkMapResult =
	Result <ChunkMap, String>;

type ChunkMapSender =
	oneshot::Sender <ChunkMapResult>;

type ChunkMapReceiver =
	CloningSharedFuture <oneshot::Receiver <ChunkMapResult>>;

type ChunkMapFutureSender =
	oneshot::Sender <ChunkMapReceiver>;

type ChunkMapFutureReceiver =
	CloningSharedFuture <oneshot::Receiver <ChunkMapReceiver>>;

struct ChunkMapFutureChannel {
	sender: ChunkMapFutureSender,
	receiver: ChunkMapFutureReceiver,
}

type Task =
	CpuFuture <(), ()>;

#[ derive (Clone) ]
pub struct BundleLoader {
	data: Arc <BundleLoaderData>,
	state: Arc <Mutex <BundleLoaderState>>,
}

struct BundleLoaderData {
	repository_core: Arc <RepositoryCore>,
	num_threads: usize,
}

struct BundleLoaderState {

	loading_now: HashMap <BundleId, ChunkMapReceiver>,
	loading_later: HashMap <BundleId, ChunkMapFutureChannel>,
	loading_later_queue: LinkedList <BundleId>,

	thread_join_handle: Option <JoinHandle <()>>,
	thread_sender: Option <mpsc::Sender <Task>>,
	cpu_pool: Option <CpuPool>,

	num_loads: u64,

}

pub struct BundleLoaderStatus {
	pub num_loads: u64,
	pub loading_now: Vec <BundleId>,
	pub loading_later: Vec <BundleId>,
}

impl BundleLoader {

	pub fn new (
		repository_core: Arc <RepositoryCore>,
		num_threads: usize,
	) -> BundleLoader {

		let (thread_sender, thread_receiver) =
			mpsc::channel (0);

		BundleLoader {

			data: Arc::new (BundleLoaderData {
				repository_core: repository_core,
				num_threads: num_threads,
			}),

			state: Arc::new (Mutex::new (BundleLoaderState {

				loading_now: HashMap::new (),
				loading_later: HashMap::new (),
				loading_later_queue: LinkedList::new (),

				thread_join_handle: Some (
					thread::spawn (
						|| Self::background_thread (
							thread_receiver))
				),

				thread_sender: Some (thread_sender),

				cpu_pool: Some (CpuPool::new (num_threads)),

				num_loads: 0,

			})),

		}

	}

	pub fn load_bundle_async_async (
		& self,
		debug: & Output,
		bundle_id: BundleId,
	) -> BoxFuture <BoxFuture <ChunkMap, String>, String> {

		let mut self_state =
			self.state.lock ().unwrap ();

		if let Some (chunk_map_receiver) =
			self_state.loading_now.get (
				& bundle_id) {

			// already loading

			output_message! (
				debug,
				"BundleLoader.load_bundle_async_async ({}) - Alreading loading",
				bundle_id);

			return future::ok (

				chunk_map_receiver.clone ().map_err (
					|_cancelled| "Cancelled".to_string (),
				).and_then (
					|result| result
				).boxed (),

			).boxed ();

		}

		if let Some (chunk_map_future_channel) =
			self_state.loading_later.get (
				& bundle_id) {

			// already queued for loading

			output_message! (
				debug,
				"BundleLoader.load_bundle_async_async ({}) - Alreading queued",
				bundle_id);

			return chunk_map_future_channel.receiver.clone ().map_err (
				|_cancelled| "Cancelled".to_string (),
			).map (
				|chunk_map_receiver|

				chunk_map_receiver.map_err (
					|_cancelled| "Cancelled".to_string (),
				).and_then (
					|result| result,
				).boxed ()

			).boxed ()

		}

		// start loading

		output_message! (
			debug,
			"BundleLoader.load_bundle_async_async ({}) - Start loading",
			bundle_id);

		self.load_bundle_async_async_impl (
			self_state.deref_mut (),
			debug,
			bundle_id,
		).map_err (
			|_cancelled| "Cancelled".to_string (),
		).map (
			|chunk_map_receiver|

			chunk_map_receiver.map_err (
				|_cancelled| "Cancelled".to_string (),
			).and_then (
				|result| result,
			).boxed ()

		).boxed ()

	}

	fn load_bundle_async_async_impl (
		& self,
		self_state: & mut BundleLoaderState,
		_debug: & Output,
		bundle_id: BundleId,
	) -> ChunkMapFutureReceiver {

		let (bundle_future_sender, bundle_future_receiver) =
			oneshot::channel ();

		let bundle_future_receiver =
			bundle_future_receiver.shared ().cloning ();

		self_state.loading_later.insert (
			bundle_id,
			ChunkMapFutureChannel {
				sender: bundle_future_sender,
				receiver: bundle_future_receiver.clone (),
			});

		self_state.loading_later_queue.push_back (
			bundle_id);

		self.start_jobs (
			self_state);

		bundle_future_receiver

	}

	fn load_bundle_async_impl (
		& self,
		self_state: & mut BundleLoaderState,
		bundle_id: BundleId,
		bundle_sender: ChunkMapSender,
		bundle_receiver: ChunkMapReceiver,
	) {

		let self_clone =
			self.clone ();

		self_state.thread_sender.clone ().unwrap ().send (
			self_state.cpu_pool.clone ().unwrap ().spawn_fn (move || {

			bundle_sender.send (
				self_clone.load_bundle_impl (
					bundle_id),
			).map_err (
				|_| "Should never happen"
			).unwrap ();

			let mut self_state =
				self_clone.state.lock ().unwrap ();

			self_state.loading_now.remove (
				& bundle_id);

			self_clone.start_jobs (
				self_state.deref_mut ());

			Ok (())

		})).wait ().unwrap ();

		self_state.loading_now.insert (
			bundle_id,
			bundle_receiver.clone ());

		self_state.num_loads += 1;

	}

	fn start_jobs (
		& self,
		self_state: & mut BundleLoaderState,
	) {

		while self_state.loading_now.len () < self.data.num_threads {

			if let Some (bundle_id) =
				self_state.loading_later_queue.pop_front () {

				let (bundle_sender, bundle_receiver) =
					oneshot::channel ();

				let bundle_receiver =
					bundle_receiver.shared ().cloning ();

				self.load_bundle_async_impl (
					self_state,
					bundle_id,
					bundle_sender,
					bundle_receiver.clone ());

				let bundle_future_channel =
					self_state.loading_later.remove (
						& bundle_id,
					).unwrap ();

				bundle_future_channel.sender.send (
					bundle_receiver,
				).map_err (
					|_| "Should never happen"
				).unwrap ();

			} else {
				break;
			}

		}

	}

	fn load_bundle_impl (
		& self,
		bundle_id: BundleId,
	) -> ChunkMapResult {

		let bundle_path =
			self.data.repository_core.bundle_path (
				bundle_id,
			);

		let bundle_data: Vec <(ChunkId, Vec <u8>)> =
			bundle_read_path (
				bundle_path,
				self.data.repository_core.encryption_key (),
			).map_err (|original_error|

				format! (
					"Error reading bundle {}: {}",
					bundle_id,
					original_error)

			) ?;

		Ok (Arc::new (
			bundle_data.into_iter ().map (
				|(chunk_id, chunk_data)|

				(
					chunk_id,
					Arc::new (chunk_data),
				)

			).collect ()
		))

	}

	pub fn status (
		& self,
	) -> BundleLoaderStatus {

		let self_state =
			self.state.lock ().unwrap ();

		BundleLoaderStatus {

			num_loads: self_state.num_loads,

			loading_now:
				self_state.loading_now.keys ()
					.map (|key| * key)
					.collect (),

			loading_later:
				self_state.loading_later_queue.iter ()
					.map (|key| * key)
					.collect (),

		}

	}

	fn background_thread (
		receiver: mpsc::Receiver <Task>,
	) {

		enum Event {
			NewTask (Task, mpsc::Receiver <Task>),
			TaskComplete,
			Stop,
		}

		fn receiver_into_future (
			receiver: mpsc::Receiver <Task>,
		) -> BoxFuture <Event, ()> {

			receiver.into_future ().map (
				|(value, receiver)|

				match value {

					Some (new_task) =>
						Event::NewTask (
							new_task,
							receiver),

					None =>
						Event::Stop,

				}

			).map_err (
				|(error, _receiver)|
				error,
			).boxed ()

		}

		fn task_into_future (
			task: Task,
		) -> BoxFuture <Event, ()> {

			task.then (
				|_| Ok (Event::TaskComplete)
			).boxed ()

		}

		let mut current_tasks: Vec <BoxFuture <Event, ()>> = vec! [
			receiver_into_future (receiver),
		];

		loop {

			match future::select_all (
				current_tasks.into_iter (),
			).wait () {

				// handle stop

				Ok ((
					Event::Stop,
					_index,
					_remaining_tasks,
				)) =>
					return,

				// handle new task

				Ok ((
					Event::NewTask (new_task, receiver),
					_index,
					remaining_tasks,
				)) =>
					current_tasks =
						remaining_tasks.into_iter ().chain (vec! [
							receiver_into_future (receiver),
							task_into_future (new_task),
						]).collect (),

				// handle completed task

				Ok ((
					Event::TaskComplete,
					_index,
					remaining_tasks,
				)) => {

					current_tasks =
						remaining_tasks;

				},

				// handle error

				Err ((
					_event,
					_index,
					_remaining_tasks,
				)) => {

					panic! (
						"Error in bundle loader");

				},

			}

		}

	}

}

impl Drop for BundleLoaderState {

	fn drop (
		& mut self,
	) {

		self.thread_sender.take ().unwrap ();

		self.thread_join_handle.take ().unwrap ().join ().unwrap ();

		self.cpu_pool.take ().unwrap ();

	}

}

// ex: noet ts=4 filetype=rust
