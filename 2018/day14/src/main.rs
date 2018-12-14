use std::time::Instant;
use std::collections::VecDeque;

struct State {
	board: VecDeque<u8>,
	elf1: usize,
	elf2: usize,
	cursor: usize,
}

impl State {
	fn new() -> Self {
		let mut state = State {
			board: VecDeque::new(),
			elf1: 0,
			elf2: 1,
			cursor: 0,
		};
		state.board.push_back(3);
		state.board.push_back(7);
		state
	}
}

impl Iterator for State {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		if self.cursor >= self.board.len() {
			let sum = self.board[self.elf1] + self.board[self.elf2];
			if sum >= 10 {
				self.board.push_back(1);
			}
			self.board.push_back(sum % 10);
			self.elf1 = (self.elf1 + self.board[self.elf1] as usize + 1) % self.board.len();
			self.elf2 = (self.elf2 + self.board[self.elf2] as usize + 1) % self.board.len();
		}
		let result = self.board[self.cursor];
		self.cursor += 1;
		Some(result)
	}
}

fn _solve1(input: usize) -> String {
	State::new()
		.skip(input)
		.take(10)
		.map(|x| (b'0' + x) as char)
		.collect()
}

fn _solve2(input: &str) -> usize {
	let input: Vec<u8> = input.chars().map(|x| x as u8 - b'0').collect();
	let tens = input.iter().fold(1, |acc, _| acc * 10);
	let pattern: i32 = input.iter().fold(0, |acc, &x| acc * 10 + x as i32);
	let mut last: i32 = 0;
	for (i, x) in State::new().enumerate() {
		last = (last * 10 + x as i32) % tens;
		if pattern == last {
			return i - input.len() + 1;
		}
	}
	0
}

fn main() {
	let time = Instant::now();
	println!("{}", _solve2("323081"));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(_solve1(9), "5158916779");
		assert_eq!(_solve1(5), "0124515891");
		assert_eq!(_solve1(18), "9251071085");
		assert_eq!(_solve1(2018), "5941429882");
	}

	#[test]
	fn test_2() {
		assert_eq!(_solve2("51589"), 9);
		assert_eq!(_solve2("01245"), 5);
		assert_eq!(_solve2("92510"), 18);
		assert_eq!(_solve2("59414"), 2018);
	}
}
