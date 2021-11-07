use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Rule {
	src: Vec<bool>,
	dst: Vec<bool>,
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<Rule> {
	input
		.trim()
		.lines()
		.map(|line| {
			let (src, dst) = line.split(" => ").collect_tuple().unwrap();
			let src = src
				.chars()
				.filter(|&c| c != '/')
				.map(|c| c == '#')
				.collect();
			let dst = dst
				.chars()
				.filter(|&c| c != '/')
				.map(|c| c == '#')
				.collect();
			Rule { src, dst }
		})
		.collect()
}

fn flips(s: &[bool]) -> Vec<Vec<bool>> {
	if s.len() == 4 {
		vec![
			// 0 1
			// 2 3
			vec![s[0], s[1], s[2], s[3]],
			vec![s[0], s[2], s[1], s[3]],
			vec![s[1], s[0], s[3], s[2]],
			vec![s[1], s[3], s[0], s[2]],
			vec![s[2], s[0], s[3], s[1]],
			vec![s[2], s[3], s[0], s[1]],
			vec![s[3], s[1], s[2], s[0]],
			vec![s[3], s[2], s[1], s[0]],
		]
	} else {
		vec![
			// 0 1 2
			// 3 4 5
			// 6 7 8
			vec![s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8]],
			vec![s[0], s[3], s[6], s[1], s[4], s[7], s[2], s[5], s[8]],
			vec![s[2], s[1], s[0], s[5], s[4], s[3], s[8], s[7], s[6]],
			vec![s[2], s[5], s[8], s[1], s[4], s[7], s[0], s[3], s[6]],
			vec![s[6], s[7], s[8], s[3], s[4], s[5], s[0], s[1], s[2]],
			vec![s[6], s[3], s[0], s[7], s[4], s[1], s[8], s[5], s[2]],
			vec![s[8], s[7], s[6], s[5], s[4], s[3], s[2], s[1], s[0]],
			vec![s[8], s[5], s[2], s[7], s[4], s[1], s[6], s[3], s[0]],
		]
	}
}

fn calculate(input: &[Rule], n: usize) -> usize {
	let mut rules = fnv::FnvHashMap::<Vec<bool>, Vec<bool>>::default();
	for rule in input {
		for f in flips(&rule.src) {
			rules.insert(f, rule.dst.clone());
		}
	}
	let mut state: Vec<Vec<bool>> = vec![vec![false, true, false], vec![false, false, true], vec![
		true, true, true,
	]];
	for _ in 0..n {
		let old_side;
		let new_side;
		if state.len() % 2 == 0 {
			old_side = 2;
			new_side = 3;
		} else {
			old_side = 3;
			new_side = 4;
		}
		let new_size = state.len() * new_side / old_side;
		let mut new_state = vec![vec![false; new_size]; new_size];
		for row in 0..state.len() / old_side {
			for col in 0..state.len() / old_side {
				let mut piece = vec![];
				for y in 0..old_side {
					for x in 0..old_side {
						piece.push(state[row * old_side + y][col * old_side + x]);
					}
				}
				let piece = rules.get(&piece).unwrap();
				let mut counter = 0;
				for y in 0..new_side {
					for x in 0..new_side {
						new_state[row * new_side + y][col * new_side + x] = piece[counter];
						counter += 1;
					}
				}
			}
		}
		state = new_state;
	}
	state.iter().flatten().filter(|&b| *b).count()
}

#[aoc(day21, part1)]
fn part1(input: &[Rule]) -> usize {
	calculate(input, 5)
}

#[aoc(day21, part2)]
fn part2(input: &[Rule]) -> usize {
	calculate(input, 18)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day21.txt"));
		assert_eq!(part1(&input), 152);
		assert_eq!(part2(&input), 1956174);
	}
}
