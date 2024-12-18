use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
	let input = parse(input);
	solve1(&input[..1024], 70).unwrap()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
	input
		.lines()
		.flat_map(|l| l.split(',').flat_map(str::parse::<usize>).collect_tuple())
		.collect_vec()
}

fn solve1(input: &[(usize, usize)], size: usize) -> Option<usize> {
	let mut m = vec![vec![true; size + 1]; size + 1];
	for &(y, x) in input {
		m[y][x] = false;
	}
	pathfinding::directed::astar::astar(
		&(0, 0),
		|&(y, x)| {
			let mut n = Vec::new();
			if y > 0 && m[y - 1][x] {
				n.push((y - 1, x));
			}
			if y < size && m[y + 1][x] {
				n.push((y + 1, x));
			}
			if x > 0 && m[y][x - 1] {
				n.push((y, x - 1));
			}
			if x < size && m[y][x + 1] {
				n.push((y, x + 1));
			}
			n.into_iter().map(|v| (v, 1)).collect_vec()
		},
		|&(y, x)| y.abs_diff(size) + x.abs_diff(size),
		|&(y, x)| y == size && x == size,
	)
	.map(|(_, len)| len)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
	solve2(input, 70)
}

fn solve2(input: &str, size: usize) -> String {
	let input = parse(input);
	let mut first = 0;
	let mut last = input.len();
	while first < last {
		let mid = first.midpoint(last);
		if solve1(&input[..mid], size).is_some() {
			first = mid + 1;
		} else {
			last = mid;
		}
	}
	let coord = &input[first - 1];
	format!("{},{}", coord.0, coord.1)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(solve1(&parse(INPUT)[..12], 6).unwrap(), 22);
	}

	#[test]
	fn test2() {
		assert_eq!(solve2(INPUT, 6), "6,1");
	}
}
