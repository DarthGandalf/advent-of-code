use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
	let mut a = Vec::with_capacity(1500);
	let mut b = Vec::with_capacity(1500);
	for l in input.lines() {
		let mut iter = l.split_whitespace();
		a.push(iter.next().unwrap().parse().unwrap());
		b.push(iter.next().unwrap().parse().unwrap());
	}
	a.sort_unstable();
	b.sort_unstable();
	(a, b)
}

#[aoc(day1, part1)]
fn realpart1(input: &(Vec<i32>, Vec<i32>)) -> u32 {
	let (a, b) = &input;
	std::iter::zip(a, b).map(|(x, y)| x.abs_diff(*y)).sum()
}

#[aoc(day1, part2)]
fn realpart2(input: &(Vec<i32>, Vec<i32>)) -> usize {
	let (a, b) = &input;
	let mut ai = a.chunk_by(|x, y| x == y).peekable();
	let mut bi = b.chunk_by(|x, y| x == y).peekable();
	let mut sum = 0;
	loop {
		let Some(&x) = ai.peek() else {
			break;
		};
		let Some(&y) = bi.peek() else {
			break;
		};
		match x[0].cmp(&y[0]) {
			std::cmp::Ordering::Equal => {
				sum += x[0] as usize * x.len() * y.len();
				ai.next();
				bi.next();
			}
			std::cmp::Ordering::Less => {
				ai.next();
			}
			std::cmp::Ordering::Greater => {
				bi.next();
			}
		}
	}
	sum
}

// for codspeed
pub fn part1(input: &str) -> u32 {
	realpart1(&parse(input))
}
pub fn part2(input: &str) -> usize {
	realpart2(&parse(input))
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 11);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 31);
	}
}
