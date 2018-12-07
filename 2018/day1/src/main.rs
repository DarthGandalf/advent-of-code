use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
	let f = File::open("input.txt").unwrap();
	let file = BufReader::new(&f);
	let mut x = 0;
	let mut seen = HashSet::new();
	let mut lines = Vec::new();
	for line in file.lines() {
		let l = line.unwrap();
		lines.push(l);
	}
	loop {
		for l in &lines {
			let mut ch = l.chars();
			let sign = ch.next().unwrap();
			let y : i32 = l.get(1..).unwrap().parse().unwrap();
			match sign {
				'-' => {
					x -= y;
				}
				'+' => {
					x += y;
				}
				_ => {}
			}
			if seen.contains(&x) {
				println!("{}", x);
				return;
			}
			seen.insert(x);
		}
	}
}
