use anyhow::Result;
use aoc_runner_derive::aoc;
use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
struct Coord {
	x: i8,
	y: i8,
}
type Map = FnvHashMap<Coord, i8>;

#[memoize::memoize(Ignore: m)]
fn reachable(pos: Coord, m: &Map) -> FnvHashSet<Coord> {
	let mut r = FnvHashSet::default();
	let h = m.get(&pos).unwrap();
	if *h == 9 {
		r.insert(pos);
		return r;
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x - 1,
		y: pos.y,
	}) {
		if *n == h + 1 {
			r = r
				.union(&reachable(
					Coord {
						x: pos.x - 1,
						y: pos.y,
					},
					m,
				))
				.cloned()
				.collect();
		}
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x + 1,
		y: pos.y,
	}) {
		if *n == h + 1 {
			r = r
				.union(&reachable(
					Coord {
						x: pos.x + 1,
						y: pos.y,
					},
					m,
				))
				.cloned()
				.collect();
		}
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x,
		y: pos.y - 1,
	}) {
		if *n == h + 1 {
			r = r
				.union(&reachable(
					Coord {
						x: pos.x,
						y: pos.y - 1,
					},
					m,
				))
				.cloned()
				.collect();
		}
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x,
		y: pos.y + 1,
	}) {
		if *n == h + 1 {
			r = r
				.union(&reachable(
					Coord {
						x: pos.x,
						y: pos.y + 1,
					},
					m,
				))
				.cloned()
				.collect();
		}
	}
	r
}

#[memoize::memoize(Ignore: m)]
fn reachable2(pos: Coord, m: &Map) -> usize {
	let mut r = 0;
	let h = m.get(&pos).unwrap();
	if *h == 9 {
		return 1;
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x - 1,
		y: pos.y,
	}) {
		if *n == h + 1 {
			r += reachable2(
				Coord {
					x: pos.x - 1,
					y: pos.y,
				},
				m,
			);
		}
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x + 1,
		y: pos.y,
	}) {
		if *n == h + 1 {
			r += reachable2(
				Coord {
					x: pos.x + 1,
					y: pos.y,
				},
				m,
			);
		}
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x,
		y: pos.y - 1,
	}) {
		if *n == h + 1 {
			r += reachable2(
				Coord {
					x: pos.x,
					y: pos.y - 1,
				},
				m,
			);
		}
	}
	if let Some(n) = m.get(&Coord {
		x: pos.x,
		y: pos.y + 1,
	}) {
		if *n == h + 1 {
			r += reachable2(
				Coord {
					x: pos.x,
					y: pos.y + 1,
				},
				m,
			);
		}
	}
	r
}

fn parse(input: &str) -> (Map, Coord) {
	let mut m = Map::default();
	let mut size = Coord::default();
	for (y, l) in input.lines().enumerate() {
		for (x, c) in l.chars().enumerate() {
			size = Coord {
				x: x as i8,
				y: y as i8,
			};
			m.insert(size, c.to_digit(10).unwrap() as i8);
		}
	}
	(m, size)
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
	let (m, size) = parse(input);
	let mut sum = 0;
	for y in 0..=size.y {
		for x in 0..=size.x {
			let pos = Coord { x, y };
			if *m.get(&pos).unwrap() == 0 {
				sum += reachable(pos, &m).len();
			}
		}
	}
	sum
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
	let (m, size) = parse(input);
	let mut sum = 0;
	for y in 0..=size.y {
		for x in 0..=size.x {
			let pos = Coord { x, y };
			if *m.get(&pos).unwrap() == 0 {
				sum += reachable2(pos, &m);
			}
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 36);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 81);
	}
}
