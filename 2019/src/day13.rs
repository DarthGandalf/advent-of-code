use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

palette!(Palette {
	Empty = [0x00, 0x00, 0x00],
	Wall = [0xFF, 0xFF, 0xFF],
	Block = "block.png",
	Paddle = "paddle.png",
	Ball = "ball.png",
});

#[aoc(day13, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<usize> {
	let mut grid = std::collections::HashMap::new();
	let (_, ri) = crossbeam::channel::bounded(0);
	let (to, ro) = crossbeam::channel::bounded(0);
	let (tw, _) = crossbeam::channel::bounded(0);
	let (te, _) = crossbeam::channel::bounded(0);
	let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	use itertools::Itertools;
	for mut chunk in &ro.into_iter().chunks(3) {
		let x = chunk.next().none_err()?;
		let y = chunk.next().none_err()?;
		let t = chunk.next().none_err()? as u8;
		*grid.entry((x, y)).or_insert(Palette::Empty) = Palette::try_from(t)?;
	}
	Ok(grid.values().filter(|&t| *t == Palette::Block).count())
}

#[aoc(day13, part2)]
fn part2(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	let mut program = program.to_vec();
	program[0] = 2;
	let mut grid = std::collections::HashMap::new();
	let (ti, ri) = crossbeam::channel::bounded(0);
	let (to, ro) = crossbeam::channel::bounded(0);
	let (tw, rw) = crossbeam::channel::bounded(0);
	let (te, re) = crossbeam::channel::bounded(0);
	let mut robot = crate::intcode::Computer::new(program, ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	#[cfg(feature = "video")]
	const WIDTH: u16 = 40;
	#[cfg(feature = "video")]
	const HEIGHT: u16 = 21;
	#[cfg(feature = "video")]
	let mut video = crate::video::OptionalVideo::<Palette>::new(Some("day13"), WIDTH, HEIGHT, 10)?;
	let mut score = 0;
	let mut ballx: crate::intcode::Type = 0;
	let mut paddlex: Option<crate::intcode::Type> = None;
	loop {
		crossbeam::channel::select! {
			recv(rw) -> _ => {
				if let Some(paddlex) = paddlex {
					ti.send((ballx - paddlex).signum())?;
				}
				#[cfg(feature = "video")]
				video.frame((0..HEIGHT as crate::intcode::Type).map(|y| {
					(0..WIDTH as crate::intcode::Type)
						.map(|x| grid.get(&(x, y)).cloned().unwrap_or(Palette::Empty))
						.collect()
				}))?;
			}
			recv(ro) -> x => {
				let x = x?;
				let y = ro.recv()?;
				let t = ro.recv()?;
				if x == -1 && y == 0 {
					score = t;
					continue;
				}
				let t = Palette::try_from(t as u8)?;
				*grid.entry((x, y)).or_insert(Palette::Empty) = t;
				match t {
					Palette::Paddle => paddlex = Some(x),
					Palette::Ball => ballx = x,
					_ => {}
				}
			}
			recv(re) -> _ => {
				return Ok(score);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day13.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 320);
		assert_eq!(part2(&input).unwrap(), 15156);
	}
}
