use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

use crate::intcode::run;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.split(',').map(|l| l.parse()).collect()
}

#[aoc(day2, part1)]
fn part1(program: &[i32]) -> Result<i32, crate::Error> {
	let mut program = program.to_vec();
	program[1] = 12;
	program[2] = 2;
	run(&mut program, &[], Some("day2"))?;
	Ok(program[0])
}

#[aoc(day2, part2)]
fn part2(program: &[i32]) -> Result<i32, crate::Error> {
	if let Some(x) = (0..100).into_par_iter().find_map_any(|noun| {
		for verb in 0..100 {
			let mut attempt = program.to_vec();
			attempt[1] = noun;
			attempt[2] = verb;
			if let Ok(_) = run(&mut attempt, &[], None) {
				if attempt[0]
					== #[allow(clippy::inconsistent_digit_grouping)]
					1969_07_20
				{
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
		assert_eq!(run(&mut program, &[], None), Ok(vec![]));
		assert_eq!(program, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}
}
