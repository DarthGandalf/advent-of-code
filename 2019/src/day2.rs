use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.split(',').map(|l| l.parse()).collect()
}

palette!(Palette {
	PC = [0xFF, 0xFF, 0xFF],
	Read = [0x00, 0xFF, 0x00],
	RW = [0xFF, 0xFF, 0x00],
	Write = [0xFF, 0x00, 0x00],
	Other = [0x00, 0x00, 0x00],
});

fn run(program: &mut Vec<i32>, _video: bool) -> Result<(), crate::Error> {
	let mut pc = 0;
	#[cfg(feature = "video")]
	let mut video =
		crate::video::OptionalVideo::<Palette>::new(_video, "day2", program.len() as u16, 1, 10)?;
	loop {
		#[cfg(feature = "video")]
		let mut read = std::collections::HashSet::new();
		#[cfg(feature = "video")]
		let mut write = std::collections::HashSet::new();
		match program[pc] {
			1 => {
				#[cfg(feature = "video")]
				{
					read.insert(program[pc + 1] as usize);
					read.insert(program[pc + 2] as usize);
					write.insert(program[pc + 3] as usize);
				}
				let a = program[program[pc + 1] as usize];
				let b = program[program[pc + 2] as usize];
				let c = program[pc + 3] as usize;
				program[c] = a + b;
				pc += 4;
			}
			2 => {
				#[cfg(feature = "video")]
				{
					read.insert(program[pc + 1] as usize);
					read.insert(program[pc + 2] as usize);
					write.insert(program[pc + 3] as usize);
				}
				let a = program[program[pc + 1] as usize];
				let b = program[program[pc + 2] as usize];
				let c = program[pc + 3] as usize;
				program[c] = a * b;
				pc += 4;
			}
			99 => return Ok(()),
			_ => return Err(format!("Position {} is unknown {}", pc, program[pc]).into()),
		}
		#[cfg(feature = "video")]
		video.frame(
			vec![program
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
				.collect()]
			.iter(),
		)?;
	}
}

#[aoc(day2, part1)]
fn part1(program: &[i32]) -> Result<i32, crate::Error> {
	let mut program = program.to_vec();
	program[1] = 12;
	program[2] = 2;
	run(&mut program, true)?;
	Ok(program[0])
}

#[aoc(day2, part2)]
fn part2(program: &[i32]) -> Result<i32, crate::Error> {
	if let Some(x) = (0..100).into_par_iter().find_map_any(|noun| {
		for verb in 0..100 {
			let mut attempt = program.to_vec();
			attempt[1] = noun;
			attempt[2] = verb;
			if let Ok(()) = run(&mut attempt, false) {
				if attempt[0] == 19690720 {
					return Some(100 * noun + verb);
				}
			}
		}
		None
	}) {
		Ok(x)
	} else {
		Err("No solution".to_string().into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
		assert_eq!(run(&mut program, false), Ok(()));
		assert_eq!(program, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}
}
