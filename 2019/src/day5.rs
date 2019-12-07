use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::run_copy;

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

#[aoc(day5, part1)]
fn part1(program: &[i32]) -> Result<i32, crate::Error> {
	let output = run_copy(program, &[1], Some("day5-1"))?.0;
	Ok(output[output.len() - 1])
}

#[aoc(day5, part2)]
fn part2(program: &[i32]) -> Result<i32, crate::Error> {
	let output = run_copy(program, &[5], Some("day5-2"))?.0;
	Ok(output[output.len() - 1])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(
			run_copy(&[3, 0, 4, 0, 99], &[444], None),
			Ok((vec![444], vec![444, 0, 4, 0, 99]))
		);
		assert_eq!(
			run_copy(&[1002, 4, 3, 4, 33], &[], None),
			Ok((vec![], vec![1002, 4, 3, 4, 99]))
		);
	}

	#[test]
	fn part2_1() {
		let program = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
		assert_eq!(
			run_copy(program, &[8], None),
			Ok((vec![1], vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]))
		);
		assert_eq!(
			run_copy(program, &[7], None),
			Ok((vec![0], vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]))
		);
		let program = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
		assert_eq!(
			run_copy(program, &[8], None),
			Ok((vec![0], vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]))
		);
		assert_eq!(
			run_copy(program, &[7], None),
			Ok((vec![1], vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]))
		);
		let program = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
		assert_eq!(
			run_copy(program, &[8], None),
			Ok((vec![1], vec![3, 3, 1108, 1, 8, 3, 4, 3, 99]))
		);
		assert_eq!(
			run_copy(program, &[7], None),
			Ok((vec![0], vec![3, 3, 1108, 0, 8, 3, 4, 3, 99]))
		);
		let program = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
		assert_eq!(
			run_copy(program, &[8], None),
			Ok((vec![0], vec![3, 3, 1107, 0, 8, 3, 4, 3, 99]))
		);
		assert_eq!(
			run_copy(program, &[7], None),
			Ok((vec![1], vec![3, 3, 1107, 1, 8, 3, 4, 3, 99]))
		);
	}

	#[test]
	fn part2_2() {
		let program = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
		assert_eq!(
			run_copy(program, &[0], None),
			Ok((vec![0], vec![
				3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9
			]))
		);
		assert_eq!(
			run_copy(program, &[1], None),
			Ok((vec![1], vec![
				3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9
			]))
		);
		let program = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
		assert_eq!(
			run_copy(program, &[0], None),
			Ok((vec![0], vec![
				3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0
			]))
		);
		assert_eq!(
			run_copy(program, &[1], None),
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
		let run = |input| match run_copy(program, &[input], None) {
			Ok((output, _)) => output[0],
			Err(err) => panic!("error {}", err),
		};
		assert_eq!(run(6), 999);
		assert_eq!(run(7), 999);
		assert_eq!(run(8), 1000);
		assert_eq!(run(9), 1001);
		assert_eq!(run(10), 1001);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day5.txt")).unwrap();
		assert_eq!(part1(&input), Ok(7839346));
		assert_eq!(part2(&input), Ok(447803));
	}
}
