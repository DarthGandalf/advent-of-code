use crate::NoneError;
use aoc_runner_derive::aoc;

#[aoc(day16, part1)]
fn part1(input: &str) -> anyhow::Result<String> {
	let input: anyhow::Result<Vec<u32>> =
		input.chars().map(|c| c.to_digit(10).none_err()).collect();
	let input: Vec<i32> = input?.into_iter().map(|x| x as i32).collect();
	let last = itertools::iterate(input, |previous| {
		previous
			.iter()
			.enumerate()
			.map(|(i, _)| {
				let pattern: Vec<i32> =
					itertools::repeat_n(0, i + 1)
						.chain(itertools::repeat_n(1, i + 1).chain(
							itertools::repeat_n(0, i + 1).chain(itertools::repeat_n(-1, i + 1)),
						))
						.collect();
				let pattern = std::iter::repeat(&pattern).flatten().skip(1);
				let sum: i32 = itertools::zip(previous, pattern).map(|(c, p)| c * p).sum();
				(sum % 10).abs()
			})
			.collect()
	})
	.nth(100)
	.unwrap_or_default();
	last.into_iter()
		.take(8)
		.map(|i| std::char::from_digit(i as u32, 10).none_err())
		.collect()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> anyhow::Result<String> {
	let offset: usize = input[0..7].parse()?;
	let input: anyhow::Result<Vec<u32>> =
		input.chars().map(|c| c.to_digit(10).none_err()).collect();
	let input = input?.into_iter().map(|x| x as i32).collect::<Vec<_>>();
	let input = itertools::repeat_n(&input, 10000)
		.flat_map(|x| x.clone())
		.skip(offset)
		.collect::<Vec<i32>>();
	let last = itertools::iterate(input, |previous| {
		let mut sum: i32 = previous.iter().sum();
		previous
			.iter()
			.map(|&x| {
				sum -= x;
				(sum + x) % 10
			})
			.collect()
	})
	.nth(100)
	.unwrap_or_default();
	last.into_iter()
		.take(8)
		.map(|i| std::char::from_digit(i as u32, 10).none_err())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		assert_eq!(
			part1("80871224585914546619083218645595").unwrap(),
			"24176176"
		);
		assert_eq!(
			part1("19617804207202209144916044189917").unwrap(),
			"73745418"
		);
		assert_eq!(
			part1("69317163492948606335995924319873").unwrap(),
			"52432133"
		);
	}

	#[test]
	fn test2() {
		assert_eq!(
			part2("03036732577212944063491565474664").unwrap(),
			"84462026"
		);
		assert_eq!(
			part2("02935109699940807407585447034323").unwrap(),
			"78725270"
		);
		assert_eq!(
			part2("03081770884921959731165446850517").unwrap(),
			"53553731"
		);
	}

	#[test]
	fn answers() {
		let input = include_str!("../input/2019/day16.txt").trim();
		assert_eq!(part1(&input).unwrap(), "50053207");
		assert_eq!(part2(&input).unwrap(), "32749588");
	}
}
