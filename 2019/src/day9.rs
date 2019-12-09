use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::run_copy;

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

#[aoc(day9, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	Ok(run_copy(program, &[1], None)?.0[0])
}

#[aoc(day9, part2)]
fn part2(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	Ok(run_copy(program, &[2], None)?.0[0])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let quine = &[
			109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
		];
		assert_eq!(run_copy(quine, &[], None).unwrap().0, quine);
		assert_eq!(
			run_copy(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0], &[], None)
				.unwrap()
				.0,
			&[1219070632396864]
		);
		assert_eq!(
			run_copy(&[104, 1125899906842624, 99], &[], None).unwrap().0,
			&[1125899906842624]
		);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day9.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 2932210790);
		assert_eq!(part2(&input).unwrap(), 73144);
	}
}
