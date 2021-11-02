use aoc_runner_derive::aoc;
use std::collections::VecDeque;

fn step(lengths: &[u8], state: &mut VecDeque<u8>, pos: &mut u16, skip: &mut u16) {
	for &len in lengths {
		let mut part = std::collections::VecDeque::with_capacity(len.into());
		for _ in 0..len {
			let x = state.pop_front().unwrap();
			part.push_front(x);
		}
		state.append(&mut part);
		*pos += len as u16;
		state.rotate_left(*skip as usize);
		*pos += *skip;
		*pos %= state.len() as u16;
		*skip += 1;
		*skip %= state.len() as u16;
	}
}

fn calculate1(input: &str, num_marks: u16) -> u16 {
	let input: Vec<u8> = crate::numbers::parse(input);
	let mut pos: u16 = 0;
	let mut state: VecDeque<u8> = (0..num_marks).map(|x| x as u8).collect();
	let mut skip = 0;
	step(&input, &mut state, &mut pos, &mut skip);
	state.rotate_right(pos.into());
	let x = state.pop_front().unwrap();
	let y = state.pop_front().unwrap();
	x as u16 * y as u16
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u16 {
	calculate1(input, 256)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> String {
	let lengths: Vec<u8> = input
		.trim()
		.as_bytes()
		.iter()
		.cloned()
		.chain([17, 31, 73, 47, 23])
		.collect();
	let mut pos: u16 = 0;
	let mut state: VecDeque<u8> = (0..256).map(|x| x as u8).collect();
	let mut skip = 0;
	for _ in 0..64 {
		step(&lengths, &mut state, &mut pos, &mut skip);
	}
	state.rotate_right(pos.into());
	let mut s = "".to_string();
	for _ in 0..16 {
		let mut xor = 0;
		for _ in 0..16 {
			let val = state.pop_front().unwrap();
			xor ^= val;
		}
		s += &format!("{:02x}", xor);
	}
	s
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		assert_eq!(calculate1("3, 4, 1, 5", 5), 12);
		assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
		assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
		assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
		assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
	}

	#[test]
	fn answers() {
		let input = include_str!("../input/2017/day10.txt");
		assert_eq!(part1(&input), 23715);
		assert_eq!(part2(&input), "541dc3180fd4b72881e39cf925a50253");
	}
}
