use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

fn run(program: &[i32], noun: i32, verb: i32, video: Option<&str>) -> anyhow::Result<Vec<i32>> {
	let mut memory = program.to_vec();
	memory[1] = noun;
	memory[2] = verb;
	Ok(crate::intcode::run_copy(&memory, &[], video)?.1)
}

#[aoc(day2, part1)]
fn part1(program: &[i32]) -> anyhow::Result<i32> {
	Ok(run(program, 12, 2, Some("day2"))?[0])
}

#[aoc(day2, part2)]
fn part2(program: &[i32]) -> anyhow::Result<i32> {
	if let Some(x) = (0..100).into_par_iter().find_map_any(|noun| {
		for verb in 0..100 {
			if let Ok(result) = run(program, noun, verb, None) {
				if result[0]
					== #[allow(clippy::inconsistent_digit_grouping)]
					1969_07_20
				{
					return Some(100 * noun + verb);
				}
			}
		}
		None
	}) {
		Ok(x)
	} else {
		Err(anyhow::anyhow!("No solution"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run() {
		let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
		let memory = crate::intcode::run_copy(&program, &[], None).unwrap().1;
		assert_eq!(memory, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day2.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 9581917);
		assert_eq!(part2(&input).unwrap(), 2505);
	}
}
