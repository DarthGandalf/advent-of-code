use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day22.pest"]
struct Day22Parser;

#[derive(Debug)]
enum Step {
	New,
	Cut(i32),
	Increment(i32),
}

#[derive(Debug)]
struct Shuffle(Vec<Step>);

#[aoc_generator(day22)]
fn parse(input: &str) -> anyhow::Result<Shuffle> {
	let input = Day22Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?;
	let steps: anyhow::Result<Vec<Step>> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::step)
		.map(|pair| {
			let step = pair.into_inner().next().none_err()?;
			Ok(match step.as_rule() {
				Rule::new => Step::New,
				Rule::cut => Step::Cut(step.into_inner().next().none_err()?.as_str().parse()?),
				Rule::increment => {
					Step::Increment(step.into_inner().next().none_err()?.as_str().parse()?)
				}
				_ => anyhow::bail!("Unknown parsing rule {}", step),
			})
		})
		.collect();
	Ok(Shuffle(steps?))
}

fn track_card(shuffle: &Shuffle, size: i32, mut which: i32) -> i32 {
	for step in &shuffle.0 {
		which = match step {
			Step::New => size - which - 1,
			Step::Cut(num) => which + size - num,
			Step::Increment(num) => which * num,
		} % size;
	}
	which
}

#[aoc(day22, part1)]
fn part1(shuffle: &Shuffle) -> i32 {
	track_card(shuffle, 10007, 2019)
}

fn inverse(x: num::BigInt, size: &num::BigInt) -> num::BigInt {
	x.modpow(&(size - num::BigInt::from(2)), size)
}

#[aoc(day22, part2)]
fn part2(shuffle: &Shuffle) -> num::BigInt {
	let size: &num::BigInt = &119_315_717_514_047_i64.into();
	let times: &num::BigInt = &101_741_582_076_661_i64.into();
	use num::traits::identities::One;
	let mut off = num::BigInt::default();
	let mut inc = num::BigInt::one();

	for step in shuffle.0.iter() {
		match step {
			Step::New => {
				inc = -inc;
				off += &inc;
			}
			Step::Cut(num) => {
				off += &inc * num::BigInt::from(*num) % size;
			}
			Step::Increment(num) => {
				inc *= inverse(num::BigInt::from(*num), size);
			}
		}
	}
	let incr = inc.modpow(&times, size);
	let offs = off * (num::BigInt::one() - &incr) * inverse(num::BigInt::one() - inc, size);
	(offs + incr * 2020) % size
}

#[cfg(test)]
mod tests {
	use super::*;

	fn check_full(shuffle: &Shuffle, result: &[i32]) {
		let mut deck = vec![0; result.len()];
		for (i, _) in result.iter().enumerate() {
			deck[track_card(&shuffle, result.len() as i32, i as i32) as usize] = i as i32;
		}
		assert_eq!(deck, result);
	}

	#[test]
	fn test1() {
		let shuffle = parse(
			"
deal with increment 7
deal into new stack
deal into new stack",
		)
		.unwrap();
		check_full(&shuffle, &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
	}

	#[test]
	fn test2() {
		let shuffle = parse(
			"
cut 6
deal with increment 7
deal into new stack",
		)
		.unwrap();
		check_full(&shuffle, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
	}

	#[test]
	fn test3() {
		let shuffle = parse(
			"
deal with increment 7
deal with increment 9
cut -2",
		)
		.unwrap();
		check_full(&shuffle, &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
	}

	#[test]
	fn test4() {
		let shuffle = parse(
			"
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
		)
		.unwrap();
		check_full(&shuffle, &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day22.txt")).unwrap();
		assert_eq!(part1(&input), 1510);
		assert_eq!(part2(&input), 10307144922975i64.into());
	}
}
