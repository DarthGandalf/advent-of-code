use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day17)]
fn parse(input: &str) -> usize {
	input.trim().parse().unwrap()
}

#[aoc(day17, part1)]
fn part1(input: &usize) -> usize {
	let mut x = VecDeque::new();
	x.push_back(0);
	for i in 1..=2017 {
		for _ in 0..*input {
			x.rotate_left(1);
		}
		x.push_back(i);
	}
	*x.front().unwrap()
}

#[aoc(day17, part2)]
fn part2(input: &usize) -> usize {
	let mut zp = 0;
	let mut result = 0;
	for i in 1..=50_000_000 {
		zp = (zp + input * i - input) % i;
		if zp == i - 1 {
			result = i;
		}
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = 3;
		assert_eq!(part1(&input), 638);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day17.txt"));
		assert_eq!(part1(&input), 417);
		assert_eq!(part2(&input), 34334221);
	}
}
