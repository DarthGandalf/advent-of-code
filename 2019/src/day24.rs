use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Default)]
struct Grid([[bool; 5]; 5]);

impl Grid {
	fn diversity(&self) -> i32 {
		let mut sum = 0;
		let mut multiplier = 1;
		for row in &self.0 {
			for &col in row {
				if col {
					sum += multiplier;
				}
				multiplier *= 2;
			}
		}
		sum
	}

	fn next(&self) -> Self {
		let mut new: Self = Default::default();
		for y in 0..5 {
			for x in 0..5 {
				let mut num = 0;
				if x > 0 && self.0[y][x - 1] {
					num += 1;
				}
				if x < 4 && self.0[y][x + 1] {
					num += 1;
				}
				if y > 0 && self.0[y - 1][x] {
					num += 1;
				}
				if y < 4 && self.0[y + 1][x] {
					num += 1;
				}
				new.0[y][x] = if self.0[y][x] {
					num == 1
				} else {
					num == 1 || num == 2
				};
			}
		}
		new
	}
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Grid {
	use std::convert::TryInto;
	let row: Vec<[bool; 5]> = input
		.lines()
		.map(|line| {
			let row: Vec<bool> = line.chars().map(|c| c == '#').collect();
			row.as_slice().try_into().unwrap_or_default()
		})
		.collect();
	Grid(row.as_slice().try_into().unwrap_or_default())
}

#[aoc(day24, part1)]
fn part1(input: &Grid) -> i32 {
	let mut seen = fnv::FnvHashSet::<i32>::default();
	seen.insert(input.diversity());
	let mut grid = input.clone();
	loop {
		grid = grid.next();
		let div = grid.diversity();
		if seen.contains(&div) {
			return div;
		}
		seen.insert(div);
	}
}

#[derive(Clone, Default)]
struct MultiGrid(fnv::FnvHashMap<i32, Grid>);

impl MultiGrid {
	fn new(grid: Grid) -> Self {
		let mut map: fnv::FnvHashMap<i32, Grid> = Default::default();
		map.insert(0, grid);
		MultiGrid(map)
	}

	fn get(&self, depth: i32, y: usize, x: usize) -> usize {
		if let Some(g) = self.0.get(&depth) {
			if g.0[y][x] {
				1
			} else {
				0
			}
		} else {
			0
		}
	}

	fn adjnum(&self, d: i32, y: usize, x: usize) -> usize {
		let mut sum = 0;
		if y == 0 {
			sum += self.get(d - 1, 1, 2);
		} else {
			sum += self.get(d, y - 1, x);
		}
		if y == 4 {
			sum += self.get(d - 1, 3, 2);
		} else {
			sum += self.get(d, y + 1, x);
		}
		if x == 0 {
			sum += self.get(d - 1, 2, 1);
		} else {
			sum += self.get(d, y, x - 1);
		}
		if x == 4 {
			sum += self.get(d - 1, 2, 3);
		} else {
			sum += self.get(d, y, x + 1);
		}
		match (y, x) {
			(1, 2) => {
				sum += (0..5).map(|xx| self.get(d + 1, 0, xx)).sum::<usize>();
			}
			(2, 1) => {
				sum += (0..5).map(|yy| self.get(d + 1, yy, 0)).sum::<usize>();
			}
			(3, 2) => {
				sum += (0..5).map(|xx| self.get(d + 1, 4, xx)).sum::<usize>();
			}
			(2, 3) => {
				sum += (0..5).map(|yy| self.get(d + 1, yy, 4)).sum::<usize>();
			}
			_ => {}
		}
		sum
	}

	fn next(&self) -> Self {
		let mut new: Self = Default::default();
		let min = self.0.keys().min().unwrap_or(&0);
		let max = self.0.keys().max().unwrap_or(&0);
		for depth in min - 1..=max + 1 {
			let mut newg: Grid = Default::default();
			let mut add = false;
			for y in 0..5 {
				for x in 0..5 {
					if x != 2 || y != 2 {
						let num = self.adjnum(depth, y, x);
						newg.0[y][x] = if self.get(depth, y, x) == 1 {
							num == 1
						} else {
							num == 1 || num == 2
						};
						if newg.0[y][x] {
							add = true;
						}
					}
				}
			}
			if add {
				new.0.insert(depth, newg);
			}
		}
		new
	}

	fn count(&self) -> usize {
		let mut sum = 0;
		for &d in self.0.keys() {
			for y in 0..5 {
				for x in 0..5 {
					sum += self.get(d, y, x);
				}
			}
		}
		sum
	}
}

fn part2_num(input: &Grid, num: usize) -> usize {
	let mut grid = MultiGrid::new(input.clone());
	for _ in 0..num {
		grid = grid.next();
	}
	grid.count()
}

#[aoc(day24, part2)]
fn part2(input: &Grid) -> usize {
	part2_num(input, 200)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn diversity() {
		assert_eq!(
			Grid([
				[false, false, false, false, false],
				[false, false, false, false, false],
				[false, false, false, false, false],
				[true, false, false, false, false],
				[false, true, false, false, false],
			])
			.diversity(),
			2129920
		);
	}

	#[test]
	fn test1() {
		let input = parse(
			"
....#
#..#.
#..##
..#..
#...."
				.trim_start_matches('\n'),
		);
		assert_eq!(part1(&input), 2129920);
		assert_eq!(part2_num(&input, 10), 99);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day24.txt"));
		assert_eq!(part1(&input), 32506911);
		assert_eq!(part2(&input), 2025);
	}
}
