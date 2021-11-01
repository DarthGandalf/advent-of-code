use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashMap;
use pest::Parser;

#[derive(Parser)]
#[grammar = "day8.pest"]
struct Day8Parser;

#[derive(Debug)]
struct Line {
	reg: String,
	delta: i32,
	cond_of: String,
	cond: String,
	cond_with: i32,
}

impl Line {
	fn exec<F: FnMut(i32)>(&self, regs: &mut FnvHashMap<String, i32>, mut cb: F) {
		let value = *regs.get(&self.cond_of).unwrap_or(&0);
		let cond = match &*self.cond {
			"==" => value == self.cond_with,
			"!=" => value != self.cond_with,
			"<" => value < self.cond_with,
			">" => value > self.cond_with,
			"<=" => value <= self.cond_with,
			">=" => value >= self.cond_with,
			_ => unreachable!(),
		};
		if cond {
			let entry = regs.entry(self.reg.clone()).or_default();
			*entry += self.delta;
			cb(*entry);
		}
	}
}

#[derive(Debug)]
struct Input(std::collections::HashMap<String, (i32, Vec<String>)>);

#[aoc_generator(day8)]
fn parse(input: &str) -> anyhow::Result<Vec<Line>> {
	let input = Day8Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?;
	let result: anyhow::Result<Vec<Line>> = input
		.into_inner()
		.map(|line| -> anyhow::Result<_> {
			let mut iter = line.into_inner();
			let reg = iter.next().none_err()?.as_str().to_string();
			let op = if iter.next().none_err()?.as_str() == "inc" {
				1
			} else {
				-1
			};
			let delta = op * iter.next().none_err()?.as_str().parse::<i32>()?;
			let cond_of = iter.next().none_err()?.as_str().to_string();
			let cond = iter.next().none_err()?.as_str().to_string();
			let cond_with = iter.next().none_err()?.as_str().parse::<i32>()?;
			Ok(Line {
				reg,
				delta,
				cond_of,
				cond,
				cond_with,
			})
		})
		.collect();
	result
}

#[aoc(day8, part1)]
fn part1(input: &[Line]) -> i32 {
	let mut regs = FnvHashMap::<String, i32>::default();
	for line in input {
		line.exec(&mut regs, |_| {});
	}
	regs.values().cloned().max().unwrap_or_default()
}

#[aoc(day8, part2)]
fn part2(input: &[Line]) -> i32 {
	let mut regs = FnvHashMap::<String, i32>::default();
	let mut max = 0;
	for line in input {
		line.exec(&mut regs, |x| max = max.max(x));
	}
	max
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
		)
		.unwrap();
		assert_eq!(part1(&input), 1);
		assert_eq!(part2(&input), 10);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day8.txt")).unwrap();
		assert_eq!(part1(&input), 5075);
		assert_eq!(part2(&input), 7310);
	}
}
