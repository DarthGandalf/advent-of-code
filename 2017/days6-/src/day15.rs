use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day15)]
fn parse(input: &str) -> (i64, i64) {
	crate::numbers::parse(input)
		.into_iter()
		.collect_tuple()
		.unwrap()
}

#[aoc(day15, part1)]
fn part1(input: &(i64, i64)) -> usize {
	let mut a = input.0;
	let mut b = input.1;
	let mut result = 0;
	let a_mul = 16807;
	let b_mul = 48271;
	for _step in 0..40_000_000 {
		a = a * a_mul % 2147483647;
		b = b * b_mul % 2147483647;
		if (a ^ b) & 0xffff == 0 {
			result += 1;
		}
	}
	result
}

fn gen(start: i64, mul: i64, check: i64) -> impl Iterator<Item = i64> {
	gen_iter::GenIter(move || {
		let mut a = start;
		loop {
			a = a * mul % 2147483647;
			if a % check == 0 {
				yield a;
			}
		}
	})
}

#[aoc(day15, part2)]
fn part2(input: &(i64, i64)) -> usize {
	let gen_a = gen(input.0, 16807, 4);
	let gen_b = gen(input.1, 48271, 8);
	gen_a
		.zip(gen_b)
		.take(5_000_000)
		.filter(|(a, b)| (a ^ b) & 0xffff == 0)
		.count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = (65, 8921);
		assert_eq!(part2(&input), 309);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day15.txt"));
		assert_eq!(part1(&input), 619);
		assert_eq!(part2(&input), 290);
	}
}
