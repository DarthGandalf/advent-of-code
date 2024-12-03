use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|l| {
			let mut iter = l.split_whitespace().flat_map(|x| x.parse());
			let a: i32 = iter.next().unwrap();
			let mut b: i32 = iter.next().unwrap();
			match a - b {
				1..=3 => {
					for x in iter {
						match b - x {
							1..=3 => {}
							_ => return 0,
						}
						b = x;
					}
				}
				-3..=-1 => {
					for x in iter {
						match b - x {
							-3..=-1 => {}
							_ => return 0,
						}
						b = x;
					}
				}
				_ => {
					return 0;
				}
			}
			1
		})
		.sum()
}

fn try_inc_exact<F>(report: F, len: usize, multiplier: i32) -> bool
where
	F: Fn(usize) -> i32,
{
	for a in 0..len - 1 {
		match (report(a) - report(a + 1)) * multiplier {
			1..=3 => {}
			_ => return false,
		}
	}
	true
}

fn try_inc(report: &[i32], multiplier: i32) -> bool {
	(0..report.len()).any(|i| {
		try_inc_exact(
			|j| {
				if j < i { report[j] } else { report[j + 1] }
			},
			report.len() - 1,
			multiplier,
		)
	})
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
	input
		.lines()
		.map(|l| {
			let report: Vec<i32> = l.split_whitespace().flat_map(|x| x.parse()).collect();

			if try_inc(&report, 1) || try_inc(&report, -1) {
				1
			} else {
				0
			}
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 2);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 4);
	}
}
