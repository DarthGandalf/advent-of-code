use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

fn solve(input: &str, num: i64) -> i64 {
	let mut input = input.lines();
	let state = input.next().unwrap().split(' ').skip(2).next().unwrap();
	let mut state : VecDeque<char> = state.chars().collect();
	let mut state_str : String = state.iter().collect();
	let mut leftmost = 0;
	let mut prev_leftmost = 0;
	let grow : HashSet<&str> = input.skip(1)
		.filter(|&line| line.chars().last().unwrap() == '#')
		.map(|line| &line[0..5]).collect();
	for k in 1..=num as i64 {
		// Ensure there is exactly 4 empty pots to the left.
		if let Some(begin) = state.iter().position(|&x| x == '#') {
			for _ in 0..begin as i32-4 {
				state.pop_front();
				leftmost += 1;
			}
		}
		if let Some(begin) = state.iter().take(4).position(|&x| x == '#') {
			for _ in 0..4-begin as i32 {
				state.push_front('.');
				leftmost -= 1;
			}
		}
		// Ensure there is exactly 4 empty pots to the right.
		if let Some(end) = state.iter().rev().position(|&x| x == '#') {
			for _ in 0..end as i32-4 {
				state.pop_back();
			}
		}
		if let Some(end) = state.iter().rev().take(4).position(|&x| x == '#') {
			for _ in 0..4-end as i32 {
				state.push_back('.');
			}
		}
		if state.len() < 5 {
			return 0;
		}
		let mut new_state = state.clone();
		for pos in 0..state.len()-4 {
			let pattern : String = state.iter().skip(pos).take(5).collect();
			new_state[pos + 2] = if grow.contains(&pattern as &str) {'#'} else {'.'};
		}
		let new_str : String = new_state.iter().collect();
		if state_str == new_str {
			leftmost += (leftmost - prev_leftmost) * (num - k + 1);
			break;
		}
		state = new_state;
		state_str = new_str;
		prev_leftmost = leftmost;
	}
	state.iter().enumerate().filter(|(_, &x)| x == '#').map(|(i, _)| i as i64+leftmost).sum()
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", solve(input, 50000000000));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(solve(include_str!("../example.txt"), 20), 325);
	}
}
