use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

palette!(Palette {
	Empty = [0x00, 0x00, 0x00],
	Wall = [0xFF, 0xFF, 0xFF],
	Center = [0xFF, 0xFF, 0x00],
	Goal = [0x00, 0xFF, 0xFF],
	Unknown = [0x80, 0x80, 0x80],
	Path = [0xFF, 0x00, 0x00],
});

fn single_path(program: &[crate::intcode::Type], path: &Path) -> anyhow::Result<Palette> {
	let (ti, ri) = crossbeam::channel::unbounded();
	let (to, ro) = crossbeam::channel::unbounded();
	let (tw, _) = crossbeam::channel::unbounded();
	let (te, _) = crossbeam::channel::unbounded();
	let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	for &dir in &path.0 {
		ti.send(dir as crate::intcode::Type)?;
	}
	match ro.into_iter().nth(path.0.len() - 1).none_err()? {
		0 => Ok(Palette::Wall),
		1 => Ok(Palette::Empty),
		2 => Ok(Palette::Goal),
		_ => Err(anyhow::anyhow!("unknown output")),
	}
}

#[derive(num_enum::TryFromPrimitive, Clone, Eq, PartialEq, Hash, Copy, Debug)]
#[repr(u8)]
enum Direction {
	Up = 1,
	Down = 2,
	Left = 3,
	Right = 4,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Path(Vec<Direction>);

impl Path {
	fn position(&self) -> (i32, i32) {
		let mut x = 0;
		let mut y = 0;
		for &dir in &self.0 {
			match dir {
				Direction::Up => y -= 1,
				Direction::Down => y += 1,
				Direction::Left => x -= 1,
				Direction::Right => x += 1,
			}
		}
		(x, y)
	}
}

#[aoc(day15, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<usize> {
	let mut grid = std::collections::HashMap::new();

	// I couldn't figure out how to use bfs() function properly, so this turned
	// out to be a combination of my own BFS implementation and bfs(). The
	// result is predictably slow.
	let path = pathfinding::prelude::bfs(
		&Path(vec![]),
		|path| {
			let mut paths = vec![];
			for dir in 1..=4 {
				let mut path_new = path.clone();
				path_new.0.push(Direction::try_from(dir).unwrap());
				if grid.contains_key(&path_new.position()) {
					continue;
				}
				if let Ok(result) = single_path(program, &path_new) {
					if grid.insert(path_new.position(), result).is_none() {
						if result != Palette::Wall {
							paths.push(path_new);
						}
					}
				}
			}
			paths
		},
		|path| {
			if path.0.is_empty() {
				false
			} else if let Ok(result) = single_path(program, &path) {
				result == Palette::Goal
			} else {
				false
			}
		},
	)
	.none_err()?;

	#[cfg(feature = "video")]
	{
		for p in &path {
			*grid.get_mut(&p.position()).none_err()? = Palette::Path;
		}
		let minx = grid.keys().map(|pos| pos.0).min().none_err()?;
		let maxx = grid.keys().map(|pos| pos.0).max().none_err()?;
		let miny = grid.keys().map(|pos| pos.1).min().none_err()?;
		let maxy = grid.keys().map(|pos| pos.1).max().none_err()?;
		let width = maxx - minx + 1;
		let height = maxy - miny + 1;
		let mut video = crate::video::OptionalVideo::<Palette>::new(
			Some("day15"),
			width as u16,
			height as u16,
			3,
		)?;
		video.frame((miny..=maxy).map(|y| {
			(minx..=maxx)
				.map(|x| {
					let p = grid.get(&(x, y)).cloned().unwrap_or(Palette::Unknown);
					if x == 0 && y == 0 {
						Palette::Center
					} else {
						p
					}
				})
				.collect()
		}))?;
	}
	return Ok(path[path.len() - 1].0.len());
}

#[aoc(day15, part2)]
fn part2(program: &[crate::intcode::Type]) -> anyhow::Result<usize> {
	let mut grid = std::collections::HashMap::new();

	pathfinding::prelude::bfs(
		&Path(vec![]),
		|path| {
			let mut paths = vec![];
			for dir in 1..=4 {
				let mut path_new = path.clone();
				path_new.0.push(Direction::try_from(dir).unwrap());
				if grid.contains_key(&path_new.position()) {
					continue;
				}
				if let Ok(result) = single_path(program, &path_new) {
					if grid.insert(path_new.position(), result).is_none() {
						if result != Palette::Wall {
							paths.push(path_new);
						}
					}
				}
			}
			paths
		},
		|_| false,
	);

	let goal = grid
		.iter()
		.find(|&pos| *pos.1 == Palette::Goal)
		.none_err()?
		.0;

	let mut distances = std::collections::HashMap::new();
	distances.insert(*goal, 0);

	pathfinding::prelude::bfs(
		goal,
		|&pos| {
			let d = distances.get(&pos).cloned().unwrap_or_default();
			vec![
				(pos.0 + 1, pos.1),
				(pos.0 - 1, pos.1),
				(pos.0, pos.1 - 1),
				(pos.0, pos.1 + 1),
			]
			.into_iter()
			.filter(|&p| {
				if distances.contains_key(&p) {
					return false;
				}
				distances.insert(p, d + 1);
				grid.get(&p) != Some(&Palette::Wall)
			})
			.collect::<Vec<_>>()
		},
		|_| false,
	);

	Ok(*distances.values().max().none_err()?)
}

// Due to my bad solution, tests are very slow.
#[cfg(test)]
#[cfg(feature = "false")]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day15.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 242);
		assert_eq!(part2(&input).unwrap(), 276);
	}
}
