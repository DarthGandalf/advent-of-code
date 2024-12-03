use anyhow::Result;
use aoc_runner_derive::aoc;

use nom::{
	Err, IResult,
	branch::alt,
	bytes::complete::tag,
	character::complete::{anychar, char, digit1},
	combinator::value,
	error::{Error, ErrorKind},
	multi::many0,
	sequence::tuple,
};

#[derive(Debug, Clone)]
enum Finding {
	Mul(u32),
	Do,
	Dont,
	Garbage,
}

fn mul(i: &str) -> IResult<&str, Finding> {
	let (remainder, (_, a, _, b, _)) =
		tuple((tag("mul("), digit1, char(','), digit1, char(')')))(i)?;
	if a.len() > 3 || b.len() > 3 {
		// ErrorKind::Tag is wrong, but what I'm supposed to return here?
		return Err(Err::Error(Error::new(i, ErrorKind::Tag)));
	}
	Ok((
		remainder,
		Finding::Mul(a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()),
	))
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
	input
		.match_indices("mul")
		.map(|(x, _)| {
			let y = &input[x..];
			if let Ok((_, Finding::Mul(m))) = mul(y) {
				m
			} else {
				0
			}
		})
		.sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
	let mut skip = false;
	many0(alt((
		mul,
		value(Finding::Do, tag("do()")),
		value(Finding::Dont, tag("don't()")),
		value(Finding::Garbage, anychar),
	)))(input)
	.unwrap()
	.1
	.into_iter()
	.map(|a| match a {
		Finding::Mul(_) if skip => 0,
		Finding::Mul(m) => m,
		Finding::Do => {
			skip = false;
			0
		}
		Finding::Dont => {
			skip = true;
			0
		}
		Finding::Garbage => 0,
	})
	.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		assert_eq!(
			part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
			161
		);
	}

	#[test]
	fn test2() {
		assert_eq!(
			part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
			48
		);
	}
}
