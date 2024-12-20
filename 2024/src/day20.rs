use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Default)]
struct Coord {
	x: usize,
	y: usize,
}

struct Input {
	m: Vec<Vec<bool>>,
	start: Coord,
}

fn parse(input: &str) -> Input {
	let mut start = Coord::default();
	let m = input
		.lines()
		.enumerate()
		.map(|(y, l)| {
			l.chars()
				.enumerate()
				.map(|(x, c)| match c {
					'.' | 'E' => true,
					'#' => false,
					'S' => {
						start = Coord { x, y };
						true
					}
					_ => panic!("unknown symbol {c}"),
				})
				.collect_vec()
		})
		.collect_vec();
	Input { m, start }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
	solve(input, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
	solve(input, 20)
}

fn solve(input: &str, cheat: i32) -> usize {
	let input = parse(input);
	let mut dist = vec![vec![-1; input.m[0].len()]; input.m.len()];
	dist[input.start.y][input.start.x] = 0;
	let mut q = vec![input.start];
	let mut r = 0;
	while let Some(a) = q.pop() {
		r += 1;
		for d in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
			let nx = (a.x as i32 + d.0) as usize;
			let ny = (a.y as i32 + d.1) as usize;
			if input.m[ny][nx] && dist[ny][nx] == -1 {
				dist[ny][nx] = r;
				q.push(Coord { x: nx, y: ny });
			}
		}
	}
	let mut z = 0;
	for (y, l) in input.m.iter().enumerate() {
		for (x, c) in l.iter().enumerate() {
			if y > 0 && y < dist.len() - 1 && x > 0 && x < dist[0].len() - 1 && *c {
				for dx in -cheat..=cheat {
					for dy in dx.abs() - cheat..=cheat - dx.abs() {
						let ex = x as i32 + dx;
						let ey = y as i32 + dy;
						if ex > 0
							&& ey > 0 && ex < dist[0].len() as i32 - 1
							&& ey < dist.len() as i32 - 1
							&& dist[ey as usize][ex as usize] >= 0
						{
							let diff =
								dist[ey as usize][ex as usize] - dist[y][x] - dx.abs() - dy.abs();
							if diff >= 100 {
								z += 1;
							}
						}
					}
				}
			}
		}
	}
	z
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		assert_eq!(part1(&include_str!("../input/2024/day20.txt").trim()), 1321);
	}

	#[test]
	fn test2() {
		assert_eq!(
			part2(&include_str!("../input/2024/day20.txt").trim()),
			971737
		);
	}
}
