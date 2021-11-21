use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<(i32, i32)> {
	input
		.trim()
		.lines()
		.map(|line| {
			let l = crate::numbers::parse(line);
			(l[0], l[1])
		})
		.collect()
}

#[derive(Clone)]
struct State {
	used: fnv::FnvHashSet<usize>,
	endpin: i32,
	sum: i32,
}

#[aoc(day24, part1)]
fn part1(input: &[(i32, i32)]) -> i32 {
	let mut index_of_pin = fnv::FnvHashMap::<i32, Vec<usize>>::default();
	for (i, &(a, b)) in input.iter().enumerate() {
		index_of_pin.entry(a).or_default().push(i);
		index_of_pin.entry(b).or_default().push(i);
	}

	let mut queue = std::collections::VecDeque::<State>::new();
	queue.push_back(State {
		used: Default::default(),
		endpin: 0,
		sum: 0,
	});

	let mut maxsum = 0;
	while !queue.is_empty() {
		let current = queue.pop_front().unwrap();
		for &nextitem in index_of_pin.get(&current.endpin).unwrap() {
			if !current.used.contains(&nextitem) {
				let mut next = current.clone();
				next.used.insert(nextitem);
				next.endpin = input[nextitem].0 + input[nextitem].1 - current.endpin;
				next.sum += input[nextitem].0 + input[nextitem].1;
				if next.sum > maxsum {
					maxsum = next.sum;
				}
				queue.push_back(next);
			}
		}
	}

	maxsum
}

#[aoc(day24, part2)]
fn part2(input: &[(i32, i32)]) -> i32 {
	let mut index_of_pin = fnv::FnvHashMap::<i32, Vec<usize>>::default();
	for (i, &(a, b)) in input.iter().enumerate() {
		index_of_pin.entry(a).or_default().push(i);
		index_of_pin.entry(b).or_default().push(i);
	}

	let mut queue = std::collections::VecDeque::<State>::new();
	queue.push_back(State {
		used: Default::default(),
		endpin: 0,
		sum: 0,
	});

	let mut maxsum = 0;
	let mut maxlen = 0;
	while !queue.is_empty() {
		let current = queue.pop_front().unwrap();
		for &nextitem in index_of_pin.get(&current.endpin).unwrap() {
			if !current.used.contains(&nextitem) {
				let mut next = current.clone();
				next.used.insert(nextitem);
				next.endpin = input[nextitem].0 + input[nextitem].1 - current.endpin;
				next.sum += input[nextitem].0 + input[nextitem].1;
				if next.used.len() > maxlen {
					maxlen = next.used.len();
					maxsum = next.sum;
				} else if next.used.len() == maxlen && next.sum > maxsum {
					maxsum = next.sum;
				}
				queue.push_back(next);
			}
		}
	}

	maxsum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10",
		);
		assert_eq!(part1(&input), 31);
		assert_eq!(part2(&input), 19);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day24.txt"));
		assert_eq!(part1(&input), 1906);
		assert_eq!(part2(&input), 1824);
	}
}

// 47 wrong
