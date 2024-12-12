use anyhow::Result;
use aoc_runner_derive::aoc;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use std::collections::VecDeque;

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
	let mut m: Map = std::iter::once(Vec::new())
		.chain(input.lines().map(|x| {
			std::iter::once('.')
				.chain(x.chars())
				.chain(std::iter::once('.'))
				.collect_vec()
		}))
		.collect();
	m[0] = vec!['.'; m[1].len()];
	m.push(m[0].clone());
	m
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
	let m = parse(input);
	let mut seen = vec![vec![false; m[0].len()]; m.len()];
	let mut q = VecDeque::new();
	let mut result = 0;
	for (y, row) in m.iter().enumerate() {
		for (x, c) in row.iter().enumerate() {
			if c == &'.' {
				continue;
			}
			let mut perimeter = 0;
			let mut area = 1;
			if !seen[y][x] {
				q.push_back((x, y));
				seen[y][x] = true;
			}
			while let Some((x, y)) = q.pop_front() {
				let mut go = |x: usize, y: usize| {
					if m[y][x] != *c {
						perimeter += 1;
					} else if !seen[y][x] {
						q.push_back((x, y));
						seen[y][x] = true;
						area += 1;
					}
				};
				go(x + 1, y);
				go(x - 1, y);
				go(x, y + 1);
				go(x, y - 1);
			}
			result += perimeter * area;
		}
	}
	result
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Direction {
	V,
	H,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct PerimeterElement(usize, usize, Direction, bool);

impl PerimeterElement {
	fn inc(&mut self, dir: bool) {
		match (&self.2, dir) {
			(Direction::V, true) => self.1 += 1,
			(Direction::V, false) => self.1 -= 1,
			(Direction::H, true) => self.0 += 1,
			(Direction::H, false) => self.0 -= 1,
		}
	}
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
	let m = parse(input);
	let mut seen = vec![vec![false; m[0].len()]; m.len()];
	let mut q = VecDeque::new();
	let mut result = 0;
	for (y, row) in m.iter().enumerate() {
		for (x, c) in row.iter().enumerate() {
			if c == &'.' {
				continue;
			}
			let mut perimeter = FnvHashSet::<PerimeterElement>::default();
			let mut area = 1;
			if !seen[y][x] {
				q.push_back((x, y));
				seen[y][x] = true;
			}
			while let Some((x, y)) = q.pop_front() {
				let mut go = |x: usize, y: usize, p: PerimeterElement| {
					if m[y][x] != *c {
						perimeter.insert(p);
					} else if !seen[y][x] {
						q.push_back((x, y));
						seen[y][x] = true;
						area += 1;
					}
				};
				go(x + 1, y, PerimeterElement(x + 1, y, Direction::V, true));
				go(x - 1, y, PerimeterElement(x, y, Direction::V, false));
				go(x, y + 1, PerimeterElement(x, y + 1, Direction::H, true));
				go(x, y - 1, PerimeterElement(x, y, Direction::H, false));
			}
			while let Some(e) = perimeter.iter().next().cloned() {
				perimeter.remove(&e);
				for dir in [false, true] {
					let mut z = e.clone();
					loop {
						z.inc(dir);
						if !perimeter.remove(&z) {
							break;
						}
					}
				}
				result += area;
			}
		}
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_1: &str = "
AAAA
BBCD
BBCC
EEEC
"
	.trim_ascii();

	const INPUT_XO: &str = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"
	.trim_ascii();

	const INPUT_LARGE: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
	.trim_ascii();

	const INPUT_E: &str = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
	.trim_ascii();

	const INPUT_AB: &str = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT_1), 140);
		assert_eq!(part1(INPUT_XO), 772);
		assert_eq!(part1(INPUT_LARGE), 1930);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT_1), 80);
		assert_eq!(part2(INPUT_XO), 436);
		assert_eq!(part2(INPUT_E), 236);
		assert_eq!(part2(INPUT_AB), 368);
		assert_eq!(part2(INPUT_LARGE), 1206);
	}
}
