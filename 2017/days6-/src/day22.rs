use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Vec<bool>> {
	input
		.trim()
		.lines()
		.map(|line| line.chars().map(|c| c == '#').collect())
		.collect()
}

#[aoc(day22, part1)]
fn part1(input: &[Vec<bool>]) -> usize {
	let mut state = fnv::FnvHashSet::<(i32, i32)>::default();
	for (y, row) in input.iter().enumerate() {
		for (x, infected) in row.iter().enumerate() {
			if *infected {
				state.insert((x as i32, y as i32));
			}
		}
	}
	let mut x = (input.len() / 2) as i32;
	let mut y = x;
	let mut dx = 0;
	let mut dy = -1;
	let mut counter = 0;
	for _ in 0..10000 {
		let nx;
		let ny;
		if state.contains(&(x, y)) {
			nx = -dy;
			ny = dx;
			state.remove(&(x, y));
		} else {
			nx = dy;
			ny = -dx;
			state.insert((x, y));
			counter += 1;
		}
		dx = nx;
		dy = ny;
		x += dx;
		y += dy;
	}
	counter
}

#[aoc(day22, part2)]
fn part2(input: &[Vec<bool>]) -> usize {
	enum Cell {
		C,
		W,
		I,
		F,
	}
	let mut state = fnv::FnvHashMap::<(i32, i32), Cell>::default();
	for (y, row) in input.iter().enumerate() {
		for (x, infected) in row.iter().enumerate() {
			if *infected {
				state.insert((x as i32, y as i32), Cell::I);
			}
		}
	}
	let mut x = (input.len() / 2) as i32;
	let mut y = x;
	let mut dx = 0;
	let mut dy = -1;
	let mut counter = 0;
	for _ in 0..10_000_000 {
		let nx;
		let ny;
		let next;
		match state.get(&(x, y)).unwrap_or(&Cell::C) {
			Cell::C => {
				nx = dy;
				ny = -dx;
				next = Cell::W;
			}
			Cell::W => {
				nx = dx;
				ny = dy;
				next = Cell::I;
				counter += 1;
			}
			Cell::I => {
				nx = -dy;
				ny = dx;
				next = Cell::F;
			}
			Cell::F => {
				nx = -dx;
				ny = -dy;
				next = Cell::C;
			}
		}
		state.insert((x, y), next);
		dx = nx;
		dy = ny;
		x += dx;
		y += dy;
	}
	counter
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
..#
#..
...",
		);
		assert_eq!(part1(&input), 5587);
		assert_eq!(part2(&input), 2511944);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day22.txt"));
		assert_eq!(part1(&input), 5348);
		assert_eq!(part2(&input), 2512225);
	}
}
