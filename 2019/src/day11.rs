use crate::NoneError;
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

trait Robot {
	fn input(&self) -> &crossbeam::channel::Sender<crate::intcode::Type>;
	fn input_signal(&self) -> &crossbeam::channel::Receiver<()>;
	fn output(&self) -> &crossbeam::channel::Receiver<crate::intcode::Type>;
	fn exit(&self) -> &crossbeam::channel::Receiver<()>;
}

struct RealRobot {
	ti: crossbeam::channel::Sender<crate::intcode::Type>,
	rw: crossbeam::channel::Receiver<()>,
	ro: crossbeam::channel::Receiver<crate::intcode::Type>,
	re: crossbeam::channel::Receiver<()>,
}

impl Robot for RealRobot {
	fn input(&self) -> &crossbeam::channel::Sender<crate::intcode::Type> {
		&self.ti
	}

	fn input_signal(&self) -> &crossbeam::channel::Receiver<()> {
		&self.rw
	}

	fn output(&self) -> &crossbeam::channel::Receiver<crate::intcode::Type> {
		&self.ro
	}

	fn exit(&self) -> &crossbeam::channel::Receiver<()> {
		&self.re
	}
}

fn run_robot<R: Robot>(
	r: &R,
	grid: &mut std::collections::HashMap<Position, bool>,
	#[cfg(feature = "video")] video: &mut crate::video::OptionalVideo<Palette>,
	_minx: i32,
	_maxx: i32,
	_miny: i32,
	_maxy: i32,
) -> anyhow::Result<()> {
	let mut pos = Position { x: 0, y: 0 };
	let mut dir = Direction::Up;
	loop {
		crossbeam::channel::select! {
			recv(&r.input_signal()) -> _ => {
				let _ = r.input().send(if grid.get(&pos).cloned().unwrap_or_default() { 1 } else { 0 });
			}
			recv(&r.output()) -> color => {
				*grid.entry(pos.clone()).or_default() = color? == 1;
				dir = if r.output().recv()? == 1 { dir.right() } else { dir.left() };
				pos = dir.forward(&pos);
				#[cfg(feature = "video")]
				video.frame((_miny..=_maxy).map(|y| (_minx..=_maxx).map(|x| {
					if pos.x == x && pos.y == y {
						Palette::Robot
					} else if grid.get(&Position{x,y}).cloned().unwrap_or_default() {
						Palette::White
					} else {
						Palette::Black
					}
				}).collect()))?;
			}
			recv(&r.exit()) -> _ => {
				return Ok(());
			}
		}
	}
}

fn run_real_robot(
	program: &[crate::intcode::Type],
	white: bool,
	_video: Option<&str>,
) -> anyhow::Result<(usize, String, i32, i32, i32, i32)> {
	let (_width, _height, minx, maxx, miny, maxy) = if _video.is_some() {
		#[cfg(feature = "video")]
		let (_, _, minx, maxx, miny, maxy) = run_real_robot(program, white, None)?;
		#[cfg(not(feature = "video"))]
		let (minx, maxx, miny, maxy) = (0, 0, 0, 0);
		let height = maxy - miny + 1;
		let width = maxx - minx + 1;
		(width as u16, height as u16, minx, maxx, miny, maxy)
	} else {
		(0, 0, 0, 0, 0, 0)
	};
	#[cfg(feature = "video")]
	let mut video = crate::video::OptionalVideo::<Palette>::new(
		#[cfg(not(test))]
		_video,
		#[cfg(test)]
		None,
		_width,
		_height,
		1,
	)?;
	let (ti, ri) = crossbeam::channel::bounded(0);
	let (to, ro) = crossbeam::channel::bounded(0);
	let (tw, rw) = crossbeam::channel::bounded(0);
	let (te, re) = crossbeam::channel::bounded(0);
	let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	let mut grid = std::collections::HashMap::<Position, bool>::new();
	grid.insert(Position { x: 0, y: 0 }, white);
	let robot = RealRobot { ti, rw, ro, re };
	run_robot(
		&robot,
		&mut grid,
		#[cfg(feature = "video")]
		&mut video,
		minx,
		maxx,
		miny,
		maxy,
	)?;
	let minx = grid.keys().map(|pos| pos.x).min().none_err()?;
	let maxx = grid.keys().map(|pos| pos.x).max().none_err()?;
	let miny = grid.keys().map(|pos| pos.y).min().none_err()?;
	let maxy = grid.keys().map(|pos| pos.y).max().none_err()?;
	Ok((
		grid.len(),
		itertools::join(
			(miny..=maxy).map(|y| {
				std::iter::once('\n')
					.chain((minx..=maxx).map(|x| {
						if grid.get(&Position { x, y }).cloned().unwrap_or_default() {
							'#'
						} else {
							' '
						}
					}))
					.collect::<String>()
			}),
			"",
		),
		minx,
		maxx,
		miny,
		maxy,
	))
}

palette!(Palette {
	Black = [0x00, 0x00, 0x00],
	White = [0xFF, 0xFF, 0xFF],
	Robot = [0xFF, 0x00, 0x00],
});

#[aoc(day11, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<usize> {
	Ok(run_real_robot(program, false, Some("day11-1"))?.0)
}

#[aoc(day11, part2)]
fn part2(program: &[crate::intcode::Type]) -> anyhow::Result<String> {
	Ok(run_real_robot(program, true, Some("day11-2"))?.1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day11.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 2064);
		assert_eq!(
			part2(&input).unwrap(),
			"
 #    ###  #### #  # #     ##  #  # ###    
 #    #  #    # # #  #    #  # #  # #  #   
 #    #  #   #  ##   #    #    #### #  #   
 #    ###   #   # #  #    # ## #  # ###    
 #    #    #    # #  #    #  # #  # # #    
 #### #    #### #  # ####  ### #  # #  #   "
		);
	}
}
