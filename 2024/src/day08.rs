use anyhow::Result;
use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq, Copy)]
struct Coord {
	x: i8,
	y: i8,
}

type Map = fnv::FnvHashMap<char, Vec<Coord>>;

fn parse(input: &str) -> (Map, Coord) {
	let mut map: Map = Map::default();
	let mut size = Coord::default();
	for (y, l) in input.lines().enumerate() {
		for (x, c) in l.chars().enumerate() {
			size = Coord {
				x: x as i8,
				y: y as i8,
			};
			if c != '.' {
				map.entry(c).or_default().push(size);
			}
		}
	}
	(map, size)
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
	let (map, size) = parse(input);
	let mut anti: fnv::FnvHashSet<Coord> = fnv::FnvHashSet::default();
	let mut maybe_add = |u: Coord| {
		if u.x >= 0 && u.y >= 0 && u.x <= size.x && u.y <= size.y {
			anti.insert(u);
		}
	};
	for (_, v) in map {
		for (a, b) in v.into_iter().tuple_combinations() {
			maybe_add(Coord {
				x: a.x * 2 - b.x,
				y: a.y * 2 - b.y,
			});
			maybe_add(Coord {
				x: b.x * 2 - a.x,
				y: b.y * 2 - a.y,
			});
		}
	}
	anti.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
	let (map, size) = parse(input);
	let mut anti: fnv::FnvHashSet<Coord> = fnv::FnvHashSet::default();
	for (_, v) in map {
		for (a, b) in v.into_iter().tuple_combinations() {
			let dx = a.x - b.x;
			let dy = a.y - b.y;
			let mut u = a;
			while u.x >= 0 && u.y >= 0 && u.x <= size.x && u.y <= size.y {
				anti.insert(u);
				u.x += dx;
				u.y += dy;
			}
			u = a;
			while u.x >= 0 && u.y >= 0 && u.x <= size.x && u.y <= size.y {
				anti.insert(u);
				u.x -= dx;
				u.y -= dy;
			}
		}
	}
	anti.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 14);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 34);
	}
}
