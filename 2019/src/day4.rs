use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

struct Input(i32, i32);

#[derive(Parser)]
#[grammar = "day4.pest"]
struct Day4Parser;

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Input, crate::Error> {
	let input = Day4Parser::parse(Rule::file, input)?.next()?;
	let mut input = input.into_inner();
	let min = input.next()?.as_str().parse()?;
	let max = input.next()?.as_str().parse()?;
	Ok(Input(min, max))
}

fn check_pass(x: i32) -> bool {
	let x: Vec<_> = format!("{}", x).chars().collect();
	let mut same = false;
	for w in x.windows(2) {
		let a = w[0];
		let b = w[1];
		if a == b {
			same = true;
		}
		if a > b {
			return false;
		}
	}
	same
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
	(input.0..=input.1).filter(|&x| check_pass(x)).count()
}

fn check_pass2(x: i32) -> bool {
	let x: Vec<_> = format!("{}", x).chars().collect();
	let mut same = std::collections::HashMap::<char, usize>::new();
	for w in x.windows(2) {
		let a = w[0];
		let b = w[1];
		if a == b {
			*same.entry(a).or_default() += 1;
		}
		if a > b {
			return false;
		}
	}
	same.iter().filter(|x| *x.1 == 1).count() > 0
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
	(input.0..=input.1).filter(|&x| check_pass2(x)).count()
}

// > 753

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert!(check_pass(111111));
		assert!(!check_pass(223450));
		assert!(!check_pass(123789));
		assert!(check_pass(112233));
		assert!(check_pass(123444));
		assert!(check_pass(111122));
		assert!(check_pass2(112233));
		assert!(!check_pass2(123444));
		assert!(check_pass2(111122));
	}
}
