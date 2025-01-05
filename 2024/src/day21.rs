use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use fnv::FnvHashMap;
use itertools::Itertools;
use regex::Regex;
use strum::{EnumIter, IntoEnumIterator};

fn get_map_by_level(digits: bool) -> FnvHashMap<char, (i8, i8)> {
	if digits {
		[
			('A', (3, 2)),
			('0', (3, 1)),
			('1', (2, 0)),
			('2', (2, 1)),
			('3', (2, 2)),
			('4', (1, 0)),
			('5', (1, 1)),
			('6', (1, 2)),
			('7', (0, 0)),
			('8', (0, 1)),
			('9', (0, 2)),
		]
		.into_iter()
		.collect()
	} else {
		[
			('A', (0, 2)),
			('^', (0, 1)),
			('<', (1, 0)),
			('v', (1, 1)),
			('>', (1, 2)),
		]
		.into_iter()
		.collect()
	}
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos {
	typed: String,
	current: char,
}

fn find_path(
	loc: &FnvHashMap<char, (i8, i8)>,
	grid: &FnvHashMap<(i8, i8), char>,
	cost: &FnvHashMap<char, usize>,
	target: &str,
) -> usize {
	pathfinding::directed::astar::astar(
		&Pos {
			typed: String::new(),
			current: 'A',
		},
		|p| {
			let mut neigh = vec![];
			let (y, x) = *loc.get(&p.current).unwrap();
			let next_typed = format!("{}{}", p.typed, p.current);
			if target.starts_with(&next_typed) {
				neigh.push((
					Pos {
						typed: next_typed,
						current: p.current,
					},
					*cost.get(&'A').unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y - 1, x)) {
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
					},
					*cost.get(&'^').unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y + 1, x)) {
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
					},
					*cost.get(&'v').unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y, x - 1)) {
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
					},
					*cost.get(&'<').unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y, x + 1)) {
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
					},
					*cost.get(&'>').unwrap(),
				));
			}
			neigh
		},
		|_| 0,
		|p| p.typed == target,
	)
	.unwrap()
	.1 as usize
}

fn next_level(
	loc: FnvHashMap<char, (i8, i8)>,
	cost: FnvHashMap<char, usize>,
) -> FnvHashMap<char, usize> {
	let grid: FnvHashMap<(i8, i8), char> = loc.iter().map(|(&a, &b)| (b, a)).collect();
	let mut new_cost = FnvHashMap::<char, usize>::default();
	for &tgt in loc.keys() {
		let target = tgt.to_string();
		new_cost.insert(tgt, find_path(&loc, &grid, &cost, &target));
	}
	new_cost
}

fn keypress(level: i8) -> FnvHashMap<char, usize> {
	if level == 0 {
		return ['A', '^', 'v', '<', '>']
			.into_iter()
			.map(|c| (c, 1))
			.collect();
	}
	let cost = keypress(level - 1);
	let loc: FnvHashMap<char, (i8, i8)> = [
		('A', (0, 2)),
		('^', (0, 1)),
		('<', (1, 0)),
		('v', (1, 1)),
		('>', (1, 2)),
	]
	.into_iter()
	.collect();
	next_level(loc, cost)
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
	let cost = keypress(2);
	let loc = get_map_by_level(true);
	let grid: FnvHashMap<(i8, i8), char> = loc.iter().map(|(&a, &b)| (b, a)).collect();
	dbg!(&cost);
	input
	.lines()
	.map(|l| {
		println!("==={l}");
		let c: usize = l[..3].parse().unwrap();
		find_path(&loc, &grid, &cost, &l[..3]) * c
	})
	.sum()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
029A
980A
179A
456A
379A
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 126384);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 0);
	}
}
