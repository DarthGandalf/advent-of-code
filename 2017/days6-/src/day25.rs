use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day25.pest"]
struct Day25Parser;

#[derive(Debug, PartialEq)]
enum Dir {
	Left,
	Right,
}

#[derive(Debug)]
struct Action {
	write: i32,
	dir: Dir,
	next: char,
}

type State = fnv::FnvHashMap<i32, Action>;

#[aoc_generator(day25)]
fn parse(input: &str) -> (char, usize, fnv::FnvHashMap<char, State>) {
	let input = match Day25Parser::parse(Rule::input, input.trim()) {
		Ok(x) => x,
		Err(e) => {
			panic!("{}", e);
		}
	}
	.next()
	.unwrap();

	let mut start: Option<char> = None;
	let mut steps: Option<usize> = None;
	let mut states: fnv::FnvHashMap<char, State> = Default::default();

	for x in input.into_inner() {
		match x.as_rule() {
			Rule::header => {
				for y in x.into_inner() {
					match y.as_rule() {
						Rule::state_name => {
							start = Some(y.as_str().chars().next().unwrap());
						}
						Rule::num => steps = Some(y.as_str().parse().unwrap()),
						_ => unreachable!(),
					}
				}
			}
			Rule::state_desc => {
				let mut name: Option<char> = None;
				let mut state: State = Default::default();
				for y in x.into_inner() {
					match y.as_rule() {
						Rule::state_name => {
							name = Some(y.as_str().chars().next().unwrap());
						}
						Rule::state_if => {
							let mut z = y.into_inner();
							let current = z.next().unwrap().as_str().parse().unwrap();
							let write = z.next().unwrap().as_str().parse().unwrap();
							let dir = z.next().unwrap().as_str();
							let dir = match dir {
								"left" => Dir::Left,
								"right" => Dir::Right,
								_ => unreachable!(),
							};
							let next = z.next().unwrap().as_str().chars().next().unwrap();
							let act = Action { write, dir, next };
							state.insert(current, act);
						}
						_ => unreachable!(),
					}
				}
				states.insert(name.unwrap(), state);
			}
			_ => unreachable!(),
		}
	}

	(start.unwrap(), steps.unwrap(), states)
}

#[aoc(day25, part1)]
fn part1(input: &(char, usize, fnv::FnvHashMap<char, State>)) -> usize {
	let mut ones = fnv::FnvHashSet::<i64>::default();
	let mut pos = 0;
	let mut state = input.0;
	for _ in 0..input.1 {
		let s = input.2.get(&state).unwrap();
		let current = if ones.contains(&pos) { 1 } else { 0 };
		let act = s.get(&current).unwrap();
		if act.write == 1 {
			ones.insert(pos);
		} else {
			ones.remove(&pos);
		}
		if act.dir == Dir::Left {
			pos -= 1;
		} else {
			pos += 1;
		}
		state = act.next;
	}
	ones.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day25.txt"));
		assert_eq!(part1(&input), 3362);
	}
}

// 47 wrong
