use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
fn part1(input: &str) -> i32 {
	let mut s: i32 = 0;
	let mut nw: i32 = 0;
	let mut ne: i32 = 0;
	for dir in input.trim().split(",") {
		match dir {
			"s" => s += 1,
			"n" => s -= 1,
			"nw" => nw += 1,
			"se" => nw -= 1,
			"ne" => ne += 1,
			"sw" => ne -= 1,
			_ => unreachable!(),
		}
	}
	((s - nw).abs() + (nw - ne).abs() + (ne - s).abs()) / 2
}

#[aoc(day11, part2)]
fn part2(input: &str) -> i32 {
	let mut s: i32 = 0;
	let mut nw: i32 = 0;
	let mut ne: i32 = 0;
	let mut max: i32 = 0;
	for dir in input.trim().split(",") {
		match dir {
			"s" => s += 1,
			"n" => s -= 1,
			"nw" => nw += 1,
			"se" => nw -= 1,
			"ne" => ne += 1,
			"sw" => ne -= 1,
			_ => unreachable!(),
		}
		max = max.max(((s - nw).abs() + (nw - ne).abs() + (ne - s).abs()) / 2);
	}
	max
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		assert_eq!(part1("ne,ne,ne"), 3);
		assert_eq!(part1("ne,ne,sw,sw"), 0);
		assert_eq!(part1("ne,ne,s,s"), 2);
		assert_eq!(part1("se,sw,se,sw,sw"), 3);
	}

	#[test]
	fn answers() {
		let input = include_str!("../input/2017/day11.txt");
		assert_eq!(part1(&input), 685);
		assert_eq!(part2(&input), 1457);
	}
}
