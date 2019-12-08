palette!(Palette {
	PC = [0xFF, 0xFF, 0xFF],
	Read = [0x00, 0xFF, 0x00],
	RW = [0xFF, 0xFF, 0x00],
	Write = [0xFF, 0x00, 0x00],
	Other = [0x00, 0x00, 0x00],
});

pub type Type = i32;

pub struct Computer {
	memory: Vec<Type>,
	input: std::sync::mpsc::Receiver<Type>,
	output: std::sync::mpsc::Sender<Type>,
}

impl Computer {
	pub fn new(
		memory: Vec<Type>,
		input: std::sync::mpsc::Receiver<Type>,
		output: std::sync::mpsc::Sender<Type>,
	) -> Self {
		Self {
			memory,
			input,
			output,
		}
	}

	pub fn into_memory(self) -> Vec<Type> {
		self.memory
	}

	pub fn run(&mut self, _video: Option<&str>) -> Result<(), crate::Error> {
		let mut pc = 0;
		#[cfg(feature = "video")]
		let mut video =
			crate::video::OptionalVideo::<Palette>::new(_video, self.memory.len() as u16, 1, 10)?;
		loop {
			#[cfg(feature = "video")]
			let mut read = std::collections::HashSet::new();
			#[cfg(feature = "video")]
			let mut write = std::collections::HashSet::new();
			let opcode = self.memory[pc];
			// dummy_read and dummy_write are only to silence warning that read_value and write_value shouldn't be mut while video feature is disabled
			let mut dummy_read = 0;
			let mut read_value = |index| -> Type {
				dummy_read = 0;
				let mut mode = opcode / 100;
				//println!("index {}", index);
				for _ in 1..index {
					mode /= 10;
				}
				let value = self.memory[pc + index];
				if mode % 10 > 0 {
					#[cfg(feature = "video")]
					read.insert(pc + index);
					value
				} else {
					#[cfg(feature = "video")]
					read.insert(value as usize);
					self.memory[value as usize]
				}
				//println!("mode {} value={} result={}", mode, value, result);
			};
			let mut dummy_write = 0;
			let mut write_value = |memory: &mut Vec<Type>, index, value| {
				dummy_write = 0;
				let pos = memory[pc + index] as usize;
				#[cfg(feature = "video")]
				write.insert(pos);
				memory[pos] = value;
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
				99 => return Ok(()),
				_ => return Err(format!("Position {} is unknown {}", pc, self.memory[pc]).into()),
			}
			#[cfg(feature = "video")]
			video.frame(std::iter::once(
				self.memory
					.iter()
					.enumerate()
					.map(|(i, _)| {
						use Palette::*;
						if read.contains(&i) {
							if write.contains(&i) {
								RW
							} else {
								Read
							}
						} else if write.contains(&i) {
							Write
						} else if i == pc {
							PC
						} else {
							Other
						}
					})
					.collect(),
			))?;
		}
	}
}

pub fn run_copy(
	program: &[i32],
	input: &[i32],
	_video: Option<&str>,
) -> Result<(Vec<i32>, Vec<i32>), crate::Error> {
	let (tx1, rx1) = std::sync::mpsc::channel();
	let (tx2, rx2) = std::sync::mpsc::channel();
	let mut computer = Computer::new(program.to_vec(), rx1, tx2);
	for &i in input {
		tx1.send(i)?;
	}
	computer.run(_video)?;
	let memory = computer.into_memory();
	let output = rx2.into_iter().collect();
	Ok((output, memory))
}
