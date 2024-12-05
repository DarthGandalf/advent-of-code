use anyhow::Result;
use aoc_runner_derive::aoc;

fn parse(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
	let mut iter = input.split("\n\n");
	(
		iter.next()
			.unwrap()
			.lines()
			.map(|l| {
				let mut it = l.split('|').map(|w| w.parse::<u8>().unwrap());
				(it.next().unwrap(), it.next().unwrap())
			})
			.collect(),
		iter.next()
			.unwrap()
			.lines()
			.map(|l| l.split(',').map(|w| w.parse::<u8>().unwrap()).collect())
			.collect(),
	)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
	let (rules, books) = parse(input);
	let mut sum = 0;
	for book in books {
		let mut map = fnv::FnvHashMap::<u8, u8>::default();
		for (index, page) in book.iter().enumerate() {
			map.entry(*page)
				.and_modify(|_| panic!("duplicate page {page}"))
				.or_insert(index as u8);
		}
		let mut good = true;
		for (a, b) in &rules {
			let aa = map.get(a);
			let bb = map.get(b);
			if aa.is_some() && bb.is_some() && aa.unwrap() > bb.unwrap() {
				good = false;
				break;
			}
		}
		if good {
			sum += book[book.len() / 2] as u32;
		}
	}
	sum
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
	let (rules, books) = parse(input);
	let mut sum = 0;
	for book in books {
		let mut map = fnv::FnvHashMap::<u8, u8>::default();
		for (index, page) in book.iter().enumerate() {
			map.entry(*page)
				.and_modify(|_| panic!("duplicate page {page}"))
				.or_insert(index as u8);
		}
		let mut good = true;
		for (a, b) in &rules {
			let aa = map.get(a);
			let bb = map.get(b);
			if aa.is_some() && bb.is_some() && aa.unwrap() > bb.unwrap() {
				good = false;
				break;
			}
		}
		if !good {
			let mut graph = petgraph::Graph::<u8, u8>::new();
			let nodes: fnv::FnvHashMap<u8, petgraph::graph::NodeIndex> =
				book.iter().map(|p| (*p, graph.add_node(*p))).collect();
			for (a, b) in &rules {
				let aa = nodes.get(a);
				let bb = nodes.get(b);
				if aa.is_some() && bb.is_some() {
					graph.add_edge(*aa.unwrap(), *bb.unwrap(), 1);
				}
			}
			let nodes = petgraph::algo::toposort(&graph, None).unwrap();
			sum += graph[nodes[book.len() / 2]] as u32;
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 143);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 123);
	}
}
