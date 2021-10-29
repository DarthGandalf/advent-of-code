use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day7.pest"]
struct Day7Parser;

#[derive(Debug)]
struct Input(std::collections::HashMap<String, (i32, Vec<String>)>);

#[aoc_generator(day7)]
fn parse(input: &str) -> anyhow::Result<Input> {
	let input = Day7Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?;
	let result: anyhow::Result<std::collections::HashMap<String, (i32, Vec<String>)>> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::line)
		.map(|line| -> anyhow::Result<(String, (i32, Vec<String>))> {
			let object = line
				.clone()
				.into_inner()
				.filter(|pair| pair.as_rule() == Rule::object)
				.next()
				.none_err()?
				.as_str()
				.to_string();
			let weight = line
				.clone()
				.into_inner()
				.filter(|pair| pair.as_rule() == Rule::weight)
				.next()
				.none_err()?
				.as_str()
				.parse::<i32>()?;
			let children = line
				.into_inner()
				.filter(|pair| pair.as_rule() == Rule::list)
				.next()
				.map(|list| -> Vec<String> {
					list.into_inner()
						.filter(|pair| pair.as_rule() == Rule::object)
						.map(|obj| obj.as_str().to_string())
						.collect()
				})
				.unwrap_or_default();
			Ok((object, (weight, children)))
		})
		.collect();
	Ok(Input(result?))
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> Option<String> {
	let mut set: std::collections::HashSet<String> = input.0.keys().cloned().collect();
	for i in input.0.values() {
		for j in &i.1 {
			set.remove(j);
		}
	}
	set.into_iter().next()
}

fn calculate(obj: &str, input: &Input, totals: &mut std::collections::HashMap<String, i32>) -> i32 {
	if let Some(result) = totals.get(obj) {
		return *result;
	}
	let (weight, children) = input.0.get(obj).unwrap();
	let mut sum = *weight;
	for child in children {
		sum += calculate(child, input, totals);
	}
	totals.insert(obj.to_string(), sum);
	sum
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> Option<i32> {
	let mut totals = std::collections::HashMap::<String, i32>::new();
	for obj in input.0.keys() {
		calculate(obj, input, &mut totals);
	}
	let (_, diff, bad) = input
		.0
		.values()
		.filter(|(_, children)| children.len() >= 3)
		.filter_map(|(_, children)| {
			let weights: Vec<_> = children
				.iter()
				.map(|child| totals.get(child).unwrap())
				.collect();
			let mut distrib = std::collections::HashMap::<i32, Vec<usize>>::new();
			for (index, w) in weights.iter().enumerate() {
				distrib.entry(**w).or_default().push(index);
			}
			if distrib.len() != 2 {
				return None;
			}
			let mut iter = distrib.iter();
			let opt1 = iter.next().unwrap();
			let opt2 = iter.next().unwrap();
			let good_set;
			let bad_set;
			if opt1.1.len() == 1 {
				good_set = &opt2;
				bad_set = &opt1;
			} else {
				good_set = &opt1;
				bad_set = &opt2;
			}
			let diff = good_set.0 - bad_set.0;
			let bad = bad_set.1.iter().next().unwrap();
			Some((*good_set.0, diff, children[*bad].to_string()))
		})
		.min_by_key(|(total, _, _)| *total)?;
	Some(input.0.get(&bad).unwrap().0 + diff)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), "tknk");
		assert_eq!(part2(&input).unwrap(), 60);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day7.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), "ahnofa");
		assert_eq!(part2(&input).unwrap(), 802);
	}
}
