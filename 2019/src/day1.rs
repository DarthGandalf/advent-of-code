use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(freqs: &[i32]) -> i32 {
	freqs.len() as i32
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "1\n3\n-1";

	#[test]
	fn part1_example() {
		let parsed = parse(INPUT).unwrap();
		assert_eq!(part1(&parsed), 33);
	}
}
