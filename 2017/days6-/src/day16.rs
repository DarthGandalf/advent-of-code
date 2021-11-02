use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pest::Parser;
use std::collections::VecDeque;

#[derive(Parser)]
#[grammar = "day16.pest"]
struct Day16Parser;

#[derive(Debug)]
enum Move {
	Spin(usize),
	Exchange(usize, usize),
	Partner(char, char),
}

#[aoc_generator(day16)]
fn parse(input: &str) -> anyhow::Result<Vec<Move>> {
	let input = Day16Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?;
	let result: anyhow::Result<Vec<Move>> = input
		.into_inner()
		.map(|mov| -> anyhow::Result<Move> {
			let mut mov = mov.into_inner();
			let mov = mov.next().unwrap();
			Ok(match mov.as_rule() {
				Rule::spin => Move::Spin(mov.into_inner().as_str().parse()?),
				Rule::exchange => {
					let (a, b) = mov
						.into_inner()
						.map(|num| num.as_str().parse().unwrap())
						.collect_tuple()
						.unwrap();
					Move::Exchange(a, b)
				}
				Rule::partner => {
					let (a, b) = mov
						.into_inner()
						.map(|p| p.as_str().chars().next().unwrap())
						.collect_tuple()
						.unwrap();
					Move::Partner(a, b)
				}
				_ => panic!(),
			})
		})
		.collect();
	result
}

fn dance(input: &[Move], state: &mut VecDeque<char>) {
	for mov in input {
		match *mov {
			Move::Spin(x) => state.rotate_right(x),
			Move::Exchange(x, y) => state.swap(x, y),
			Move::Partner(x, y) => {
				let x = state.iter().position(|&a| a == x).unwrap();
				let y = state.iter().position(|&a| a == y).unwrap();
				state.swap(x, y);
			}
		}
	}
}

fn calculate1(input: &[Move], num: usize) -> String {
	let mut state: VecDeque<char> = ('a'..).take(num).collect();
	dance(input, &mut state);
	state.into_iter().collect()
}

#[aoc(day16, part1)]
fn part1(input: &[Move]) -> String {
	calculate1(input, 16)
}

#[aoc(day16, part2)]
fn part2(input: &[Move]) -> String {
	let mut state: VecDeque<char> = ('a'..).take(16).collect();
	let initial = state.clone();
	let mut period = 0;
	for i in 1.. {
		dance(input, &mut state);
		if state == initial {
			period = i;
			break;
		}
	}
	for _ in 0..1_000_000_000 % period {
		dance(input, &mut state);
	}
	state.into_iter().collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse("s1,x3/4,pe/b").unwrap();
		assert_eq!(calculate1(&input, 5), "baedc");
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day16.txt")).unwrap();
		assert_eq!(part1(&input), "iabmedjhclofgknp");
		assert_eq!(part2(&input), "oildcmfeajhbpngk");
	}
}
