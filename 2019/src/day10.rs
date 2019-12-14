#[cfg(feature = "video")]
use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Default, Copy, Clone, Debug)]
struct Position {
	x: i32,
	y: i32,
}

#[derive(Debug)]
struct Asteroids(Vec<Position>);

#[aoc_generator(day10)]
fn parse(input: &str) -> Asteroids {
	Asteroids(
		input
			.trim()
			.lines()
			.enumerate()
			.flat_map(|(y, row)| {
				row.chars()
					.enumerate()
					.filter(|&(_, c)| c == '#')
					.map(move |(x, _)| Position {
						x: x as i32,
						y: y as i32,
					})
			})
			.collect(),
	)
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Angle {
	xsign: i32,
	ratio: num::rational::Ratio<i32>,
}

impl Angle {
	fn new(x: i32, y: i32) -> Self {
		let xsign = x.signum();
		let ysign = y.signum();
		let ratio = if xsign == 0 {
			num::rational::Ratio::new(ysign, 1)
		} else {
			num::rational::Ratio::new(y, x)
		};
		Angle { xsign, ratio }
	}
}

impl std::cmp::PartialOrd for Angle {
	fn partial_cmp(&self, other: &Angle) -> std::option::Option<std::cmp::Ordering> {
		Some(self.cmp(&other))
	}
}

impl std::cmp::Ord for Angle {
	fn cmp(&self, other: &Angle) -> std::cmp::Ordering {
		// println!("cmp {:?} {:?}", self, other);
		if self == other {
			return std::cmp::Ordering::Equal;
		}
		if self.xsign == 0 && *self.ratio.numer() < 0 {
			return std::cmp::Ordering::Less;
		}
		if other.xsign == 0 && *other.ratio.numer() < 0 {
			return std::cmp::Ordering::Greater;
		}
		if self.xsign > 0 && other.xsign <= 0 {
			return std::cmp::Ordering::Less;
		}
		if other.xsign > 0 && self.xsign <= 0 {
			return std::cmp::Ordering::Greater;
		}
		if self.xsign > 0 && other.xsign > 0 {
			return self.ratio.cmp(&other.ratio);
		}
		if self.xsign == 0 {
			return std::cmp::Ordering::Less;
		}
		if other.xsign == 0 {
			return std::cmp::Ordering::Greater;
		}
		self.ratio.cmp(&other.ratio)
	}
}

fn part1_full(asteroids: &Asteroids) -> (usize, Position) {
	asteroids
		.0
		.iter()
		.map(|pos| {
			let z: std::collections::HashSet<_> = asteroids
				.0
				.iter()
				.filter(|&p| p != pos)
				.map(|p| {
					let x = pos.x - p.x;
					let y = pos.y - p.y;
					Angle::new(x, y)
				})
				.collect();
			(z.len(), *pos)
		})
		.max_by_key(|(count, _)| *count)
		.unwrap_or_default()
}

#[aoc(day10, part1)]
fn part1(asteroids: &Asteroids) -> usize {
	part1_full(asteroids).0
}

#[aoc(day10, part2)]
fn part2(asteroids: &Asteroids) -> anyhow::Result<usize> {
	let station = part1_full(asteroids).1;
	let mut directions: std::collections::BTreeMap<Angle, Vec<Position>> =
		std::collections::BTreeMap::new();
	for (a, p) in asteroids
		.0
		.iter()
		.filter(|&&p| p != station)
		.map(|p| (Angle::new(p.x - station.x, p.y - station.y), p))
	{
		directions.entry(a).or_default().push(*p);
	}
	for line in directions.values_mut() {
		line.sort_by_key(|p| -(p.x - station.x).abs() - (p.y - station.y).abs());
	}
	#[cfg(feature = "video")]
	let height = asteroids.0.iter().map(|p| p.x).max().none_err()? as u16 + 1;
	#[cfg(feature = "video")]
	let width = asteroids.0.iter().map(|p| p.y).max().none_err()? as u16 + 1;
	#[cfg(feature = "video")]
	let mut video = crate::video::OptionalVideo::new(
		#[cfg(not(test))]
		Some("day10"),
		#[cfg(test)]
		None,
		height,
		width,
		5,
	)?;
	#[cfg(feature = "video")]
	let mut grid = vec![vec![Palette::Empty; width as usize]; height as usize];
	#[cfg(feature = "video")]
	{
		for a in &asteroids.0 {
			grid[a.y as usize][a.x as usize] = Palette::Rock;
		}
		grid[station.y as usize][station.x as usize] = Palette::Station;
	}
	let mut index = 0;
	loop {
		for line in directions.values_mut() {
			if let Some(pos) = line.pop() {
				#[cfg(feature = "video")]
				{
					grid[pos.y as usize][pos.x as usize] = Palette::Explosion;
					video.frame(grid.iter().cloned())?;
					grid[pos.y as usize][pos.x as usize] = Palette::Empty;
				}
				index += 1;
				if index == 200 {
					return Ok((pos.x * 100 + pos.y) as usize);
				}
			}
		}
	}
}

palette!(Palette {
	Empty = [0x00, 0x00, 0x00],
	Rock = [0x80, 0x80, 0x00],
	Explosion = [0xFF, 0x00, 0x00],
	Station = [0xFF, 0xFF, 0xFF],
});

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(
			part1_full(&parse(
				"
.#..#
.....
#####
....#
...##",
			)),
			(8, Position { x: 3, y: 4 })
		);
		assert_eq!(
			part1_full(&parse(
				"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
			)),
			(33, Position { x: 5, y: 8 })
		);
		assert_eq!(
			part1_full(&parse(
				"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
			)),
			(35, Position { x: 1, y: 2 })
		);
		assert_eq!(
			part1_full(&parse(
				".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
			)),
			(41, Position { x: 6, y: 3 })
		);
		let big_map = parse(
			".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
		);
		assert_eq!(part1_full(&big_map), (210, Position { x: 11, y: 13 }));
		assert_eq!(part2(&big_map).unwrap(), 802);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day10.txt"));
		assert_eq!(part1(&input), 278);
		assert_eq!(part2(&input).unwrap(), 1417);
	}
}
