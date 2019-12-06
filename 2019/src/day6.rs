use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day6.pest"]
struct Day6Parser;

#[derive(Debug)]
struct Input(std::collections::HashMap<String, String>);

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Input, crate::Error> {
	let input = Day6Parser::parse(Rule::input, input)?.next()?;
	let orbits: Result<std::collections::HashMap<String, String>, crate::Error> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::orbit)
		.map(|pair| -> Result<(String, String), crate::Error> {
			let mut orb = pair.into_inner();
			let center = orb.next()?.as_str().to_string();
			let object = orb.next()?.as_str().to_string();
			Ok((object, center))
		})
		.collect();
	Ok(Input(orbits?))
}

fn distance(
	input: &Input,
	what: String,
	cache: &mut std::collections::HashMap<String, i32>,
) -> i32 {
	if let Some(v) = cache.get(&what) {
		return *v;
	}
	let result = if what == "COM" {
		0
	} else if let Some(parent) = input.0.get(&what) {
		let result = distance(input, parent.to_string(), cache) + 1;
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
		.map(|what| distance(input, what.to_string(), &mut cache))
		.sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> Option<usize> {
	let mut edges = std::collections::HashMap::<String, Vec<String>>::new();
	for (x, y) in &input.0 {
		edges.entry(x.to_string()).or_default().push(y.to_string());
		edges.entry(y.to_string()).or_default().push(x.to_string());
	}
	pathfinding::prelude::bfs(
		&"YOU".to_string(),
		|what| edges.get(what).unwrap_or(&Vec::new()).clone(),
		|what| what == "SAN",
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
