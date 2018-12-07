use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
extern crate edit_distance;

fn _main1() {
	let f = File::open("input.txt").unwrap();
	let file = BufReader::new(&f);
	let mut doubles = 0;
	let mut triples = 0;
	for line in file.lines() {
		let l = line.unwrap();
		let mut letters = HashMap::new();
		for c in l.chars() {
			*letters.entry(c).or_insert(0) += 1;
		}
		let mut double = false;
		let mut triple = false;
		for (_, y) in letters {
			if y == 2 {
				double = true;
			}
			if y == 3 {
				triple = true;
			}
		}
		if double {
			doubles += 1;
		}
		if triple {
			triples += 1;
		}
	}
	println!("{}", doubles * triples);
}

fn main() {
	let f = File::open("input.txt").unwrap();
	let file = BufReader::new(&f);
	let boxes :Vec<_> = file.lines().map(|x| x.unwrap()).collect();
	for x in boxes.clone().iter() {
		for y in boxes.clone().iter() {
			let d = edit_distance::edit_distance(&x, &y);
			if d == 1 {
				println!("{}", &x);
				println!("{}", &y);
				return;
			}
		}
	}
}
