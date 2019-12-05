palette!(Palette {
	PC = [0xFF, 0xFF, 0xFF],
	Read = [0x00, 0xFF, 0x00],
	RW = [0xFF, 0xFF, 0x00],
	Write = [0xFF, 0x00, 0x00],
	Other = [0x00, 0x00, 0x00],
});

pub fn run(
	program: &mut Vec<i32>,
	input: &[i32],
	_video: Option<&str>,
) -> Result<Vec<i32>, crate::Error> {
	let mut pc = 0;
	let mut input = input.iter();
	let mut output = Vec::new();
	#[cfg(feature = "video")]
	let mut video = crate::video::OptionalVideo::<Palette>::new(_video, program.len() as u16, 1, 10)?;
	loop {
		#[cfg(feature = "video")]
		let mut read = std::collections::HashSet::new();
		#[cfg(feature = "video")]
		let mut write = std::collections::HashSet::new();
		let opcode = program[pc];
		let mut read_value = |index| -> i32 {
			let mut mode = opcode / 100;
			//println!("index {}", index);
			for _ in 1..index {
				mode /= 10;
			}
			let value = program[pc + index];
			let result = if mode % 10 > 0 {
				#[cfg(feature = "video")]
				read.insert(pc + index);
				value
			} else {
				#[cfg(feature = "video")]
				read.insert(value as usize);
				program[value as usize]
			};
			//println!("mode {} value={} result={}", mode, value, result);
			result
		};
		let mut write_value = |program: &mut Vec<i32>, index, value| {
			let pos = program[pc + index] as usize;
			#[cfg(feature = "video")]
			write.insert(pos);
			program[pos] = value;
		};
		//println!("{:?} pc={} output={:?}", program, pc, output);
		match opcode % 100 {
			1 => {
				let a = read_value(1);
				let b = read_value(2);
				write_value(program, 3, a + b);
				pc += 4;
			}
			2 => {
				let a = read_value(1);
				let b = read_value(2);
				write_value(program, 3, a * b);
				pc += 4;
			}
			3 => {
				write_value(program, 1, *input.next()?);
				pc += 2;
			}
			4 => {
				output.push(read_value(1));
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
				write_value(program, 3, result);
				pc += 4;
			}
			8 => {
				let result = if read_value(1) == read_value(2) { 1 } else { 0 };
				write_value(program, 3, result);
				pc += 4;
			}
			99 => return Ok(output),
			_ => return Err(format!("Position {} is unknown {}", pc, program[pc]).into()),
		}
		#[cfg(feature = "video")]
		video.frame(std::iter::once(
			program
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

pub fn run_copy(
	program: &[i32],
	input: &[i32],
	_video: Option<&str>,
) -> Result<(Vec<i32>, Vec<i32>), crate::Error> {
	let mut program = program.to_vec();
	let output = run(&mut program, input, _video)?;
	Ok((output, program))
}
