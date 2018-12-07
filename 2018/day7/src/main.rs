use std::time::Instant;
use std::collections::BTreeSet;
use std::ops::Sub;

fn _solve1(input: &str) -> String {
	let mut edges : Vec<_> = input.lines().map(|line| {
		(line[5..].chars().next().unwrap(), line[36..].chars().next().unwrap())
	}).collect();
	let mut nodes : BTreeSet<_> = edges.iter().map(|(a, _)| *a).collect();
	nodes.extend(edges.iter().map(|(_, a)| *a));
	let mut result = String::new();
	while nodes.len() > 0 {
		let x = *nodes.sub(&edges.iter().map(|(_, a)| *a).collect()).iter().next().unwrap();
		result.push(x);
		edges = edges.into_iter().filter(|(a, _)| *a != x).collect();
		nodes.remove(&x);
	}
	result
}

#[derive(Clone)]
struct Work {
	node: char,
	left: u8,
}

fn solve2(input: &str) -> i32 {
	let mut edges : Vec<_> = input.lines().map(|line| {
		(line[5..].chars().next().unwrap(), line[36..].chars().next().unwrap())
	}).collect();
	let mut nodes : BTreeSet<_> = edges.iter().map(|(a, _)| *a).collect();
	nodes.extend(edges.iter().map(|(_, a)| *a));
	let mut timer : i32 = 0;
	let mut workers: Vec<Option<Work>> = vec![None; 5];
	while nodes.len() > 0 {
		let mut assigned = false;
		if let Some(&x) = nodes.sub(&edges.iter().map(|(_, a)| *a).collect()).iter().next() {
			if let Some(w) = workers.iter_mut().filter(|w| w.is_none()).next() {
				nodes.remove(&x);
				*w = Some(Work {
					node: x,
					left: x as u8 - b'A' + 1 + 60,
				});
				assigned = true;
			}
		}
		if !assigned {
			let t = workers.iter().filter_map(|w| if let Some(ww) = w {Some(ww.left)} else {None}).min().unwrap();
			timer += t as i32;
			for w in workers.iter_mut() {
				if let Some(ww) = w {
					ww.left -= t;
					if ww.left == 0 {
						edges = edges.into_iter().filter(|(a, _)| *a != ww.node).collect();
						*w = None;
					}
				}
			}
		}
	}
	workers.iter().filter_map(|w| if let Some(ww) = w {Some(ww.left)} else {None}).max().unwrap() as i32 + timer
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", solve2(input));
	println!("{:?}", time.elapsed());
}
