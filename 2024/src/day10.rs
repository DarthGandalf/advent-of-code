use anyhow::Result;
use aoc_runner_derive::aoc;
use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
struct Coord {
	x: i8,
	y: i8,
}
type Map = FnvHashMap<Coord, i8>;

trait Part {
	type R;
	fn initial(pos: Coord) -> Self::R;
	fn combine(a: Self::R, b: Self::R) -> Self::R;
}

struct Part1;

impl Part for Part1 {
	type R = FnvHashSet<Coord>;

	fn initial(pos: Coord) -> Self::R {
		[pos].into_iter().collect()
	}

	fn combine(a: Self::R, b: Self::R) -> Self::R {
		a.union(&b).cloned().collect()
	}
}

struct Part2;

impl Part for Part2 {
	type R = usize;

	fn initial(_pos: Coord) -> Self::R {
		1
	}

	fn combine(a: Self::R, b: Self::R) -> Self::R {
		a + b
	}
}

fn reachable<P: Part<R: Default>>(pos: Coord, m: &Map) -> P::R {
	let mut r: P::R = Default::default();
	let h = m.get(&pos).unwrap();
	if *h == 9 {
		return P::initial(pos);
	}
	let attempt = |prev: P::R, c: Coord| {
		if let Some(n) = m.get(&c) {
			if *n == h + 1 {
				return P::combine(prev, reachable::<P>(c, &m));
			}
		}
		prev
	};
	r = attempt(r, Coord {
		x: pos.x - 1,
		y: pos.y,
	});
	r = attempt(r, Coord {
		x: pos.x + 1,
		y: pos.y,
	});
	r = attempt(r, Coord {
		x: pos.x,
		y: pos.y - 1,
	});
	r = attempt(r, Coord {
		x: pos.x,
		y: pos.y + 1,
	});
	r
}

// I couldn't find a better way to memoize generic function other than to ungeneric it
#[memoize::memoize(Ignore: m)]
fn reachable1(pos: Coord, m: &Map) -> <Part1 as Part>::R {
	reachable::<Part1>(pos, m)
}

#[memoize::memoize(Ignore: m)]
fn reachable2(pos: Coord, m: &Map) -> <Part2 as Part>::R {
	reachable::<Part2>(pos, m)
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

fn solve(input: &str, f: impl Fn(Coord, &Map) -> usize) -> usize {
	let (m, size) = parse(input);
	let mut sum = 0;
	for y in 0..=size.y {
		for x in 0..=size.x {
			let pos = Coord { x, y };
			if *m.get(&pos).unwrap() == 0 {
				sum += f(pos, &m);
			}
		}
	}
	sum
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
	solve(input, |pos, m| reachable1(pos, m).len())
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
	solve(input, |pos, m| reachable2(pos, m))
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
