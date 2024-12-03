use aoc_runner_derive::aoc;

use nom::{
	Err, IResult,
	bytes::complete::{tag, take},
	character::complete::{char, digit1},
	error::{Error, ErrorKind},
	sequence::tuple,
};

fn mul(i: &str) -> IResult<&str, (u32, u32)> {
	let (remainder, (_, a, _, b, _)) =
		tuple((tag("mul("), digit1, char(','), digit1, char(')')))(i)?;
	if a.len() > 3 || b.len() > 3 {
		return Err(Err::Error(Error::new(i, ErrorKind::Tag)));
	}
	Ok((remainder, (a.parse().unwrap(), b.parse().unwrap())))
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
	input
		.match_indices("mul")
		.map(|(x, _)| {
			let y = &input[x..];
			let Ok((_, (a, b))) = mul(y) else { return 0 };
			a * b
		})
		.sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
	0
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
