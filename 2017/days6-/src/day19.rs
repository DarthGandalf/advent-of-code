use aoc_runner_derive::{aoc, aoc_generator};

struct Grid(Vec<Vec<char>>);

impl Grid {
	fn get(&self, y: i32, x: i32) -> char {
		if (0..self.0.len() as i32).contains(&y) {
			if (0..self.0[y as usize].len() as i32).contains(&x) {
				return self.0[y as usize][x as usize];
			}
		}
		return ' ';
	}
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Grid {
	Grid(
		input
			.lines()
			.map(|line| line.chars().into_iter().collect())
			.collect(),
	)
}

fn calculate(input: &Grid) -> (String, i32) {
	let mut word = "".to_string();
	let mut steps = 0;
	let mut pos = (0i32, 0i32);
	let mut dir = (1, 0);
	pos.1 = input.0[0].iter().position(|&c| c == '|').unwrap() as i32;

	loop {
		steps += 1;
		pos.0 += dir.0;
		pos.1 += dir.1;
		match input.get(pos.0, pos.1) {
			'A'..='Z' => {
				word = format!("{}{}", word, input.get(pos.0, pos.1));
			}
			'+' => {
				for new_dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
					if new_dir.0 == -dir.0 && new_dir.1 == -dir.1 {
						continue;
					}
					let p = (pos.0 + new_dir.0, pos.1 + new_dir.1);
					if input.get(p.0, p.1) != ' ' {
						dir = new_dir;
						break;
					}
				}
			}
			' ' => return (word, steps),
			_ => {}
		}
	}
}

#[aoc(day19, part1)]
fn part1(input: &Grid) -> String {
	calculate(input).0
}

#[aoc(day19, part2)]
fn part2(input: &Grid) -> i32 {
	calculate(input).1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
",
		);
		assert_eq!(calculate(&input), ("ABCDEF".to_string(), 38));
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day19.txt"));
		assert_eq!(calculate(&input), ("SXWAIBUZY".to_string(), 16676));
	}
}
