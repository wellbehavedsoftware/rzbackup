use futures;
use futures::BoxFuture;
use futures::Future;

use output::Output;

pub fn concurrent_controller <

	StateType,
	TaskType: Send + Sized + 'static,

	GetNextTaskFunction:
		Fn (& mut StateType) -> Option <BoxFuture <TaskType, String>>,

	HandleResultFunction:
		Fn (& mut StateType, TaskType) -> Result <(), String>,

> (
	output: & Output,
	max_tasks: usize,
	state: & mut StateType,
	get_next_task: GetNextTaskFunction,
	handle_result: HandleResultFunction,
) -> Result <(), String> {

	let mut task_futures: Vec <BoxFuture <TaskType, String>> =
		Vec::new ();

	output.pause ();

	loop {

		while task_futures.len () < max_tasks {

			if let Some (task_future) =
				get_next_task (state) {

				task_futures.push (
					task_future,
				);

			} else {

				break;

			}

		}

		// wait for background tasks

		if task_futures.is_empty () {
			break;
		}

		output.unpause ();

		let (task_value, _index, remaining_tasks) =
			futures::select_all (
				task_futures,
			).wait ().map_err (
				|(error, _index, _remaining_tasks)|

				error

			) ?;

		task_futures = remaining_tasks;

		output.pause ();

		// process background task

		handle_result (
			state,
			task_value,
		) ?;

	}

	output.unpause ();

	Ok (())

}

// ex: noet ts=4 filetype=rust
