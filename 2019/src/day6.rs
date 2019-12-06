use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day6.pest"]
struct Day6Parser;

#[derive(Debug)]
struct Input(std::collections::HashMap<i32, i32>);

fn parse_num(input: &str) -> Result<i32, std::num::ParseIntError> {
	i32::from_str_radix(input, 36)
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Input, crate::Error> {
	let input = Day6Parser::parse(Rule::input, input)?.next()?;
	let orbits: Result<std::collections::HashMap<i32, i32>, crate::Error> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::orbit)
		.map(|pair| -> Result<(i32, i32), crate::Error> {
			let mut orb = pair.into_inner();
			let center = parse_num(orb.next()?.as_str())?;
			let object = parse_num(orb.next()?.as_str())?;
			Ok((object, center))
		})
		.collect();
	Ok(Input(orbits?))
}

fn distance(input: &Input, what: i32, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
	if let Some(v) = cache.get(&what) {
		return *v;
	}
	let result = if what == 16438
	/* COM */
	{
		0
	} else if let Some(parent) = input.0.get(&what) {
		let result = distance(input, *parent, cache) + 1;
		result
	} else {
		0
	};
	cache.insert(what, result);
	result
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> i32 {
	let mut cache = std::collections::HashMap::new();
	input
		.0
		.keys()
		.map(|&what| distance(input, what, &mut cache))
		.sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> Option<usize> {
	let you = 44958;
	let san = 36671;
	let mut edges = std::collections::HashMap::<i32, Vec<i32>>::new();
	for (&x, &y) in &input.0 {
		edges.entry(x).or_default().push(y);
		edges.entry(y).or_default().push(x);
	}
	pathfinding::prelude::bfs(
		&you,
		|what| edges.get(what).unwrap_or(&Vec::new()).clone(),
		|what| what == &san,
	)
	.map(|path| path.len() - 3)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let input = match parse(
			"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
		) {
			Ok(input) => input,
			Err(err) => panic!("{:?}", err),
		};
		assert_eq!(part1(&input), 42);
	}

	#[test]
	fn test_part2() {
		let input = match parse(
			"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
		) {
			Ok(input) => input,
			Err(err) => panic!("{:?}", err),
		};
		assert_eq!(part2(&input), Some(4));
	}
}
