use aoc_runner_derive::aoc;
use fnv::FnvHashMap;
use itertools::Itertools;
use regex::Regex;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Hash, Eq, PartialEq, EnumIter, Clone, Copy)]
enum Key {
	Up,
	Down,
	Left,
	Right,
	A,
}

#[derive(Default)]
struct Pusher {
	m: FnvHashMap<(Key, Key), usize>,
}

#[memoize::memoize]
fn keypress(level: u8, target: u8, from: char, to: char) -> usize {
	if level == target {
		return 1;
	}
	let loc: FnvHashMap<char, (i8, i8)> = if level == 0 {
		[
			('A', (3, 2)),
			('0', (3, 1)),
			('1', (2, 0)),
			('2', (2, 1)),
			('3', (2, 2)),
			('4', (1, 0)),
			('5', (1, 1)),
			('6', (1, 2)),
			('7', (0, 0)),
			('8', (0, 1)),
			('9', (0, 2)),
		]
		.into_iter()
		.collect()
	} else {
		[
			('A', (0, 2)),
			('^', (0, 1)),
			('<', (1, 0)),
			('v', (1, 1)),
			('>', (1, 2)),
		]
		.into_iter()
		.collect()
	};
	let from = loc.get(&from).unwrap();
	let to = loc.get(&to).unwrap();
	let mut sum = 0;
	let mut current = 'A';
	if from.0 < to.0 {
		for i in from.0..to.0 {
			sum += &keypress(level + 1, target, current, 'v');
			current = 'v';
		}
	}
	if from.0 > to.0 {
		for i in to.0..from.0 {
			sum += &keypress(level + 1, target, current, '^');
			current = '^';
		}
	}
	if from.1 < to.1 {
		for i in from.1..to.1 {
			sum += &keypress(level + 1, target, current, '>');
			current = '>';
		}
	}
	if from.1 > to.1 {
		for i in to.1..from.1 {
			sum += &keypress(level + 1, target, current, '<');
			current = '<';
		}
	}
	sum += &keypress(level + 1, target, current, 'A');
	sum
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
	input
		.lines()
		.map(|l| {
			println!("==={l}");
			let c: usize = l[..3].parse().unwrap();
			std::iter::once('A')
				.chain(l.chars())
				.map_windows(|a: &[char; 2]| keypress(0, 3, a[0], a[1]))
				.inspect(|x| println!("{x}"))
				.sum::<usize>()
				* c
		})
		.sum()
}

/*#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
	let loc: FnvHashMap<Key, (i32, i32)> = [
		(Key::Up, (0, 1)),
		(Key::A, (0, 2)),
		(Key::Left, (1, 0)),
		(Key::Down, (1, 1)),
		(Key::Right, (1, 2)),
	]
	.into_iter()
	.collect();
	let mut p = Pusher::default();
	for from in Key::iter() {
		for to in Key::iter() {
			p.m.insert((from, to), 1);
		}
	}
	for i in 0..2 {
		let mut newp = Pusher::default();
		for from in Key::iter() {
			let frloc = loc.get(&from).unwrap();
			for to in Key::iter() {
				let toloc = loc.get(&to).unwrap();
				let dy = toloc.0 - frloc.0;
				let dx = toloc.1 - frloc.1;
				let sum = 0;
				let mut needed_buttons = vec![Key::A];
				if dx > 0 {
					needed_buttons.push(Key::Right);
				}
				if dx < 0 {
					needed_buttons.push(Key::Left);
				}
				if dy > 0 {
					needed_buttons.push(Key::Down);
				}
				if dy < 0 {
					needed_buttons.push(Key::Up);
				}
				let miny = needed_buttons.iter().map(|b| loc.get(&b).unwrap().0).min().unwrap();
				let maxy = needed_buttons.iter().map(|b| loc.get(&b).unwrap().0).max().unwrap();
				let minx = needed_buttons.iter().map(|b| loc.get(&b).unwrap().1).min().unwrap();
				let maxx = needed_buttons.iter().map(|b| loc.get(&b).unwrap().1).max().unwrap();
				let dx = maxx - minx;
				let dy = maxy - miny;
				p.m.get();
			}
		}
		p = newp;
	}
	0
}*/

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
	0
}
/*
#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
029A
980A
179A
456A
379A
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 126384);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 0);
	}
}
*/
