use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day6.pest"]
struct Day6Parser;

#[derive(Debug)]
struct Input(std::collections::HashMap<i32, i32>);

fn parse_num(input: &str) -> i32 {
	let a = input.chars().nth(0).unwrap_or_default() as i32;
	let b = input.chars().nth(1).unwrap_or_default() as i32;
	let c = input.chars().nth(2).unwrap_or_default() as i32;
	(a * 256 + b) * 256 + c
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Input, crate::Error> {
	let input = Day6Parser::parse(Rule::input, input.trim())?.next()?;
	let orbits: Result<std::collections::HashMap<i32, i32>, crate::Error> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::orbit)
		.map(|pair| -> Result<(i32, i32), crate::Error> {
			let mut orb = pair.into_inner();
			let center = parse_num(orb.next()?.as_str());
			let object = parse_num(orb.next()?.as_str());
			Ok((object, center))
		})
		.collect();
	Ok(Input(orbits?))
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
	let mut cache = std::collections::HashMap::new();
	cache.insert(/* COM */ 4411213, 0);
	input
		.0
		.keys()
		.map(|&what| -> usize {
			let mut x = what;
			let mut stack = Vec::new();
			let dist;
			loop {
				if let Some(y) = cache.get(&x) {
					dist = *y;
					break;
				}
				stack.push(x);
				if let Some(parent) = input.0.get(&x) {
					x = *parent;
				}
			}
			let len = stack.len();
			for (d, z) in stack.into_iter().rev().enumerate() {
				cache.insert(z, dist + d + 1);
			}
			dist + len
		})
		.sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> Option<usize> {
	let you = 5853013;
	let san = 5456206;
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

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day6.txt")).unwrap();
		assert_eq!(part1(&input), 144909);
		assert_eq!(part2(&input), Some(259));
	}
}
