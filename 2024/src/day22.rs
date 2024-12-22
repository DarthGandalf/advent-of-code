use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

fn next(x: i64) -> i64 {
	let x = ((x * 64) ^ x) % 16777216;
	let x = ((x / 32) ^ x) % 16777216;
	((x * 2048) ^ x) % 16777216
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
	input
		.lines()
		.map(|l| {
			let mut x = l.parse().unwrap();
			for _ in 0..2000 {
				x = next(x);
			}
			x
		})
		.sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
	let mut result = FnvHashMap::default();
	for l in input.lines() {
		let mut x = l.parse().unwrap();
		let mut diff = VecDeque::with_capacity(4);
		let mut seen = FnvHashSet::default();
		diff.push_back(x % 10);
		for _ in 1..2000 {
			let prev = x;
			x = next(x);
			if diff.len() > 3 {
				diff.pop_front();
			}
			diff.push_back(x % 10 - prev % 10);
			if diff.len() != 4 {
				continue;
			}
			let key = diff.iter().join(",");
			if !seen.contains(&key) {
				seen.insert(key.clone());
				*result.entry(key).or_default() += x % 10;
			}
		}
	}
	*result.values().max().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
1
10
100
2024
"
	.trim_ascii();

	const INPUT_2: &str = "
1
2
3
2024
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 37327623);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT_2), 23);
	}
}
