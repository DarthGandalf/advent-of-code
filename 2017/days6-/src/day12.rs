use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
	input
		.trim()
		.lines()
		.map(|line| {
			let mut x: std::collections::VecDeque<_> = crate::numbers::parse(line).into();
			let start = x.pop_front().unwrap();
			(start, x.into())
		})
		.collect()
}

fn calculate(input: &[(usize, Vec<usize>)]) -> (usize, usize) {
	let mut neigh = fnv::FnvHashMap::<usize, Vec<usize>>::default();
	let mut add_neigh = |a, b| {
		neigh.entry(a).or_default().push(b);
	};
	for (x, nei) in input {
		for y in nei {
			add_neigh(*x, *y);
			add_neigh(*y, *x);
		}
	}
	let nodes: Vec<usize> = neigh.keys().cloned().collect();
	let result =
		pathfinding::undirected::connected_components::connected_components(&nodes, |node| {
			neigh.get(node).unwrap().iter().cloned()
		});
	let indexed = pathfinding::undirected::connected_components::component_index(&result);
	(result[*indexed.get(&0).unwrap()].len(), result.len())
}

#[aoc(day12, part1)]
fn part1(input: &[(usize, Vec<usize>)]) -> usize {
	calculate(input).0
}

#[aoc(day12, part2)]
fn part2(input: &[(usize, Vec<usize>)]) -> usize {
	calculate(input).1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
		);
		assert_eq!(calculate(&input), (6, 2));
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day12.txt"));
		assert_eq!(calculate(&input), (380, 181));
	}
}
