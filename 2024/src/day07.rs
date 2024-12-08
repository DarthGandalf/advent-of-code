use anyhow::Result;
use aoc_runner_derive::aoc;

use nom::{
	IResult,
	bytes::complete::tag,
	character::complete::{digit1, space1},
	combinator::{eof, map, map_res},
	multi::separated_list1,
	sequence::tuple,
};

struct Task {
	operands: Vec<u64>,
	expected: u64,
	powers: Vec<u64>,
}

fn parseu64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, str::parse)(input)
}

fn parseu64_2(input: &str) -> IResult<&str, (u64, u8)> {
	map(digit1, |s: &str| (s.parse().unwrap(), s.len() as u8))(input)
}

fn attempt(already: u64, index: usize, task: &Task) -> bool {
	if already > task.expected {
		return false;
	}
	let operands = &task.operands;
	if index == task.operands.len() {
		return already == task.expected;
	}
	attempt(already + operands[index], index + 1, task)
		|| attempt(already * operands[index], index + 1, task)
}

fn attempt_2(already: u64, index: usize, task: &Task) -> bool {
	if already > task.expected {
		return false;
	}
	let operands = &task.operands;
	if index == task.operands.len() {
		return already == task.expected;
	}
	attempt_2(already + operands[index], index + 1, task)
		|| attempt_2(already * operands[index], index + 1, task)
		|| attempt_2(
			already * task.powers[index] + operands[index],
			index + 1,
			task,
		)
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
	input
		.lines()
		.map(|l| {
			let (expected, _, operands, _) =
				tuple((parseu64, tag(": "), separated_list1(space1, parseu64), eof))(l)
					.unwrap()
					.1;
			let task = Task {
				expected,
				operands,
				powers: vec![],
			};
			if attempt(task.operands[0], 1, &task) {
				expected
			} else {
				0
			}
		})
		.sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
	input
		.lines()
		.map(|l| {
			let ((expected, _), _, operands, _) = tuple((
				parseu64_2,
				tag(": "),
				separated_list1(space1, parseu64_2),
				eof,
			))(l)
			.unwrap()
			.1;
			let task = Task {
				expected,
				operands: operands.iter().map(|(x, _)| *x).collect(),
				powers: operands.iter().map(|(_, x)| 10u64.pow(*x as u32)).collect(),
			};
			if attempt_2(task.operands[0], 1, &task) {
				expected
			} else {
				0
			}
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 3749);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 11387);
	}
}
