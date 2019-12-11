palette!(Palette {
	PC = [0xFF, 0xFF, 0xFF],
	Read = [0x00, 0xFF, 0x00],
	RW = [0xFF, 0xFF, 0x00],
	Write = [0xFF, 0x00, 0x00],
	Other = [0x00, 0x00, 0x00],
});

pub type Type = i64;

pub struct Computer {
	memory: std::collections::BTreeMap<usize, Type>,
	input: crossbeam::channel::Receiver<Type>,
	want_input: crossbeam::channel::Sender<()>,
	output: crossbeam::channel::Sender<Type>,
	exit: crossbeam::channel::Sender<()>,
}

impl Computer {
	pub fn new(
		memory: Vec<Type>,
		input: crossbeam::channel::Receiver<Type>,
		want_input: crossbeam::channel::Sender<()>,
		output: crossbeam::channel::Sender<Type>,
		exit: crossbeam::channel::Sender<()>,
	) -> Self {
		Self {
			memory: memory.into_iter().enumerate().collect(),
			input,
			want_input,
			output,
			exit,
		}
	}

	pub fn into_memory(self) -> Vec<Type> {
		self.memory.values().cloned().collect()
	}

	pub fn run(&mut self, _video: Option<&str>) -> anyhow::Result<()> {
		let mut pc: usize = 0;
		let mut base: Type = 0;
		#[cfg(feature = "video")]
		let mut video = crate::video::OptionalVideo::<Palette>::new(
			#[cfg(not(test))]
			_video,
			#[cfg(test)]
			None,
			self.memory.len() as u16,
			1,
			10,
		)?;
		#[cfg(feature = "video")]
		video.silence_unused_warning();
		loop {
			#[cfg(feature = "video")]
			let mut read = std::collections::HashSet::<Type>::new();
			#[cfg(feature = "video")]
			let mut write = std::collections::HashSet::<Type>::new();
			let opcode = *self.memory.get(&pc).unwrap_or(&0);
			let mode = |index| {
				let mut mode = opcode / 100;
				for _ in 1..index {
					mode /= 10;
				}
				mode % 10
			};
			// dummy_read and dummy_write are only to silence warning that read_value and write_value shouldn't be mut while video feature is disabled
			let mut dummy_read = 0;
			let mut read_value = |index| -> Type {
				let value = *self.memory.get(&(pc + index)).unwrap_or(&0);
				match mode(index) {
					0 => {
						#[cfg(feature = "video")]
						#[cfg(not(test))]
						read.insert(value);
						*self.memory.get(&(value as usize)).unwrap_or(&0)
					}
					1 => {
						#[cfg(feature = "video")]
						#[cfg(not(test))]
						read.insert((pc + index) as Type);
						value as Type
					}
					2 => *self
						.memory
						.get(&((value as Type + base) as usize))
						.unwrap_or(&0),
					_ => {
						dummy_read = 0;
						0
					}
				}
				//println!("mode {} value={} result={}", mode, value, result);
			};
			let mut dummy_write = 0;
			let mut write_value =
				|memory: &mut std::collections::BTreeMap<usize, Type>, index, value| {
					let pos = *memory.entry(pc + index).or_default();
					match mode(index) {
						0 => {
							#[cfg(feature = "video")]
							#[cfg(not(test))]
							write.insert(pos);
							*memory.entry(pos as usize).or_default() = value;
						}
						2 => {
							*memory.entry((base + pos) as usize).or_default() = value;
						}
						_ => dummy_write = 0,
					}
				};
			//println!("{:?} pc={} output={:?}", program, pc, output);
			match opcode % 100 {
				1 => {
					let a = read_value(1);
					let b = read_value(2);
					write_value(&mut self.memory, 3, a + b);
					pc += 4;
				}
				2 => {
					let a = read_value(1);
					let b = read_value(2);
					write_value(&mut self.memory, 3, a * b);
					pc += 4;
				}
				3 => {
					let _ = self.want_input.send(());
					write_value(&mut self.memory, 1, self.input.recv()?);
					pc += 2;
				}
				4 => {
					self.output.send(read_value(1))?;
					pc += 2;
				}
				5 => {
					if read_value(1) != 0 {
						pc = read_value(2) as usize;
					} else {
						pc += 3;
					}
				}
				6 => {
					if read_value(1) == 0 {
						pc = read_value(2) as usize;
					} else {
						pc += 3;
					}
				}
				7 => {
					let result = if read_value(1) < read_value(2) { 1 } else { 0 };
					write_value(&mut self.memory, 3, result);
					pc += 4;
				}
				8 => {
					let result = if read_value(1) == read_value(2) { 1 } else { 0 };
					write_value(&mut self.memory, 3, result);
					pc += 4;
				}
				9 => {
					base += read_value(1);
					pc += 2;
				}
				99 => {
					let _ = self.exit.send(());
					return Ok(());
				}
				_ => {
					return Err(anyhow::anyhow!(
						"Position {} is unknown {}",
						pc,
						self.memory.get(&pc).unwrap_or(&0)
					));
				}
			}
			#[cfg(feature = "video")]
			#[cfg(not(test))]
			video.frame(std::iter::once(
				self.memory
					.iter()
					.enumerate()
					.map(|(i, _)| {
						let i = i as Type;
						use Palette::*;
						if read.contains(&i) {
							if write.contains(&i) {
								RW
							} else {
								Read
							}
						} else if write.contains(&i) {
							Write
						} else if i == pc as Type {
							PC
						} else {
							Other
						}
					})
					.collect(),
			))?;
			#[cfg(feature = "video")]
			{
				read.insert(0);
				write.insert(0);
				std::mem::drop(read);
				std::mem::drop(write);
			}
		}
	}
}

pub fn run_copy(
	program: &[Type],
	input: &[Type],
	_video: Option<&str>,
) -> anyhow::Result<(Vec<Type>, Vec<Type>)> {
	let (tx1, rx1) = crossbeam::channel::unbounded();
	let (tx2, rx2) = crossbeam::channel::unbounded();
	let (txw, _) = crossbeam::channel::unbounded();
	let (txe, _) = crossbeam::channel::unbounded();
	let mut computer = Computer::new(program.to_vec(), rx1, txw, tx2, txe);
	for &i in input {
		tx1.send(i)?;
	}
	computer.run(_video)?;
	let memory = computer.into_memory();
	let output = rx2.into_iter().collect();
	Ok((output, memory))
}
