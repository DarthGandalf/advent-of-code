use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
struct Bot {
	x: [i32; 3],
	r: i32,
}

fn parse(input: &str) -> Vec<Bot> {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"(?m)^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
	}
	RE.captures_iter(input)
		.map(|c| Bot {
			x: [
				c[1].parse().unwrap(),
				c[2].parse().unwrap(),
				c[3].parse().unwrap(),
			],
			r: c[4].parse().unwrap(),
		})
		.collect()
}

fn _solve1(input: &str) -> usize {
	let bots = parse(&input);

	let mut max = 0;
	for (i, b) in bots.iter().enumerate() {
		if b.r > bots[max].r {
			max = i;
		}
	}
	bots.iter()
		.filter(|b| {
			b.x.iter()
				.zip(bots[max].x.iter())
				.map(|(&u, &v)| (u - v).abs())
				.sum::<i32>()
				<= bots[max].r
		})
		.count()
}

fn _solve2(input: &str) -> i32 {
	let bots = parse(&input);
	let mut tppp: Vec<(i32, i8, usize)> = Vec::new();
	for (i, b) in bots.iter().enumerate() {
		tppp.push((b.x[0] + b.x[1] + b.x[2] - b.r, -1, i));
		tppp.push((b.x[0] + b.x[1] + b.x[2] + b.r, 1, i));
	}
	tppp.sort();
	let mut max_bots = 0;
	let mut min_dist = 0;
	let mut active_bots_ppp = HashSet::new();
	for (xppp, addition, num) in tppp {
		println!("ppp: {} {} {}", xppp, addition, num);
		if addition == -1 {
			active_bots_ppp.insert(num);
		}
		let mut tppn: Vec<(i32, i8, usize)> = Vec::new();
		for &i in &active_bots_ppp {
			let b = &bots[i];
			tppn.push((b.x[0] + b.x[1] - b.x[2] - b.r, -1, i));
			tppn.push((b.x[0] + b.x[1] - b.x[2] + b.r, 1, i));
		}
		tppn.sort();
		let mut active_bots_ppn = HashSet::new();
		for (xppn, addition, num) in tppn {
//			println!("  ppn: {} {} {}", xppn, addition, num);
			if addition == -1 {
				active_bots_ppn.insert(num);
			}
			let mut tpnp: Vec<(i32, i8, usize)> = Vec::new();
			for &i in &active_bots_ppn {
				let b = &bots[i];
				tpnp.push((b.x[0] - b.x[1] + b.x[2] - b.r, -1, i));
				tpnp.push((b.x[0] - b.x[1] + b.x[2] + b.r, 1, i));
			}
			tpnp.sort();
			let mut active_bots_pnp = 0;
			for (xpnp, addition, _) in tpnp {
//				println!("    pnp: {} {} {}", xpnp, addition, num);
				if addition == -1 {
					active_bots_pnp += 1;
				}
				let x=(xppn+xpnp)/2;
				let y=(xppp-xpnp)/2;
				let z=(xppp-xppn)/2;
				let dist = x.abs() + y.abs() + z.abs();
//				println!("    testing: bots={} dist={} ", active_bots_pnp, dist);
				if active_bots_pnp > max_bots {
					max_bots = active_bots_pnp;
					min_dist = dist;
				} else if active_bots_pnp == max_bots && dist < min_dist {
					min_dist = dist;
				}
				if addition == 1 {
					active_bots_pnp -= 1;
				}
			}
			if addition == 1 {
				active_bots_ppn.remove(&num);
			}
		}
		if addition == 1 {
			active_bots_ppp.remove(&num);
		}
	}
	min_dist
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", _solve2(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let input = include_str!("../example.txt");
		assert_eq!(_solve1(&input), 7);
	}

	#[test]
	fn test_2() {
		let input = include_str!("../example2.txt");
		assert_eq!(_solve2(&input), 36);
	}
}
