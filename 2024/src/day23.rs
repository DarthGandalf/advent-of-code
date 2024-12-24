use aoc_runner_derive::aoc;
use fnv::FnvHashSet;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn numerize(a: &str) -> i16 {
	a.chars().fold(0, |acc, x| acc * 36 + x.to_digit(36).unwrap() as i16)
}

fn stringize(a: i16) -> String {
	format!("{}{}", char::from_digit((a / 36) as u32, 36).unwrap(), char::from_digit((a % 36) as u32, 36).unwrap())
}

fn parse(input: &str) -> (BTreeSet<i16>, BTreeMap<i16, BTreeSet<i16>>) {
	let mut vertices = BTreeSet::<i16>::default();
	let mut edges = BTreeMap::<i16, BTreeSet<i16>>::default();
	for l in input.lines() {
		let (a, b) = l.split('-').collect_tuple().unwrap();
		let a = numerize(a);
		let b = numerize(b);
		edges.entry(a).or_default().insert(b);
		edges.entry(b).or_default().insert(a);
		vertices.insert(a);
		vertices.insert(b);
	}
	(vertices, edges)
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
	let (vertices, edges) = parse(input);
	let mut result = FnvHashSet::<String>::default();
	for v in &vertices {
		if char::from_digit((v/36) as u32, 36) != Some('t') {
			continue;
		}
		for u in edges.get(v).unwrap() {
			if v == u {
				continue;
			}
			for t in edges.get(u).unwrap() {
				if t == u || t == v {
					continue;
				}
				if edges.get(v).unwrap().contains(t) {
					let mut vec = vec![u, v, t];
					vec.sort_unstable();
					result.insert(vec.into_iter().join(","));
				}
			}
		}
	}
	result.len()
}

struct A<'a> {
	edges: &'a BTreeMap<i16, BTreeSet<i16>>,
	r: BTreeSet<i16>,
	p: BTreeSet<i16>,
	x: BTreeSet<i16>,
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron(mut a: A<'_>) -> Vec<BTreeSet<i16>> {
	if a.p.is_empty() && a.x.is_empty() {
		return vec![a.r];
	}
	let mut q = vec![];
	let u = if a.p.is_empty() { &a.x } else { &a.p }
		.iter()
		.next()
		.unwrap();
	let p: BTreeSet<i16> = a.p.difference(a.edges.get(u).unwrap()).cloned().collect();
	for &v in &p {
		let b = A {
			edges: a.edges,
			r: a.r.iter().cloned().chain(std::iter::once(v)).collect(),
			p: a.p.intersection(a.edges.get(&v).unwrap()).cloned().collect(),
			x: a.x.intersection(a.edges.get(&v).unwrap()).cloned().collect(),
		};
		q.extend(bron(b));
		a.p.remove(&v);
		a.x.insert(v);
	}
	q
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
	let (vertices, edges) = parse(input);
	bron(A {
		edges: &edges,
		r: Default::default(),
		p: vertices,
		x: Default::default(),
	})
	.into_iter()
	.max_by_key(|s| s.len())
	.unwrap()
	.into_iter()
	.map(stringize)
	.join(",")
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 7);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), "co,de,ka,ta");
	}
}
