use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
	x: i32,
	y: i32,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn left(self) -> Direction {
		use Direction::*;
		match self {
			Down => Right,
			Right => Up,
			Up => Left,
			Left => Down,
		}
	}

	fn right(self) -> Direction {
		use Direction::*;
		match self {
			Up => Right,
			Right => Down,
			Down => Left,
			Left => Up,
		}
	}

	fn forward(self, pos: &Position) -> Position {
		use Direction::*;
		match self {
			Up => Position {
				x: pos.x,
				y: pos.y - 1,
			},
			Right => Position {
				x: pos.x + 1,
				y: pos.y,
			},
			Down => Position {
				x: pos.x,
				y: pos.y + 1,
			},
			Left => Position {
				x: pos.x - 1,
				y: pos.y,
			},
		}
	}
}

#[aoc(day11, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<usize> {
	let (ti, ri) = crossbeam::channel::unbounded();
	let (to, ro) = crossbeam::channel::unbounded();
	let (tw, rw) = crossbeam::channel::unbounded();
	let (te, re) = crossbeam::channel::unbounded();
	let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	let mut grid = std::collections::HashMap::<Position, bool>::new();
	//	grid.insert(Position { x: 0, y: 0 }, true);
	let mut pos = Position { x: 0, y: 0 };
	let mut dir = Direction::Up;
	let handle_out = |color,
	                  rot,
	                  grid: &mut std::collections::HashMap<Position, bool>,
	                  pos: &mut Position,
	                  dir: &mut Direction|
	 -> anyhow::Result<()> {
		*grid.entry(pos.clone()).or_default() = color? == 1;
		*dir = if rot? == 1 { dir.right() } else { dir.left() };
		*pos = dir.forward(&pos);
		println!("color={:?} dir={:?} pos={:?}", color, dir, pos);
		Ok(())
	};
	loop {
		crossbeam::channel::select! {
			recv(rw) -> _ => {
				let _ = ti.send(if grid.get(&pos).cloned().unwrap_or_default() { 1 } else { 0 });
			}
			recv(ro) -> color => {
				handle_out(color, ro.recv(), &mut grid, &mut pos, &mut dir)?;
			}
			recv(re) -> _ => {
				while let Ok(x) = ro.recv_timeout(std::time::Duration::from_secs(1)) {
					handle_out(Ok(x), ro.recv(), &mut grid, &mut pos, &mut dir)?;
				}
				println!("{:?}", grid);
				let minx = grid.keys().min_by_key(|pos| pos.x).unwrap().x;
				let maxx = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
				let miny = grid.keys().min_by_key(|pos| pos.y).unwrap().y;
				let maxy = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
				for y in miny..=maxy {
					println!("{}", (minx..=maxx).map(|x| if grid.get(&Position{x,y}).cloned().unwrap_or_default() { '#' } else { ' ' }).collect::<String>());
				}
				return Ok(grid.len());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day11.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 2064);
	}
}
