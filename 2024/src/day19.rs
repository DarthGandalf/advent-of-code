use aoc_runner_derive::aoc;
use fnv::FnvHashMap;
use itertools::Itertools;
use regex::Regex;

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
	let mut iter = input.lines();
	let towels = iter.next().unwrap().split(", ").collect_vec();
	let re = Regex::new(&format!("^(?:{})+$", towels.into_iter().join("|"))).unwrap();
	iter.skip(1).filter(|&s| re.is_match(s)).count()
}

struct A<'a> {
	s: &'a str,
	towels: &'a [&'a str],
	cache: FnvHashMap<usize, usize>,
}

impl A<'_> {
	fn get(&mut self, from: usize) -> usize {
		if from == self.s.len() {
			return 1;
		}
		if let Some(&x) = self.cache.get(&from) {
			return x;
		}
		let x = self
			.towels
			.iter()
			.filter(|&t| from + t.len() <= self.s.len() && &self.s[from..from + t.len()] == *t)
			.map(|t| self.get(from + t.len()))
			.sum();
		self.cache.insert(from, x);
		x
	}
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
	let mut iter = input.lines();
	let towels = iter.next().unwrap().split(", ").collect_vec();
	iter.skip(1)
		.map(|s| {
			let mut a = A {
				towels: &towels,
				s,
				cache: Default::default(),
			};
			a.get(0)
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 6);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 16);
	}
}
