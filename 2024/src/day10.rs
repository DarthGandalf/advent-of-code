use anyhow::Result;
use aoc_runner_derive::aoc;
use fnv::FnvHashSet;

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
struct Coord {
	x: i8,
	y: i8,
}
type Map = Vec<Vec<i8>>;

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
	let h = m[pos.y as usize][pos.x as usize];
	if h == 9 {
		return P::initial(pos);
	}
	let attempt = |prev: P::R, c: Coord| {
		if c.x >= 0 && c.y >= 0 && (c.y as usize) < m.len() && (c.x as usize) < m[0].len() {
			let n = m[c.y as usize][c.x as usize];
			if n == h + 1 {
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

fn parse(input: &str) -> Map {
	input
		.lines()
		.map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
		.collect()
}

fn solve(input: &str, f: impl Fn(Coord, &Map) -> usize) -> usize {
	let m = parse(input);
	let mut sum = 0;
	for y in 0..m.len() {
		for x in 0..m[0].len() {
			let pos = Coord { x: x as i8, y: y as i8 };
			if m[y][x] == 0 {
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
