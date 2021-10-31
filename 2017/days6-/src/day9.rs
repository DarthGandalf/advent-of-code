use crate::NoneError;
use aoc_runner_derive::aoc;
use pest::Parser;

#[derive(Parser)]
#[grammar = "day9.pest"]
struct Day9Parser;

fn count1(x: pest::iterators::Pair<Rule>, level: i32) -> (i32, i32) {
	let mut num = 1;
	let mut score = level;
	for group in x
		.into_inner()
		.filter(|pair| pair.as_rule() != Rule::garbage)
	{
		let (num1, score1) = count1(group, level + 1);
		num += num1;
		score += score1;
	}
	(num, score)
}

fn calculate1(input: &str) -> anyhow::Result<(i32, i32)> {
	let tree = Day9Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?
		.into_inner()
		.next()
		.none_err()?;
	let (num, score) = count1(tree, 1);
	Ok((num, score))
}

#[aoc(day9, part1)]
fn part1(input: &str) -> anyhow::Result<i32> {
	Ok(calculate1(input)?.1)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> anyhow::Result<usize> {
	let count = Day9Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?
		.into_inner()
		.flatten()
		.filter(|pair| pair.as_rule() == Rule::good_garbage_symbol)
		.count();

	Ok(count)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(calculate1("{}").unwrap(), (1, 1));
		assert_eq!(calculate1("{{{}}}").unwrap(), (3, 6));
		assert_eq!(calculate1("{{},{}}").unwrap(), (3, 5));
		assert_eq!(calculate1("{{{},{},{{}}}}").unwrap(), (6, 16));
		assert_eq!(calculate1("{<a>,<a>,<a>,<a>}").unwrap(), (1, 1));
		assert_eq!(calculate1("{{<a>},{<a>},{<a>},{<a>}}").unwrap().0, 5);
		assert_eq!(calculate1("{{<!>},{<!>},{<!>},{<a>}}").unwrap().0, 2);
		assert_eq!(calculate1("{{<ab>},{<ab>},{<ab>},{<ab>}}").unwrap().1, 9);
		assert_eq!(calculate1("{{<!!>},{<!!>},{<!!>},{<!!>}}").unwrap().1, 9);
		assert_eq!(calculate1("{{<a!>},{<a!>},{<a!>},{<ab>}}").unwrap().1, 3);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2("{<>}").unwrap(), 0);
		assert_eq!(part2("{<random characters>}").unwrap(), 17);
		assert_eq!(part2("{<<<<>}").unwrap(), 3);
		assert_eq!(part2("{<{!>}>}").unwrap(), 2);
		assert_eq!(part2("{<!!>}").unwrap(), 0);
		assert_eq!(part2("{<!!!>>}").unwrap(), 0);
		assert_eq!(part2(r#"{<{o"i!a,<{i<a>}"#).unwrap(), 10);
	}

	#[test]
	fn answers() {
		let input = include_str!("../input/2017/day9.txt");
		assert_eq!(part1(&input).unwrap(), 12803);
		assert_eq!(part2(&input).unwrap(), 6425);
	}
}
