use aoc_runner_derive::aoc;
use fnv::FnvHashSet;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn numerize(a: &str) -> u16 {
	a.chars()
		.fold(0, |acc, x| acc * 36 + x.to_digit(36).unwrap() as u16)
}

fn stringize(a: u16) -> String {
	format!(
		"{}{}",
		char::from_digit((a / 36) as u32, 36).unwrap(),
		char::from_digit((a % 36) as u32, 36).unwrap()
	)
}

fn parse(input: &str) -> (BTreeSet<u16>, BTreeMap<u16, BTreeSet<u16>>) {
	let mut vertices = BTreeSet::<u16>::default();
	let mut edges = BTreeMap::<u16, BTreeSet<u16>>::default();
	for l in input.lines() {
		let (a, b) = l.split('-').map(numerize).collect_tuple().unwrap();
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
	let mut result = FnvHashSet::<u32>::default();
	for v in &vertices {
		if char::from_digit((v / 36) as u32, 36) != Some('t') {
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
					let mut vec = [u, v, t];
					vec.sort_unstable();
					result.insert(*vec[0] as u32 * 36 * 36 + *vec[1] as u32 * 36 + *vec[2] as u32);
				}
			}
		}
	}
	result.len()
}

struct A<'a> {
	edges: &'a BTreeMap<u16, BTreeSet<u16>>,
	r: BTreeSet<u16>,
	p: BTreeSet<u16>,
	x: BTreeSet<u16>,
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron(mut a: A<'_>) -> Vec<BTreeSet<u16>> {
	if a.p.is_empty() && a.x.is_empty() {
		return vec![a.r];
	}
	let mut q = vec![];
	let u = if a.p.is_empty() { &a.x } else { &a.p }
		.iter()
		.next()
		.unwrap();
	let p: BTreeSet<u16> = a.p.difference(a.edges.get(u).unwrap()).cloned().collect();
	for &v in &p {
		let b = A {
			edges: a.edges,
			r: a.r.iter().cloned().chain(std::iter::once(v)).collect(),
			p: a.p
				.intersection(a.edges.get(&v).unwrap())
				.cloned()
				.collect(),
			x: a.x
				.intersection(a.edges.get(&v).unwrap())
				.cloned()
				.collect(),
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
