use aoc_runner_derive::aoc;
use fnv::FnvHashSet;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn parse(input: &str) -> (BTreeSet<&str>, BTreeMap<&str, BTreeSet<&str>>) {
	let mut vertices = BTreeSet::<&str>::default();
	let mut edges = BTreeMap::<&str, BTreeSet<&str>>::default();
	for l in input.lines() {
		let (a, b) = l.split('-').collect_tuple().unwrap();
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
		if !v.starts_with('t') {
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
	edges: &'a BTreeMap<&'a str, BTreeSet<&'a str>>,
	r: BTreeSet<&'a str>,
	p: BTreeSet<&'a str>,
	x: BTreeSet<&'a str>,
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron(mut a: A<'_>) -> Vec<BTreeSet<&str>> {
	if a.p.is_empty() && a.x.is_empty() {
		return vec![a.r];
	}
	let p = a.p.clone();
	let mut q = vec![];
	for &v in &p {
		let b = A {
			edges: a.edges,
			r: a.r.iter().cloned().chain(std::iter::once(v)).collect(),
			p: a.p.intersection(a.edges.get(v).unwrap()).cloned().collect(),
			x: a.x.intersection(a.edges.get(v).unwrap()).cloned().collect(),
		};
		q.extend(bron(b));
		a.p.remove(v);
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
