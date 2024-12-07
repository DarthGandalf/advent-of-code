use anyhow::Result;
use aoc_runner_derive::aoc;

use nom::{
	IResult,
	bytes::complete::tag,
	character::complete::{digit1, space1},
	combinator::{eof, map_res},
	multi::separated_list1,
	sequence::tuple,
};

struct Task {
	operands: Vec<u64>,
	expected: u64,
	concat: bool,
}

fn parseu64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, str::parse)(input)
}

fn attempt(already: u64, index: usize, task: &Task) -> bool {
	if already > task.expected {
		return false;
	}
	let operands = &task.operands;
	if index == operands.len() {
		return already == task.expected;
	}
	attempt(already + operands[index], index + 1, task)
		|| attempt(already * operands[index], index + 1, task)
		|| task.concat
			&& attempt(
				format!("{}{}", already, operands[index]).parse().unwrap(),
				index + 1,
				task,
			)
}

fn solve(input: &str, concat: bool) -> u64 {
	input
		.lines()
		.map(|l| {
			let (expected, _, operands, _) =
				tuple((parseu64, tag(": "), separated_list1(space1, parseu64), eof))(l)
					.unwrap()
					.1;
			let task = Task {
				expected,
				concat,
				operands,
			};
			if attempt(task.operands[0], 1, &task) {
				expected
			} else {
				0
			}
		})
		.sum()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
	solve(input, false)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
	solve(input, true)
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
