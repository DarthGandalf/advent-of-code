use anyhow::Result;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::{
	collections::{BTreeMap, BTreeSet},
	fmt::Write,
};

#[derive(Debug)]
struct Seq([u8; 5]);

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
	let mut keys = vec![];
	let mut locks = vec![];
	for block in input.split("\n\n") {
		let lines = block.lines().map(|l| l.chars().collect_vec()).collect_vec();
		let first = lines[0][0];
		let mut heights = [0; 5];
		for i in 0..5 {
			heights[i] = lines.iter().take_while(|&c| c[i] == first).count() as u8 - 1;
		}
		if first == '#' {
			keys.push(Seq(heights));
		} else {
			for u in &mut heights {
				*u = lines.len() as u8 - *u - 2;
			}
			locks.push(Seq(heights));
		}
	}
	keys.iter().cartesian_product(&locks).filter(|&(k, l)| (0..5).all(|i| k.0[i] + l.0[i] <= 5)).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 3);
	}
}
