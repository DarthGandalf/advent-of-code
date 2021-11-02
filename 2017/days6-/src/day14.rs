use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	(0..128)
		.map(|row| format!("{}-{}", input.trim(), row))
		.map(|id| crate::day10::part2(&id))
		.map(|hexes| {
			hexes
				.chars()
				.map(|c| i32::from_str_radix(&c.to_string(), 16).unwrap())
				.map(|c| format!("{:04b}", c).chars().collect_vec())
				.flatten()
				.collect_vec()
		})
		.flatten()
		.filter(|&c| c == '1')
		.count()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
	let grid = (0..128)
		.map(|row| format!("{}-{}", input.trim(), row))
		.map(|id| crate::day10::part2(&id))
		.map(|hexes| {
			hexes
				.chars()
				.map(|c| i32::from_str_radix(&c.to_string(), 16).unwrap())
				.map(|c| format!("{:04b}", c).chars().collect_vec())
				.flatten()
				.collect_vec()
		})
		.collect_vec();
	let mut nodes = vec![];
	for y in 0..128i32 {
		for x in 0..128i32 {
			if grid[y as usize][x as usize] == '1' {
				nodes.push((x, y));
			}
		}
	}
	let result =
		pathfinding::undirected::connected_components::connected_components(&nodes, |&(x, y)| {
			vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
				.into_iter()
				.filter(|&(x, y)| x >= 0 && x < 128 && y >= 0 && y < 128)
				.filter(|&(x, y)| grid[y as usize][x as usize] == '1')
		});
	result.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = "flqrgnkx";
		assert_eq!(part1(&input), 8108);
		assert_eq!(part2(&input), 1242);
	}

	#[test]
	fn answers() {
		let input = include_str!("../input/2017/day14.txt");
		assert_eq!(part1(&input), 8316);
		assert_eq!(part2(&input), 1074);
	}
}
