use aoc_runner_derive::aoc;
use itertools::Itertools;
use petgraph::visit::{EdgeRef, NodeRef};
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, FromRepr, Default)]
enum Dir {
	#[default]
	E = 0,
	N = 1,
	W = 2,
	S = 3,
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
struct Coord {
	x: u8,
	y: u8,
	d: Dir,
}

impl Coord {
	fn stepfwd(&self) -> Self {
		let mut n = self.clone();
		match self.d {
			Dir::E => n.x += 1,
			Dir::W => n.x -= 1,
			Dir::N => n.y -= 1,
			Dir::S => n.y += 1,
		}
		n
	}
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i64 {
	let m = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
	let mut s = Coord::default();
	let mut ex = 0;
	let mut ey = 0;
	for y in 0..m.len() {
		for x in 0..m[0].len() {
			if m[y][x] == 'S' {
				s.x = x as u8;
				s.y = y as u8;
			}
			if m[y][x] == 'E' {
				ex = x as u8;
				ey = y as u8;
			}
		}
	}
	pathfinding::directed::astar::astar(
		&s,
		|n| {
			let mut v = Vec::with_capacity(3);
			let fwd = n.stepfwd();
			if m[fwd.y as usize][fwd.x as usize] != '#' {
				v.push((fwd, 1i64));
			}
			v.push((
				Coord {
					x: n.x,
					y: n.y,
					d: Dir::from_repr((n.d as usize + 1) % 4).unwrap(),
				},
				1000,
			));
			v.push((
				Coord {
					x: n.x,
					y: n.y,
					d: Dir::from_repr((n.d as usize + 3) % 4).unwrap(),
				},
				1000,
			));
			v
		},
		|n| n.x.abs_diff(ex) as i64 + n.y.abs_diff(ey) as i64,
		|n: &Coord| n.x == ex && n.y == ey,
	)
	.unwrap()
	.1
}

#[aoc(day16, part2)]
pub fn part2(_input: &str) -> u64 {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_1: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
	.trim_ascii();

	const INPUT_2: &str = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT_1), 7036);
		assert_eq!(part1(INPUT_2), 11048);
	}

	#[test]
	fn test2() {}
}
