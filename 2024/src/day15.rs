use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
	let mut inputiter = input.split("\n\n");
	let mut m = inputiter
		.next()
		.unwrap()
		.lines()
		.map(|l| l.chars().collect_vec())
		.collect_vec();
	let mut ry = 0;
	let mut rx = 0;
	for (y, row) in m.iter().enumerate() {
		for (x, c) in row.iter().enumerate() {
			if *c == '@' {
				rx = x;
				ry = y;
			}
		}
	}
	m[ry][rx] = '.';
	for c in inputiter.next().unwrap().chars() {
		match c {
			'<' => match m[ry][rx - 1] {
				'.' => rx -= 1,
				'O' => {
					let mut nx = rx - 1;
					while m[ry][nx] == 'O' {
						nx -= 1;
					}
					if m[ry][nx] == '.' {
						rx -= 1;
						m[ry][nx] = 'O';
						m[ry][rx] = '.';
					}
				}
				_ => {}
			},
			'>' => match m[ry][rx + 1] {
				'.' => rx += 1,
				'O' => {
					let mut nx = rx + 1;
					while m[ry][nx] == 'O' {
						nx += 1;
					}
					if m[ry][nx] == '.' {
						rx += 1;
						m[ry][nx] = 'O';
						m[ry][rx] = '.';
					}
				}
				_ => {}
			},
			'^' => match m[ry - 1][rx] {
				'.' => ry -= 1,
				'O' => {
					let mut ny = ry - 1;
					while m[ny][rx] == 'O' {
						ny -= 1;
					}
					if m[ny][rx] == '.' {
						ry -= 1;
						m[ny][rx] = 'O';
						m[ry][rx] = '.';
					}
				}
				_ => {}
			},
			'v' => match m[ry + 1][rx] {
				'.' => ry += 1,
				'O' => {
					let mut ny = ry + 1;
					while m[ny][rx] == 'O' {
						ny += 1;
					}
					if m[ny][rx] == '.' {
						ry += 1;
						m[ny][rx] = 'O';
						m[ry][rx] = '.';
					}
				}
				_ => {}
			},
			_ => {}
		}
		if false {
			for (y, row) in m.iter().enumerate() {
				for (x, c) in row.iter().enumerate() {
					if x == rx && y == ry {
						assert_eq!(*c, '.');
						print!("@");
					} else {
						print!("{c}");
					}
				}
				println!();
			}
		}
	}
	m.into_iter()
		.enumerate()
		.map(|(y, row)| {
			row.into_iter()
				.enumerate()
				.filter(|&(_x, c)| c == 'O')
				.map(|(x, _c)| y * 100 + x)
				.sum::<usize>()
		})
		.sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
	let mut inputiter = input.split("\n\n");
	let mut m = inputiter
		.next()
		.unwrap()
		.lines()
		.map(|l| {
			l.chars()
				.flat_map(|c| match c {
					'#' => "##".chars(),
					'@' => "@.".chars(),
					'O' => "[]".chars(),
					'.' => "..".chars(),
					_ => panic!(),
				})
				.collect_vec()
		})
		.collect_vec();
	let mut ry = 0;
	let mut rx = 0;
	for (y, row) in m.iter().enumerate() {
		for (x, c) in row.iter().enumerate() {
			if *c == '@' {
				rx = x;
				ry = y;
			}
		}
	}
	m[ry][rx] = '.';
	for c in inputiter.next().unwrap().chars() {
		match c {
			'<' => match m[ry][rx - 1] {
				'.' => rx -= 1,
				']' => {
					let mut nx = rx - 1;
					while m[ry][nx] == ']' {
						nx -= 2;
					}
					if m[ry][nx] == '.' {
						for x in nx..rx {
							m[ry][x] = m[ry][x + 1];
						}
						rx -= 1;
					}
				}
				_ => {}
			},
			'>' => match m[ry][rx + 1] {
				'.' => rx += 1,
				'[' => {
					let mut nx = rx + 1;
					while m[ry][nx] == '[' {
						nx += 2;
					}
					if m[ry][nx] == '.' {
						for x in (rx..nx).rev() {
							m[ry][x + 1] = m[ry][x];
						}
						rx += 1;
					}
				}
				_ => {}
			},
			'^' => match m[ry - 1][rx] {
				'.' => ry -= 1,
				'[' | ']' => {
					let ny = ry - 1;
					let mut q = VecDeque::new();
					let mut seen = BTreeSet::new();
					{
						let x = if m[ny][rx] == '[' { rx } else { rx - 1 };
						q.push_back((ny, x));
						seen.insert((ny, x));
					}
					let mut possible = true;
					'loo: while let Some((y, x)) = q.pop_front() {
						for dx in [0, 1] {
							match m[y - 1][x + dx] {
								'[' => {
									if seen.insert((y - 1, x + dx)) {
										q.push_back((y - 1, x + dx));
									}
								}
								']' => {
									if seen.insert((y - 1, x + dx - 1)) {
										q.push_back((y - 1, x + dx - 1));
									}
								}
								'#' => {
									possible = false;
									break 'loo;
								}
								_ => {}
							}
						}
					}
					if possible {
						for (y, x) in seen {
							m[y - 1][x] = '[';
							m[y - 1][x + 1] = ']';
							m[y][x] = '.';
							m[y][x + 1] = '.';
						}
						ry -= 1;
					}
				}
				_ => {}
			},
			'v' => match m[ry + 1][rx] {
				'.' => ry += 1,
				'[' | ']' => {
					let ny = ry + 1;
					let mut q = VecDeque::new();
					let mut seen = BTreeSet::new();
					{
						let x = if m[ny][rx] == '[' { rx } else { rx - 1 };
						q.push_back((ny, x));
						seen.insert((ny, x));
					}
					let mut possible = true;
					'loo: while let Some((y, x)) = q.pop_front() {
						for dx in [0, 1] {
							match m[y + 1][x + dx] {
								'[' => {
									if seen.insert((y + 1, x + dx)) {
										q.push_back((y + 1, x + dx));
									}
								}
								']' => {
									if seen.insert((y + 1, x + dx - 1)) {
										q.push_back((y + 1, x + dx - 1));
									}
								}
								'#' => {
									possible = false;
									break 'loo;
								}
								_ => {}
							}
						}
					}
					if possible {
						for (y, x) in seen.into_iter().rev() {
							m[y + 1][x] = '[';
							m[y + 1][x + 1] = ']';
							m[y][x] = '.';
							m[y][x + 1] = '.';
						}
						ry += 1;
					}
				}
				_ => {}
			},
			_ => {}
		}
		if false {
			for (y, row) in m.iter().enumerate() {
				for (x, c) in row.iter().enumerate() {
					if x == rx && y == ry {
						assert_eq!(*c, '.');
						print!("@");
					} else {
						print!("{c}");
					}
				}
				println!();
			}
		}
	}
	m.into_iter()
		.enumerate()
		.map(|(y, row)| {
			row.into_iter()
				.enumerate()
				.filter(|&(_x, c)| c == '[')
				.map(|(x, _c)| y * 100 + x)
				.sum::<usize>()
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_LARGE: &str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
	.trim_ascii();

	const INPUT_SMALL: &str = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT_SMALL), 2028);
		assert_eq!(part1(INPUT_LARGE), 10092);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT_LARGE), 9021);
	}
}
