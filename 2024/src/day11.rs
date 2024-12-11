use anyhow::Result;
use aoc_runner_derive::aoc;
use fnv::FnvHashMap;

fn blink1(i: i64) -> Vec<i64> {
	if i == 0 {
		vec![1]
	} else {
		let s = format!("{i}");
		if s.len() % 2 == 0 {
			vec![
				s[..s.len() / 2].parse().unwrap(),
				s[s.len() / 2..].parse().unwrap(),
			]
		} else {
			vec![i * 2024]
		}
	}
}

fn blink(x: FnvHashMap<i64, usize>) -> FnvHashMap<i64, usize> {
	let mut n = FnvHashMap::<i64, usize>::default();
	for (item, num) in x {
		for k in blink1(item) {
			*n.entry(k).or_default() += num;
		}
	}
	n
}

fn solve(input: &str, iterations: usize) -> usize {
	let mut x = FnvHashMap::<i64, usize>::default();
	for i in input.split_whitespace().flat_map(str::parse::<i64>) {
		*x.entry(i).or_default() += 1;
	}
	for _ in 0..iterations {
		x = blink(x);
	}
	x.values().sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
	solve(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
	solve(input, 75)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "125 17";

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 55312);
	}

	#[test]
	fn test2() {
	}
}
