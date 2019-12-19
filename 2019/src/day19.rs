use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

fn get_pixel(
	program: &[crate::intcode::Type],
	x: crate::intcode::Type,
	y: crate::intcode::Type,
) -> bool {
	let (ti, ri) = crossbeam::channel::unbounded();
	let (to, ro) = crossbeam::channel::unbounded();
	let (tw, _) = crossbeam::channel::unbounded();
	let (te, _) = crossbeam::channel::unbounded();
	let _ = ti.send(x);
	let _ = ti.send(y);
	let mut camera = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	let _ = camera.run(None);
	ro.recv().unwrap_or_default() == 1
}

#[aoc(day19, part1)]
fn part1(program: &[crate::intcode::Type]) -> usize {
	let mut num = 0;
	for y in 0..50 {
		for x in 0..50 {
			let pixel = get_pixel(program, x, y);
			if pixel {
				num += 1;
			}
			//print!("{}", if pixel { '#' } else { '.' });
		}
		//println!();
	}
	num
}

#[aoc(day19, part2)]
fn part2(program: &[crate::intcode::Type]) -> crate::intcode::Type {
	let mut bottom_y = 0;
	for x in 100.. {
		while !get_pixel(program, x, bottom_y) {
			bottom_y += 1;
		}
		while get_pixel(program, x, bottom_y) {
			bottom_y += 1;
		}
		bottom_y -= 1;
		let mut y = bottom_y - 99;
		let right_x = x + 99;
		if !get_pixel(program, right_x, y) {
			continue;
		}
		while get_pixel(program, right_x, y) {
			y -= 1;
		}
		y += 1;
		return x * 10000 + y;
	}
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day19.txt")).unwrap();
		assert_eq!(part1(&input), 150);
		assert_eq!(part2(&input), 12201460);
	}
}
