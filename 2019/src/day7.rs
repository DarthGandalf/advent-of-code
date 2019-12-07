use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::run_copy;

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.split(',').map(|l| l.parse()).collect()
}

#[aoc(day7, part1)]
fn part1(program: &[i32]) -> Result<i32, crate::Error> {
	use fallible_iterator::FallibleIterator;
	let result = fallible_iterator::convert(permute::permutations_of(&[0, 1, 2, 3, 4]).map(
		|x| -> Result<i32, crate::Error> {
			let mut signal = vec![0];
			for &amp in x {
				signal.insert(0, amp);
				let new_signal = run_copy(program, &signal, None)?.0;
				signal = new_signal;
			}
			Ok(signal[0])
		},
	))
	.max()??;
	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(
			part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
			Ok(43210)
		);
		assert_eq!(
			part1(&[
				3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
				23, 99, 0, 0
			]),
			Ok(54321)
		);
		assert_eq!(
			part1(&[
				3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
				1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
			]),
			Ok(65210)
		);
	}
}
