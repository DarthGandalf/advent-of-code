use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.split(',').map(|l| l.parse()).collect()
}

fn run(program: &mut Vec<i32>, input: &[i32], _video: bool) -> Result<Vec<i32>, crate::Error> {
	let mut pc = 0;
	let mut input = input.iter();
	let mut output = Vec::new();
	#[cfg(feature = "video")]
	let mut video =
		crate::video::OptionalVideo::<Palette>::new(_video, "day5", program.len() as u16, 1, 10)?;
	loop {
		let opcode = program[pc];
		let read_value = |index| -> i32 {
			let mut mode = opcode / 100;
			//println!("index {}", index);
			for _ in 1..index {
				mode /= 10;
			}
			let value = program[pc + index];
			let result = if mode % 10 > 0 {
				value
			} else {
				program[value as usize]
			};
			//println!("mode {} value={} result={}", mode, value, result);
			result
		};
		//println!("{:?} pc={} output={:?}", program, pc, output);
		match opcode % 100 {
			1 => {
				let a = read_value(1);
				let b = read_value(2);
				let c = program[pc + 3] as usize;
				program[c] = a + b;
				pc += 4;
			}
			2 => {
				let a = read_value(1);
				let b = read_value(2);
				let c = program[pc + 3] as usize;
				program[c] = a * b;
				pc += 4;
			}
			3 => {
				let a = program[pc + 1] as usize;
				program[a] = *input.next()?;
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
				let c = program[pc + 3] as usize;
				if read_value(1) < read_value(2) {
					program[c] = 1;
				} else {
					program[c] = 0;
				}
				pc += 4;
			}
			8 => {
				let c = program[pc + 3] as usize;
				if read_value(1) == read_value(2) {
					program[c] = 1;
				} else {
					program[c] = 0;
				}
				pc += 4;
			}
			99 => return Ok(output),
			_ => return Err(format!("Position {} is unknown {}", pc, program[pc]).into()),
		}
	}
}

fn run_copy(
	program: &[i32],
	input: &[i32],
	_video: bool,
) -> Result<(Vec<i32>, Vec<i32>), crate::Error> {
	let mut program = program.to_vec();
	let output = run(&mut program, input, _video)?;
	Ok((output, program))
}

#[aoc(day5, part1)]
fn part1(program: &[i32]) -> Result<(i32), crate::Error> {
	let output = run_copy(program, &[1], true)?.0;
	Ok(output[output.len() - 1])
}

#[aoc(day5, part2)]
fn part2(program: &[i32]) -> Result<(i32), crate::Error> {
	let output = run_copy(program, &[5], true)?.0;
	Ok(output[output.len() - 1])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(
			run_copy(&[3, 0, 4, 0, 99], &[444], false),
			Ok((vec![444], vec![444, 0, 4, 0, 99]))
		);
		assert_eq!(
			run_copy(&[1002, 4, 3, 4, 33], &[], false),
			Ok((vec![], vec![1002, 4, 3, 4, 99]))
		);
	}

	#[test]
	fn part2_1() {
		let program = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
		assert_eq!(
			run_copy(program, &[8], false),
			Ok((vec![1], vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]))
		);
		assert_eq!(
			run_copy(program, &[7], false),
			Ok((vec![0], vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]))
		);
		let program = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
		assert_eq!(
			run_copy(program, &[8], false),
			Ok((vec![0], vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]))
		);
		assert_eq!(
			run_copy(program, &[7], false),
			Ok((vec![1], vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]))
		);
		let program = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
		assert_eq!(
			run_copy(program, &[8], false),
			Ok((vec![1], vec![3, 3, 1108, 1, 8, 3, 4, 3, 99]))
		);
		assert_eq!(
			run_copy(program, &[7], false),
			Ok((vec![0], vec![3, 3, 1108, 0, 8, 3, 4, 3, 99]))
		);
		let program = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
		assert_eq!(
			run_copy(program, &[8], false),
			Ok((vec![0], vec![3, 3, 1107, 0, 8, 3, 4, 3, 99]))
		);
		assert_eq!(
			run_copy(program, &[7], false),
			Ok((vec![1], vec![3, 3, 1107, 1, 8, 3, 4, 3, 99]))
		);
	}

	#[test]
	fn part2_2() {
		let program = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
		assert_eq!(
			run_copy(program, &[0], false),
			Ok((vec![0], vec![
				3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9
			]))
		);
		assert_eq!(
			run_copy(program, &[1], false),
			Ok((vec![1], vec![
				3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9
			]))
		);
		let program = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
		assert_eq!(
			run_copy(program, &[0], false),
			Ok((vec![0], vec![
				3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0
			]))
		);
		assert_eq!(
			run_copy(program, &[1], false),
			Ok((vec![1], vec![
				3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1
			]))
		);
	}

	#[test]
	fn part2_3() {
		let program = &[
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
			0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
			20, 1105, 1, 46, 98, 99,
		];
		let run = |input| match run_copy(program, &[input], false) {
			Ok((output, _)) => output[0],
			Err(err) => panic!("error {}", err),
		};
		assert_eq!(run(6), 999);
		assert_eq!(run(7), 999);
		assert_eq!(run(8), 1000);
		assert_eq!(run(9), 1001);
		assert_eq!(run(10), 1001);
	}
}
