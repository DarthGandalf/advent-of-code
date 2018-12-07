use std::collections::BTreeMap;
use std::time::Instant;

fn solve(input: &'static str) -> i32 {
	let mut lines : Vec<_> = input.lines().collect();
	lines.sort_unstable();
	let mut byguard : BTreeMap<&str, [i32; 60]> = BTreeMap::new();
	let mut curguard = "";
	let mut asleepsince = -1;
	for x in lines {
		match x[25..].chars().next() {
			Some('a') => {
				asleepsince = x[15..=16].parse().unwrap();
			}
			Some('u') => {
				let ar = byguard.entry(curguard).or_insert([0; 60]);
				let up : i32 = x[15..=16].parse().unwrap();
				for i in asleepsince..up {
					ar[i as usize] += 1;
				}
				asleepsince = -1;
			}
			Some('#') => {
				if asleepsince >= 0 {
					let ar = byguard.entry(curguard).or_insert([0; 60]);
					for i in asleepsince..60 {
						ar[i as usize] += 1;
					}
					asleepsince = -1;
				}
				curguard = x[26..].split(' ').next().unwrap();
			}
			_ => {}
		}
	}
	for (guard, byminute) in byguard.iter() {
		print!("{} ", guard);
		for z in byminute.iter() {
			print!("{} ", z);
		}
		println!();
	}
	let mut minute : i32 = 0;
	let mut max = -1;
/*	let guards : Vec<(&str, i32)> = byguard.iter().map(|(&guard, &byminute)| (guard, byminute.iter().sum())).collect();
	let (guard, _) = guards.iter().max_by_key(|(_, total)| total).unwrap();
	for (i, num) in byguard.get(guard).unwrap().iter().enumerate() {
		if num > &max {
			max = *num;
			minute = i as i32;
		}
	}*/
	let mut guard = "";
	for (g, byminute) in byguard.iter() {
		for (i, num) in byminute.iter().enumerate() {
			if num > &max {
				max = *num;
				minute = i as i32;
				guard = g;
			}
		}
	}
	let guard : i32 = guard.parse().unwrap();
	minute * guard
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", solve(input));
	println!("{:?}", time.elapsed());
}
