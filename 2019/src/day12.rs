use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day12.pest"]
struct Day12Parser;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Moon {
	pos: (i32, i32, i32),
	vel: (i32, i32, i32),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Input(Vec<Moon>);

#[aoc_generator(day12)]
fn parse(input: &str) -> anyhow::Result<Input> {
	let input = Day12Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?;
	let moons: anyhow::Result<Vec<Moon>> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::moon)
		.map(|pair| -> anyhow::Result<Moon> {
			let mut moon = pair.into_inner();
			let x: i32 = moon.next().none_err()?.as_str().parse()?;
			let y: i32 = moon.next().none_err()?.as_str().parse()?;
			let z: i32 = moon.next().none_err()?.as_str().parse()?;
			Ok(Moon {
				pos: (x, y, z),
				vel: (0, 0, 0),
			})
		})
		.collect();
	Ok(Input(moons?))
}

fn iterator(input: Input) -> impl Iterator<Item = Input> {
	std::iter::successors(Some(input), |moons| {
		let mut new_moons = moons.clone();
		for m1 in 0..new_moons.0.len() {
			for m2 in 0..new_moons.0.len() {
				if new_moons.0[m1].pos.0 < new_moons.0[m2].pos.0 {
					new_moons.0[m1].vel.0 += 1;
					new_moons.0[m2].vel.0 -= 1;
				}
				if new_moons.0[m1].pos.1 < new_moons.0[m2].pos.1 {
					new_moons.0[m1].vel.1 += 1;
					new_moons.0[m2].vel.1 -= 1;
				}
				if new_moons.0[m1].pos.2 < new_moons.0[m2].pos.2 {
					new_moons.0[m1].vel.2 += 1;
					new_moons.0[m2].vel.2 -= 1;
				}
			}
		}
		for m in &mut new_moons.0 {
			m.pos.0 += m.vel.0;
			m.pos.1 += m.vel.1;
			m.pos.2 += m.vel.2;
		}
		Some(new_moons)
	})
}

fn part1_energy(input: &Input, steps: usize) -> anyhow::Result<i32> {
	let result = iterator(input.clone()).skip(steps).next().none_err()?;
	Ok(result
		.0
		.iter()
		.map(|m| {
			let pot = m.pos.0.abs() + m.pos.1.abs() + m.pos.2.abs();
			let kin = m.vel.0.abs() + m.vel.1.abs() + m.vel.2.abs();
			pot * kin
		})
		.sum())
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> anyhow::Result<i32> {
	part1_energy(input, 1000)
}

fn find_repeat(input: &Input, check: impl Fn(&Input) -> bool) -> usize {
	for (i, moons) in iterator(input.clone()).enumerate().skip(1) {
		if check(&moons) {
			return i;
		}
	}
	0
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
	let x = find_repeat(&input, |moons| {
		itertools::zip(&input.0, &moons.0).all(|(a, b)| a.pos.0 == b.pos.0 && a.vel.0 == b.vel.0)
	});
	let y = find_repeat(&input, |moons| {
		itertools::zip(&input.0, &moons.0).all(|(a, b)| a.pos.1 == b.pos.1 && a.vel.1 == b.vel.1)
	});
	let z = find_repeat(&input, |moons| {
		itertools::zip(&input.0, &moons.0).all(|(a, b)| a.pos.2 == b.pos.2 && a.vel.2 == b.vel.2)
	});
	use num::Integer;
	x.lcm(&y).lcm(&z)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		let input = parse(
			"
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
		)
		.unwrap();
		assert_eq!(part1_energy(&input, 10).unwrap(), 179);
		assert_eq!(part2(&input), 2772);
	}

	#[test]
	fn test2() {
		let input = parse(
			"
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
		)
		.unwrap();
		assert_eq!(part1_energy(&input, 100).unwrap(), 1940);
		assert_eq!(part2(&input), 4686774924);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day12.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 14907);
		assert_eq!(part2(&input), 467081194429464);
	}
}
