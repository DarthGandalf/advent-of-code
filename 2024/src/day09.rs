use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::VecDeque;

fn gauss(pos: i64, len: i64) -> i64 {
	(pos + pos + len - 1) * len / 2
}

type Holes = VecDeque<(i64, i64)>;
type Files = VecDeque<(i64, i64, i64)>;

fn parse(input: &str) -> (Holes, Files) {
	let mut holes = VecDeque::new();
	let mut files = VecDeque::new();
	let mut flip = true;
	let mut pos: i64 = 0;
	let mut id = 0;
	for c in input.chars().flat_map(|c| c.to_digit(10)) {
		let c: i64 = c as i64;
		if flip {
			assert!(c > 0);
			files.push_back((pos, c, id));
			id += 1;
		} else if c > 0 {
			holes.push_back((pos, c));
		}
		flip = !flip;
		pos += c;
	}
	(holes, files)
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
	let (mut holes, mut files) = parse(input);
	let mut s = 0;
	let mut pos = 0;
	while !files.is_empty() {
		if files[0].0 == pos {
			s += files[0].2 * gauss(pos, files[0].1);
			pos += files[0].1;
			files.pop_front();
		} else {
			let l = files.back().unwrap().1.min(holes[0].1);
			s += files.back().unwrap().2 * gauss(pos, l);
			pos += l;
			holes[0].1 -= l;
			files.back_mut().unwrap().1 -= l;
			if holes[0].1 == 0 {
				holes.pop_front();
			}
			if files.back().unwrap().1 == 0 {
				files.pop_back();
			}
		}
	}
	s
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
	let (mut holes, mut files) = parse(input);
	let n = files.len();
	for f in 0..n {
		let f = n - f - 1;
		let f = &mut files[f];
		if let Some((hi, ho)) = holes.iter().find_position(|h| h.1 >= f.1) {
			if ho.0 < f.0 {
				f.0 = ho.0;
				holes[hi].0 += f.1;
				holes[hi].1 -= f.1;
				if holes[hi].1 == 0 {
					holes.remove(hi);
				}
			}
		}
	}
	files.into_iter().map(|f| gauss(f.0, f.1) * f.2).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "2333133121414131402";

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 1928);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 2858);
	}
}
