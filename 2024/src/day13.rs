use aoc_runner_derive::aoc;

use nom::{
	Err, IResult,
	bytes::complete::tag,
	character::complete::digit1,
	combinator::{eof, map_res},
	error::{Error, ErrorKind},
	sequence::tuple,
};

#[derive(Debug)]
struct Coord {
	x: i64,
	y: i64,
}

#[derive(Debug)]
struct Machine {
	a: Coord,
	b: Coord,
	prize: Coord,
}

fn parseu64(input: &str) -> IResult<&str, i64> {
	map_res(digit1, str::parse)(input)
}

fn parse_block(input: &str) -> IResult<&str, Machine> {
	let Ok((_, (_, ax, _, ay, _, bx, _, by, _, px, _, py, _))) = tuple((
		tag("Button A: X+"),
		parseu64,
		tag(", Y+"),
		parseu64,
		tag("\nButton B: X+"),
		parseu64,
		tag(", Y+"),
		parseu64,
		tag("\nPrize: X="),
		parseu64,
		tag(", Y="),
		parseu64,
		eof,
	))(input) else {
		// ErrorKind::Tag is wrong, but what I'm supposed to return here?
		return Err(Err::Error(Error::new(input, ErrorKind::Tag)));
	};
	Ok(("", Machine {
		a: Coord { x: ax, y: ay },
		b: Coord { x: bx, y: by },
		prize: Coord { x: px, y: py },
	}))
}

// na * ax + nb * bx = px
// na * ay + nb * by = py
// na * 3 + nb -> min
//
// na * ax = px - nb * bx
// na = (px - nb * bx) / ax
// ay * (px - nb * bx) / ax + nb * by = py
// ay * (px - nb * bx) + nb * by * ax = py * ax
// ay * px - ay * nb * bx + nb * by * ax = py * ax
// nb * by * ax - ay * nb * bx = py * ax - ay * px
// nb * (by * ax - ay * bx) = py * ax - ay * px
// nb = (py * ax - ay * px) / (by * ax - ay * bx)
// na = (px - bx * (py * ax - ay * px) / (by * ax - ay * bx)) / ax
// na = (px * (by * ax - ay * bx) - bx * (py * ax - ay * px)) / (ax * (by * ax - ay * bx))
// na = (px * by * ax - px * ay * bx - bx * py * ax + bx * ay * px) /
// na = (px * by * ax - bx * py * ax) / (ax * (by * ax - ay * bx))
// na = (px * by - bx * py) / (by * ax - ay * bx)
//
// na = (px * by - bx * py) / (by * ax - ay * bx)
// nb = (py * ax - ay * px) / (by * ax - ay * bx)

fn solve(input: &str, offset: i64) -> i64 {
	input
		.split("\n\n")
		.map(|block| {
			if let Ok((_, Machine { a, b, prize: mut p })) = parse_block(block) {
				p.x += offset;
				p.y += offset;
				let d = b.y * a.x - a.y * b.x;
				assert!(d != 0);
				let nad = p.x * b.y - b.x * p.y;
				let nbd = p.y * a.x - a.y * p.x;
				if nad % d == 0 && nbd % d == 0 {
					(3 * nad + nbd) / d
				} else {
					0
				}
			} else {
				0
			}
		})
		.sum()
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
	solve(input, 0)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
	solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 480);
	}

	#[test]
	fn test2() {}
}
