use aoc_runner_derive::aoc;
use anyhow::Result;
use std::{collections::{BTreeMap, BTreeSet}, fmt::Write};
use itertools::Itertools;

#[derive(Copy, Clone, Ord, PartialEq, PartialOrd, Eq)]
struct Id(u16);

impl Id {
	fn from_string(a: &str) -> Self {
		Id(a.chars()
			.fold(0, |acc, x| acc * 36 + x.to_digit(36).unwrap() as u16))
	}
}

impl std::fmt::Debug for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.write_str(&self.to_string())
	}
}

impl std::fmt::Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.write_char(char::from_digit((self.0 / 36 / 36) as u32, 36).unwrap())?;
		f.write_char(char::from_digit(((self.0 / 36) % 36) as u32, 36).unwrap())?;
		f.write_char(char::from_digit((self.0 % 36) as u32, 36).unwrap())?;
		Ok(())
	}
}

enum Op {
	And,
	Or,
	Xor,
}

impl Op {
	fn parse(a: &str) -> Self {
		match a {
			"AND" => Op::And,
			"OR" => Op::Or,
			"XOR" => Op::Xor,
			_ => panic!("unknown op"),
		}
	}

	fn calc(&self, a: bool, b: bool) -> bool {
		match self {
			Op::And => a && b,
			Op::Or => a || b,
			Op::Xor => a ^ b,
		}
	}

	fn color(&self) -> &'static str {
		match self {
			Op::And => "lightskyblue1",
			Op::Or => "wheat3",
			Op::Xor => "thistle",
		}
	}
}

impl std::fmt::Display for Op {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Op::And => f.write_str("AND")?,
			Op::Or => f.write_str("OR")?,
			Op::Xor => f.write_str("XOR")?,
		}
		Ok(())
	}
}

struct Dep {
	op: Op,
	operands: [Id; 2],
	result: Id,
}

fn parse(input: &str) -> (BTreeMap<Id, bool>, Vec<Dep>) {
	let mut iter = input.split("\n\n");
	let start: BTreeMap<Id, bool> = iter.next().unwrap().lines().map(|l| {
		let mut it = l.split(": ");
		(Id::from_string(it.next().unwrap()), it.next().unwrap() == "1")
	}).collect();
	let deps = iter.next().unwrap().lines().map(|l| {
		let v = l.split(' ').collect_vec();
		Dep {
			op: Op::parse(v[1]),
			operands: [Id::from_string(v[0]), Id::from_string(v[2])],
			result: Id::from_string(v[4]),
		}
	}).collect_vec();
	(start, deps)
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u64 {
	let (mut values, deps) = parse(input);
	let mut g = petgraph::Graph::<Id, ()>::new();
	let mut nodes = BTreeMap::<Id, petgraph::graph::NodeIndex>::default();
	let mut how = BTreeMap::<Id, &Dep>::default();
	for d in &deps {
		let a = *nodes.entry(d.operands[0]).or_insert_with_key(|&k| g.add_node(k));
		let b = *nodes.entry(d.operands[1]).or_insert_with_key(|&k| g.add_node(k));
		let c = *nodes.entry(d.result).or_insert_with_key(|&k| g.add_node(k));
		g.add_edge(a, c, ());
		g.add_edge(b, c, ());
		how.insert(d.result, d);
	}
	let order = petgraph::algo::toposort(&g, None).unwrap();
	for d in order {
		if let Some(&rule) = how.get(&g[d]) {
			values.insert(rule.result, rule.op.calc(*values.get(&rule.operands[0]).unwrap(), *values.get(&rule.operands[1]).unwrap()));
		}
	}
	let mut result = 0;
	for (_, yes) in values.into_iter().rev().take_while(|(id, _)| id.0 >= 35 * 36 * 36) {
		result <<= 1;
		if yes {
			result += 1;
		}
	}
	result
}

fn fix(deps: &mut Vec<Dep>) {
	for d in deps {
		match d.result.to_string().as_str() {
			"rjm" => d.result = Id::from_string("wsv"),
			"wsv" => d.result = Id::from_string("rjm"),
			"z07" => d.result = Id::from_string("swt"),
			"swt" => d.result = Id::from_string("z07"),
			"z13" => d.result = Id::from_string("pqc"),
			"pqc" => d.result = Id::from_string("z13"),
			_ => {}
		}
	}
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> Result<u32> {
	let mut deps = parse(input).1;
	fix(&mut deps);
	let mut nodes = BTreeSet::<Id>::new();
	for d in &deps {
		nodes.insert(d.operands[0]);
		nodes.insert(d.operands[1]);
		nodes.insert(d.result);
	}
	let f = std::fs::File::create("24-2.dot")?;
	let mut f = std::io::BufWriter::new(f);
	use std::io::Write;
	writeln!(&mut f, "digraph g {{")?;
	for n in &nodes {
		let color = match n.to_string().chars().next().unwrap() {
			'x' => "lightgreen",
			'y' => "yellow",
			'z' => "hotpink",
			_ => "white",
		};
		writeln!(&mut f, "{n}[style=filled,fillcolor={color}];")?;
	}
	for (i, d) in deps.iter().enumerate() {
		writeln!(&mut f, "_{i}[style=filled,fillcolor={},label={}];", d.op.color(), d.op)?;
		writeln!(&mut f, "{} -> _{i};", d.operands[0])?;
		writeln!(&mut f, "{} -> _{i};", d.operands[1])?;
		writeln!(&mut f, "_{i} -> {};", d.result)?;
	}
	writeln!(&mut f, "}}")?;
	f.flush()?;
	Ok(0)
}

// rjm,wsv
// z07,swt
// z13,pqc

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT_1: &str = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
".trim_ascii();

	const INPUT_2: &str = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
".trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(Id::from_string("abc").to_string(), "abc");
		assert_eq!(part1(INPUT_1), 4);
		assert_eq!(part1(INPUT_2), 2024);
	}

	#[test]
	fn test2() {
	}
}
