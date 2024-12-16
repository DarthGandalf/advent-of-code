use aoc_runner_derive::aoc;
use itertools::Itertools;
use petgraph::visit::{EdgeRef, NodeRef};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone, Copy, PartialEq)]
enum Dir {
	E = 0,
	N = 1,
	W = 2,
	S = 3,
}

#[derive(Default)]
struct Coord {
	x: u8,
	y: u8,
	d: u8,
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i64 {
	let m = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
	let mut g =
		petgraph::Graph::<Coord, i64, petgraph::Directed>::new(
		);
	let mi = m
		.iter()
		.enumerate()
		.map(|(y, row)| {
			row.iter()
				.enumerate()
				.map(|(x, c)| {
					Dir::iter()
						.map(|d| {
							g.add_node(Coord {
								x: x as u8,
								y: y as u8,
								d: d as u8,
							})
						})
						.collect_vec()
				})
				.collect_vec()
		})
		.collect_vec();
	let mut s = Coord::default();
	let mut ex = 0;
	let mut ey = 0;
	for y in 0..m.len() {
		for x in 0..m[0].len() {
			if m[y][x] == '#' {
				continue;
			}
			if m[y][x] == 'S' {
				s.x = x as u8;
				s.y = y as u8;
			}
			if m[y][x] == 'E' {
				ex = x as u8;
				ey = y as u8;
			}
			for d in Dir::iter() {
				if m[y][x - 1] != '#' && d == Dir::W {
					g.add_edge(mi[y][x][d as usize], mi[y][x - 1][d as usize], 1);
				}
				if m[y][x + 1] != '#' && d == Dir::E {
					g.add_edge(mi[y][x][d as usize], mi[y][x + 1][d as usize], 1);
				}
				if m[y - 1][x] != '#' && d == Dir::N {
					g.add_edge(mi[y][x][d as usize], mi[y - 1][x][d as usize], 1);
				}
				if m[y + 1][x] != '#' && d == Dir::S {
					g.add_edge(mi[y][x][d as usize], mi[y + 1][x][d as usize], 1);
				}
				g.add_edge(mi[y][x][d as usize], mi[y][x][(d as usize + 1) % 4], 1000);
				g.add_edge(mi[y][x][d as usize], mi[y][x][(d as usize + 3) % 4], 1000);
			}
		}
	}
	petgraph::algo::astar(
		&g,
		mi[s.y as usize][s.x as usize][s.d as usize],
		|n| g[n].x == ex && g[n].y == ey,
		|e| *e.weight(),
		|n| (g[n].x.abs_diff(ex) + g[n].y.abs_diff(ey)).into(),
	).unwrap().0
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
