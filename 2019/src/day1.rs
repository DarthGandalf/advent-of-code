use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.lines().map(|l| l.parse()).collect()
}

fn raw_fuel_for(mass: i32) -> i32 {
	mass / 3 - 2
}

fn fuel_for(mass: i32) -> i32 {
	std::cmp::max(raw_fuel_for(mass), 0)
}

fn total_fuel_for(mass: i32) -> i32 {
	std::iter::successors(Some(mass), |&m| {
		let new = raw_fuel_for(m);
		if new <= 0 {
			None
		} else {
			Some(new)
		}
	})
	.skip(1)
	.sum()
}

#[aoc(day1, part1)]
fn part1(masses: &[i32]) -> i32 {
	masses.iter().cloned().map(fuel_for).sum()
}

#[aoc(day1, part2)]
fn part2(masses: &[i32]) -> i32 {
	masses.iter().cloned().map(total_fuel_for).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_fuel() {
		assert_eq!(fuel_for(12), 2);
		assert_eq!(fuel_for(14), 2);
		assert_eq!(fuel_for(1969), 654);
		assert_eq!(fuel_for(100756), 33583);
	}

	#[test]
	fn part2_fuel() {
		assert_eq!(total_fuel_for(12), 2);
		assert_eq!(total_fuel_for(14), 2);
		assert_eq!(total_fuel_for(1969), 966);
		assert_eq!(total_fuel_for(100756), 50346);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day1.txt")).unwrap();
		assert_eq!(part1(&input), 3273715);
		assert_eq!(part2(&input), 4907702);
	}
}
