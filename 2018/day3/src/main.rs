use std::collections::BTreeSet;
extern crate regex;
use regex::Regex;

fn main() {
	let input = include_str!("../input.txt");
	let re = Regex::new(r"#(.*?) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
	let mut events = Vec::new();
	let mut ids = BTreeSet::new();
	for l in input.lines() {
		let captures = re.captures(&l).unwrap();
		let id = captures.get(1).unwrap().as_str().to_string();
		let x :i32 = captures.get(2).unwrap().as_str().parse().unwrap();
		let y :i32 = captures.get(3).unwrap().as_str().parse().unwrap();
		let w :i32 = captures.get(4).unwrap().as_str().parse().unwrap();
		let h :i32 = captures.get(5).unwrap().as_str().parse().unwrap();
		let x2 = x + w;
		let y2 = y + h;
		events.push((x, true, y, y2, id.clone()));
		events.push((x2, false, y, y2, id.clone()));
		ids.insert(id);
	}
	events.sort();
	let mut sweep :BTreeSet<(i32, i32, String)> = BTreeSet::new();
	let mut count = 0;
	let mut lastx = -1;
	for (x, new, y1, y2, id) in events {
		if x > lastx {
			let mut length = 0;
			let mut prevy = -1;
			let mut curcount = 0;
			let mut curids :BTreeSet<String> = BTreeSet::new();
			for (y, diff, idy) in sweep.iter() {
				if curcount > 1 {
					length += y - prevy;
					for i in curids.iter() {
						ids.remove(i);
					}
				}
				prevy = *y;
				curcount += diff;
				if diff > &0 {
					curids.insert(idy.to_string());
				} else {
					curids.remove(idy);
				}
			}
			count += length * (x - lastx);
			lastx = x;
		}
		if new {
			sweep.insert((y1, 1, id.clone()));
			sweep.insert((y2, -1, id));
		} else {
			sweep.remove(&(y1, 1, id.clone()));
			sweep.remove(&(y2, -1, id));
		}
	}
	println!("{} {:?}", count, ids);
}
