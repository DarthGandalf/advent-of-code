use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day13)]
fn parse(input: &str) -> std::collections::BTreeMap<i32, i32> {
	crate::numbers::parse(input)
		.into_iter()
		.chunks(2)
		.into_iter()
		.map(|ch| ch.collect_tuple().unwrap())
		.collect()
}

#[aoc(day13, part1)]
fn part1(input: &std::collections::BTreeMap<i32, i32>) -> i32 {
	input
		.iter()
		.filter(|(k, v)| *k % (2 * *v - 2) == 0)
		.map(|(k, v)| *k * *v)
		.sum()
}

#[aoc(day13, part2)]
fn part2(input: &std::collections::BTreeMap<i32, i32>) -> i32 {
	for delay in 0.. {
		if input.iter().all(|(k, v)| (*k + delay) % (2 * *v - 2) != 0) {
			return delay;
		}
	}
	unreachable!();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
0: 3
1: 2
4: 4
6: 4",
		);
		assert_eq!(part1(&input), 24);
		assert_eq!(part2(&input), 10);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day13.txt"));
		assert_eq!(part1(&input), 1528);
		assert_eq!(part2(&input), 3896406);
	}
}
