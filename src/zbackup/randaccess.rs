use protobuf;
use protobuf::stream::CodedInputStream;

use rustc_serialize::hex::ToHex;

use std::cmp;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::sync::Arc;

use ::misc::*;

use zbackup::proto;
use zbackup::repo::Repository;

enum InstructionRefContent {

	Chunk ([u8; 24]),
	Bytes (Arc <Vec <u8>>),

}

struct InstructionRef {

	content: InstructionRefContent,

	start: u64,
	end: u64,

}

pub struct RandomAccess <'a> {

	repo: & 'a mut Repository,
	instruction_refs: Vec <InstructionRef>,
	size: u64,

	position: u64,
	chunk_bytes: Arc <Vec <u8>>,
	chunk_position: u64,

}

impl <'a> RandomAccess <'a> {

	pub fn new (
		repo: & 'a mut Repository,
		backup_name: & str,
	) -> Result <RandomAccess <'a>, String> {

		let mut input =
			Cursor::new (
				try! (
					repo.read_and_expand_backup (
						backup_name)));

		let mut coded_input_stream =
			CodedInputStream::new (
				& mut input);

		let mut instruction_refs: Vec <InstructionRef> =
			vec! ();

		let mut offset: u64 = 0;

		while ! try! (
			protobuf_result (
				coded_input_stream.eof ())) {

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
						protobuf::core::parse_from::<proto::BackupInstruction> (
							& mut coded_input_stream)));

			coded_input_stream.pop_limit (
				instruction_old_limit);

			if backup_instruction.has_chunk_to_emit () {

				let chunk_id =
					to_array (
						backup_instruction.get_chunk_to_emit ());

				let index_entry =
					try! (
						repo.get_index_entry (
							& chunk_id));

				instruction_refs.push (
					InstructionRef {

					content:
						InstructionRefContent::Chunk (
							to_array (
								backup_instruction.get_chunk_to_emit ())),

					start:
						offset,

					end:
						offset + index_entry.size,

				});

				offset +=
					index_entry.size;

			}

			if backup_instruction.has_bytes_to_emit () {

				let bytes =
					backup_instruction.get_bytes_to_emit ();

				let size =
					bytes.len () as u64;

				instruction_refs.push (
					InstructionRef {

					content:
						InstructionRefContent::Bytes (
							Arc::new (
								bytes.to_owned ())),

					start:
						offset,

					end:
						offset + size,

				});

				offset +=
					size;

			}

		}

		Ok (RandomAccess {

			repo: repo,
			instruction_refs: instruction_refs,
			size: offset,

			position: 0,
			chunk_bytes: Arc::new (vec! ()),
			chunk_position: 0,

		})

	}

}

impl <'a> Read for RandomAccess <'a> {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> Result <usize, io::Error> {

		let mut function_bytes_read: u64 = 0;

		loop {

			let loop_bytes_read: u64 =
				cmp::min (
					self.chunk_bytes.len () as u64
						- self.chunk_position,
					buffer.len () as u64
						- function_bytes_read);

			{

				let source =
					& self.chunk_bytes [
						self.chunk_position as usize ..
						self.chunk_position as usize
							+ loop_bytes_read as usize];

				let mut target =
					& mut buffer [
						function_bytes_read as usize ..
						function_bytes_read as usize
							+ loop_bytes_read as usize];

				try! (
					target.write_all (
						source));

			}

			self.position +=
				loop_bytes_read;

			self.chunk_position +=
				loop_bytes_read;

			function_bytes_read +=
				loop_bytes_read;

			if function_bytes_read == buffer.len () as u64 {

				break;

			}

			let position_temp =
				self.position;

			let instruction_ref_index =
				match self.instruction_refs.binary_search_by (

				|probe|

				if position_temp < probe.start {
					cmp::Ordering::Greater
				} else if probe.end <= position_temp {
					cmp::Ordering::Less
				} else {
					cmp::Ordering::Equal
				}

			) {

				Ok (value) =>
					value,

				Err (_) =>
					break,

			};

			let instruction_ref =
				& mut self.instruction_refs [
					instruction_ref_index];

			match instruction_ref.content {

				InstructionRefContent::Chunk (ref chunk_id) => {

					self.chunk_bytes =
						try! (
							self.repo.get_chunk (
								* chunk_id,
							).map_err (
								|_error|
								io::Error::new (
									io::ErrorKind::InvalidData,
									format! (
										"Chunk not found: {}",
										chunk_id.to_hex ()))
							));

					self.chunk_position =
						0;

				},

				InstructionRefContent::Bytes (ref mut bytes_data) => {

					self.chunk_bytes =
						bytes_data.clone ();

					self.chunk_position =
						0;

				},

			}

		}

		Ok (
			function_bytes_read as usize)

	}

}

impl <'a> Seek for RandomAccess <'a> {

	fn seek (
		& mut self,
		seek_from: SeekFrom,
	) -> io::Result <u64> {

		self.position =
			match seek_from {

			SeekFrom::Start (value) =>
				value,

			SeekFrom::Current (value) =>
				((self.position as i64) + value) as u64,

			SeekFrom::End (value) =>
				((self.size as i64) + value) as u64,

		};

		let position_temp =
			self.position;

		match self.instruction_refs.binary_search_by (

			|probe|

			if position_temp < probe.start {
				cmp::Ordering::Greater
			} else if probe.end <= position_temp {
				cmp::Ordering::Less
			} else {
				cmp::Ordering::Equal
			}

		) {

			Ok (instruction_ref_index) => {

				let instruction_ref =
					& self.instruction_refs [
						instruction_ref_index];

				match instruction_ref.content {

					InstructionRefContent::Chunk (ref chunk_id) => {

						self.chunk_bytes =
							try! (
								self.repo.get_chunk (
									* chunk_id,
								).map_err (
									|_error|
									io::Error::new (
										io::ErrorKind::InvalidData,
										format! (
											"Chunk not found: {}",
											chunk_id.to_hex ()))
								));

						self.chunk_position =
							self.position - instruction_ref.start;

					},

					InstructionRefContent::Bytes (ref bytes_data) => {

						self.chunk_bytes =
							bytes_data.clone ();

						self.chunk_position =
							self.position - instruction_ref.start;

					},

				};

			},

			Err (_) => {

				self.chunk_bytes =
					Arc::new (
						vec! ());

				self.chunk_position =
					0;

			},

		};

		Ok (self.position)

	}

}
