use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> anyhow::Result<Vec<i32>> {
	let result: Result<Vec<i32>, _> = input.split_whitespace().map(|s| s.parse::<i32>()).collect();
	Ok(result?)
}

fn next(state: &mut Vec<i32>) {
	let mut max = *state.iter().max().unwrap();
	let mut pos = state.iter().position(|el| *el == max).unwrap();
	state[pos] = 0;
	while max > 0 {
		pos = (pos + 1) % state.len();
		state[pos] += 1;
		max -= 1;
	}
}

#[aoc(day6, part1)]
fn part1(input: &[i32]) -> usize {
	let mut current = input.to_owned();
	let mut seen = std::collections::HashSet::<Vec<i32>>::new();
	loop {
		if seen.contains(&current) {
			return seen.len();
		}
		seen.insert(current.clone());
		next(&mut current);
	}
}

#[aoc(day6, part2)]
fn part2(input: &[i32]) -> usize {
	let mut current = input.to_owned();
	let mut seen = std::collections::HashSet::<Vec<i32>>::new();
	loop {
		if seen.contains(&current) {
			return part1(&current);
		}
		seen.insert(current.clone());
		next(&mut current);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = vec![0, 2, 7, 0];
		assert_eq!(part1(&input), 5);
		assert_eq!(part2(&input), 4);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day6.txt")).unwrap();
		assert_eq!(part1(&input), 4074);
		assert_eq!(part2(&input), 2793);
	}
}
