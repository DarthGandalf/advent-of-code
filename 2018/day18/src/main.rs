use std::collections::HashSet;
use std::time::Instant;

fn step(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut new = vec![vec![' '; grid[0].len()]; grid.len()];
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			let have = |f| {
				let empty = vec![];
				[
					(-1 as i8, -1 as i8),
					(1, -1),
					(1, 1),
					(-1, 1),
					(0, 1),
					(0, -1),
					(-1, 0),
					(1, 0),
				]
				.iter()
				.filter_map(|&(dx, dy)| {
					grid.get((y as i8 + dy) as usize)
						.unwrap_or(&empty)
						.get((x as i8 + dx) as usize)
				})
				.filter(|&&c| c == f)
				.count()
			};
			let trees = have('|');
			let yards = have('#');
			new[y][x] = match grid[y][x] {
				'|' => {
					if yards >= 3 {
						'#'
					} else {
						'|'
					}
				}
				'#' => {
					if yards >= 1 && trees >= 1 {
						'#'
					} else {
						'.'
					}
				}
				'.' => {
					if trees >= 3 {
						'|'
					} else {
						'.'
					}
				}
				_ => panic!("unknown character"),
			}
		}
	}
	new
}

fn value(grid: &Vec<Vec<char>>) -> usize {
	let trees: usize = grid
		.iter()
		.map(|row| row.iter().filter(|&&c| c == '|').count())
		.sum();
	let yards: usize = grid
		.iter()
		.map(|row| row.iter().filter(|&&c| c == '#').count())
		.sum();
	trees * yards
}

fn _solve1(input: &str) -> usize {
	let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	for _ in 0..10 {
		grid = step(grid);
	}
	value(&grid)
}

fn _solve2(input: &str) -> usize {
	let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	for _ in 0..1000 {
		grid = step(grid);
	}
	for _ in 0..(1000000000 - 1000)%28 {
		grid = step(grid);
	}
	if false {
		let mut values = HashSet::new();
		let mut step_done = None;
		for i in 1000.. {
			grid = step(grid);
			//		std::thread::sleep_ms(500);
			let v = value(&grid);
			if values.contains(&v) {
				step_done = Some(i);
				break;
			}
			values.insert(v);
			/*		println!("{}", v);
			for row in &grid {
				println!("{}", row.iter().collect::<String>());
			}*/
		}
		let step_done = step_done.unwrap();
		return values.len();
	}
	value(&grid)
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", _solve2(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(_solve1(include_str!("../example.txt")), 1147);
	}
}
