use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
	IResult,
	bytes::complete::tag,
	character::complete::digit1,
	combinator::{eof, map_res, opt},
	sequence::tuple,
};
use fnv::FnvHashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coord {
	x: i64,
	y: i64,
}

#[derive(Debug)]
struct Robot {
	p: Coord,
	v: Coord,
}

fn parsei64(input: &str) -> IResult<&str, i64> {
	let (input, minus) = opt(tag("-"))(input)?;
	let minus = minus.is_some();
	map_res(digit1, |s: &str| {
		s.parse().map(|x: i64| x * if minus { -1 } else { 1 })
	})(input)
}

fn parseu64(input: &str) -> IResult<&str, i64> {
	map_res(digit1, str::parse)(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
	let (_, (_, px, _, py, _, vx, _, vy, _)) = tuple((
		tag("p="),
		parseu64,
		tag(","),
		parseu64,
		tag(" v="),
		parsei64,
		tag(","),
		parsei64,
		eof,
	))(input)?;
	Ok(("", Robot {
		p: Coord { x: px, y: py },
		v: Coord { x: vx, y: vy },
	}))
}

fn solve1(input: &str, width: i64, height: i64) -> i64 {
	let w2 = width / 2;
	let h2 = height / 2;
	let mut q = [0; 4];
	for r in input.lines().flat_map(|l| parse_robot(l)).map(|r| r.1) {
		let x = (r.p.x + 100 * r.v.x).rem_euclid(width);
		let y = (r.p.y + 100 * r.v.y).rem_euclid(height);
		if x == w2 || y == h2 {
			continue;
		}
		let index = if x < w2 && y < h2 {
			0
		} else if x < w2 && y > h2 {
			1
		} else if y < h2 {
			2
		} else {
			3
		};
		q[index] += 1;
	}
	q.iter().product()
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
	solve1(input, 101, 103)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
	const WIDTH: usize = 101;
	const HEIGHT: usize = 103;
	let mut robots = input
		.lines()
		.flat_map(|l| parse_robot(l))
		.map(|r| r.1)
		.collect_vec();
	let mut counter = 0;
	loop {
		counter += 1;
		let mut poses = FnvHashSet::default();
		for r in &mut robots {
			r.p.x = (r.p.x + r.v.x).rem_euclid(WIDTH as i64);
			r.p.y = (r.p.y + r.v.y).rem_euclid(HEIGHT as i64);
			poses.insert(r.p.clone());
		}
		if poses.len() == robots.len() {
			break;
		}
	}
	//let mut m = [['.'; width]; height];
	//for r in robots {
	//	m[r.p.y as usize][r.p.x as usize] = 'X';
	//}
	//println!();
	//for r in m {
	//	for c in r {
	//		print!("{c}");
	//	}
	//	println!();
	//}
	counter
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(solve1(INPUT, 7, 11), 12);
	}

	#[test]
	fn test2() {}
}
