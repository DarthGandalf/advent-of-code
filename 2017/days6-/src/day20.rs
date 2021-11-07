use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Point {
	px: i64,
	py: i64,
	pz: i64,
	vx: i64,
	vy: i64,
	vz: i64,
	ax: i64,
	ay: i64,
	az: i64,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<Point> {
	input
		.trim()
		.lines()
		.map(|line| {
			let p = crate::numbers::parse(line);
			Point {
				px: p[0],
				py: p[1],
				pz: p[2],
				vx: p[3],
				vy: p[4],
				vz: p[5],
				ax: p[6],
				ay: p[7],
				az: p[8],
			}
		})
		.collect()
}

#[aoc(day20, part1)]
fn part1(input: &[Point]) -> usize {
	let mut state: Vec<Point> = input.iter().cloned().collect();
	for _ in 0..100_000 {
		for p in &mut state {
			p.vx += p.ax;
			p.vy += p.ay;
			p.vz += p.az;
			p.px += p.vx;
			p.py += p.vy;
			p.pz += p.vz;
		}
	}
	state
		.iter()
		.enumerate()
		.min_by_key(|(_, p)| p.px.abs() + p.py.abs() + p.pz.abs())
		.unwrap()
		.0
}

#[aoc(day20, part2)]
fn part2(input: &[Point]) -> usize {
	let mut state: Vec<Point> = input.iter().cloned().collect();
	for _ in 0..10_000 {
		let mut freq = fnv::FnvHashMap::<(i64, i64, i64), usize>::default();
		for p in &state {
			*freq.entry((p.px, p.py, p.pz)).or_default() += 1;
		}
		state.retain(|p| freq.get(&(p.px, p.py, p.pz)).unwrap() < &2);
		for p in &mut state {
			p.vx += p.ax;
			p.vy += p.ay;
			p.vz += p.az;
			p.px += p.vx;
			p.py += p.vy;
			p.pz += p.vz;
		}
	}
	state.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day20.txt"));
		assert_eq!(part1(&input), 364);
	}
}
