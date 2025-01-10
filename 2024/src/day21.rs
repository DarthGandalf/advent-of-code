use aoc_runner_derive::aoc;
use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Pos {
	typed: String,
	current: char,
	controlling_robot: char,
}

fn find_path(
	loc: &FnvHashMap<char, (i8, i8)>,
	grid: &FnvHashMap<(i8, i8), char>,
	cost: &FnvHashMap<(char, char), usize>,
	src: char,
	target: &str,
) -> usize {
	pathfinding::directed::astar::astar(
		&Pos {
			typed: String::new(),
			current: src,
			controlling_robot: 'A',
		},
		|p| {
			let mut neigh = vec![];
			let (y, x) = *loc.get(&p.current).unwrap();
			let next_typed = format!("{}{}", p.typed, p.current);
			if target.starts_with(&next_typed) {
				let controlling_robot = 'A';
				neigh.push((
					Pos {
						typed: next_typed,
						current: p.current,
						controlling_robot,
					},
					*cost.get(&(p.controlling_robot, controlling_robot)).unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y - 1, x)) {
				let controlling_robot = '^';
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
						controlling_robot,
					},
					*cost.get(&(p.controlling_robot, controlling_robot)).unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y + 1, x)) {
				let controlling_robot = 'v';
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
						controlling_robot,
					},
					*cost.get(&(p.controlling_robot, controlling_robot)).unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y, x - 1)) {
				let controlling_robot = '<';
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
						controlling_robot,
					},
					*cost.get(&(p.controlling_robot, controlling_robot)).unwrap(),
				));
			}
			if let Some(&n) = grid.get(&(y, x + 1)) {
				let controlling_robot = '>';
				neigh.push((
					Pos {
						typed: p.typed.clone(),
						current: n,
						controlling_robot,
					},
					*cost.get(&(p.controlling_robot, controlling_robot)).unwrap(),
				));
			}
			neigh
		},
		|p| target.len() - p.typed.len(),
		|p| p.typed == target,
	)
	.unwrap()
	.1
}

fn next_level(
	loc: FnvHashMap<char, (i8, i8)>,
	cost: FnvHashMap<(char, char), usize>,
) -> FnvHashMap<(char, char), usize> {
	let grid: FnvHashMap<(i8, i8), char> = loc.iter().map(|(&a, &b)| (b, a)).collect();
	let mut new_cost = FnvHashMap::<(char, char), usize>::default();
	for &src in loc.keys() {
		for &tgt in loc.keys() {
			let target = tgt.to_string();
			new_cost.insert((src, tgt), find_path(&loc, &grid, &cost, src, &target));
		}
	}
	new_cost
}

#[memoize::memoize]
fn keypress(level: i8) -> FnvHashMap<(char, char), usize> {
	if level == 0 {
		let keys = ['A', '^', 'v', '<', '>'];
		return keys
			.iter()
			.cartesian_product(&keys)
			.map(|(&f, &t)| ((f, t), 1))
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

fn solve(input: &str, level: i8) -> usize {
	let cost = keypress(level);
	let loc: FnvHashMap<char, (i8, i8)> = [
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
	.collect();
	let grid: FnvHashMap<(i8, i8), char> = loc.iter().map(|(&a, &b)| (b, a)).collect();
	input
		.lines()
		.map(|l| {
			let c: usize = l[..3].parse().unwrap();
			find_path(&loc, &grid, &cost, 'A', &l) * c
		})
		.sum()
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
	solve(input, 2)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
	solve(input, 25)
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
	fn test2() {}
}
