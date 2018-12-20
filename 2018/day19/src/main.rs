use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Op {
	Addr,
	Addi,
	Mulr,
	Muli,
	Banr,
	Bani,
	Borr,
	Bori,
	Setr,
	Seti,
	Gtir,
	Gtri,
	Gtrr,
	Eqir,
	Eqri,
	Eqrr,
}

impl Op {
	fn translate(self, a: i32, b: i32, c: i32) -> String {
		match self {
			Op::Addr => format!("r{} = r{} + r{}", c, a, b),
			Op::Addi => format!("r{} = r{} + {}", c, a, b),
			Op::Mulr => format!("r{} = r{} * r{}", c, a, b),
			Op::Muli => format!("r{} = r{} * {}", c, a, b),
			Op::Banr => format!("r{} = r{} & r{}", c, a, b),
			Op::Bani => format!("r{} = r{} & {}", c, a, b),
			Op::Borr => format!("r{} = r{} | r{}", c, a, b),
			Op::Bori => format!("r{} = r{} | {}", c, a, b),
			Op::Setr => format!("r{} = r{}", c, a),
			Op::Seti => format!("r{} = {}", c, a),
			Op::Gtir => format!("r{} = {} > r{}", c, a, b),
			Op::Gtri => format!("r{} = r{} > {}", c, a, b),
			Op::Gtrr => format!("r{} = r{} > r{}", c, a, b),
			Op::Eqir => format!("r{} = {} == r{}", c, a, b),
			Op::Eqri => format!("r{} = r{} == {}", c, a, b),
			Op::Eqrr => format!("r{} = r{} == r{}", c, a, b),
		}
	}
}

#[derive(Debug)]
struct Instruction {
	op: Op,
	a: i32,
	b: i32,
	c: i32,
}

impl Instruction {
	fn new(line: &str) -> Self {
		let mut words = line.split_whitespace();
		let op = match words.next().unwrap() {
			"addr" => Op::Addr,
			"addi" => Op::Addi,
			"mulr" => Op::Mulr,
			"muli" => Op::Muli,
			"banr" => Op::Banr,
			"bani" => Op::Bani,
			"borr" => Op::Borr,
			"bori" => Op::Bori,
			"setr" => Op::Setr,
			"seti" => Op::Seti,
			"gtir" => Op::Gtir,
			"gtri" => Op::Gtri,
			"gtrr" => Op::Gtrr,
			"eqir" => Op::Eqir,
			"eqri" => Op::Eqri,
			"eqrr" => Op::Eqrr,
			_ => panic!("unknown instruction"),
		};
		let mut numbers = words.map(|n| n.parse().unwrap());
		let a = numbers.next().unwrap();
		let b = numbers.next().unwrap();
		let c = numbers.next().unwrap();
		Instruction { op, a, b, c }
	}

	fn translate(&self) -> String {
		self.op.translate(self.a, self.b, self.c)
	}
}

fn _solve1(input: &str) -> i32 {
	solve(&input, "")
}

fn _solve2(input: &str) -> i32 {
	solve(&input, "r0=1;")
}

#[derive(Debug)]
enum Block {
	Loop(usize, usize),
	If(usize, usize),
}

fn solve(input: &str, init: &str) -> i32 {
	let mut lines = input.lines();
	let ipreg: i32 = (lines.next().unwrap().chars().skip(4).next().unwrap() as u8 - b'0') as i32;
	let program: Vec<_> = lines.map(|l| Instruction::new(l)).enumerate().collect();
	let temp_file = "/tmp/rustcxx";
	let mut child = std::process::Command::new("g++")
		.arg("-x")
		.arg("c++")
		.arg("/dev/stdin")
		.arg("-o")
		.arg("/dev/stdout")
		.arg("-E")
		//.arg(temp_file)
		.arg("-O3")
		.stdin(std::process::Stdio::piped())
		.spawn()
		.unwrap();
	let comparisons: HashSet<_> = [Op::Eqir, Op::Eqri, Op::Eqrr, Op::Gtir, Op::Gtri, Op::Gtrr]
		.into_iter()
		.collect();
	{
		use std::io::Write;
		let stdin = child.stdin.as_mut().unwrap();
		write!(stdin, "#include <iostream>\n").unwrap();
		write!(stdin, "int main() {{\n").unwrap();
		for i in 0..6 {
			write!(stdin, "  int r{} = {};\n", i, 0).unwrap();
		}
		write!(stdin, "{}", init).unwrap();
		write!(stdin, "  while (true) {{\n").unwrap();
		write!(stdin, "    switch (r{}) {{\n", ipreg).unwrap();
		for (i, _) in program.iter() {
			write!(stdin, "      case {}:\n        ", i).unwrap();
			let mut blocks = Vec::new();
			for w in program.windows(3) {
				let mut w = w.iter();
				let (j, instr1) = w.next().unwrap();
				let (_, instr2) = w.next().unwrap();
				let (_, instr3) = w.next().unwrap();
				if comparisons.contains(&instr1.op)
					&& (instr1.c == instr2.b && instr2.a == ipreg
						|| instr1.c == instr2.a && instr2.b == ipreg)
					&& instr2.c == ipreg
					&& instr2.op == Op::Addr
					&& instr3.op == Op::Seti
					&& instr3.c == ipreg
					&& instr3.a >= *i as i32
					&& instr3.a < *j as i32
				{
					if let Some(Block::Loop(prev, _)) = blocks.get(blocks.len() - 1) {
						if *prev as i32 >= instr3.a {
							blocks.push(Block::Loop(instr3.a as usize, *j));
						}
					} else {
						blocks.push(Block::Loop(instr3.a as usize, *j));
					}
				}
				if comparisons.contains(&instr1.op)
					&& (instr1.c == instr2.b && instr2.a == ipreg
						|| instr1.c == instr2.a && instr2.b == ipreg)
					&& instr2.c == ipreg
					&& instr2.op == Op::Addr
					&& instr3.op == Op::Addi
					&& instr3.c == ipreg
					&& instr3.b > 0
				{
					blocks.push(Block::If(*j, instr3.b as usize));
				}
			}
			let ifs: HashMap<_, _> = blocks
				.iter()
				.filter_map(|b| {
					if let Block::If(start, off) = b {
						Some((start, start + off))
					} else {
						None
					}
				})
				.collect();
			let loops: Vec<(_, _)> = blocks
				.iter()
				.filter_map(|b| {
					if let Block::Loop(start, cond) = b {
						Some((start, cond))
					} else {
						None
					}
				})
				.collect();
			let loop_starts: HashSet<_> = loops.iter().map(|&(s, _)| s).collect();
			let loop_ends: HashSet<_> = loops.iter().map(|&(_, e)| e).collect();
			let mut inside_if: Option<(usize, usize)> = None;
			let mut num_loops = 0;
			let mut inside_loop_end = None;
			for (j, inst) in program[*i..].iter() {
				if let Some(_if) = inside_if {
					if _if.0 > *j {
						continue;
					}
					if _if.1 == *j {
						write!(stdin, " }} r{} = {};\n       ", ipreg, j).unwrap();
						inside_if = None;
					} else {
						write!(stdin, "  ").unwrap();
					}
				} else if let Some(_loop) = inside_loop_end {
					if _loop > *j {
						continue;
					}
					inside_loop_end = None;
				}
				if loop_starts.contains(&j) {
					write!(stdin, " do {{\n       ").unwrap();
					num_loops += 1;
				}
				if ifs.contains_key(&j) {
					write!(stdin, " \"{}\"; {};\n       ", j, inst.translate()).unwrap();
					write!(stdin, " if (r{}) {{\n       ", inst.c).unwrap();
					let _if = ifs.get(&j).unwrap();
					inside_if = Some((j + 3, _if + 3));
				} else if loop_ends.contains(&j) {
					write!(
						stdin,
						" \"{}\"; {}; ++r{};\n       ",
						j,
						inst.translate(),
						ipreg
					)
					.unwrap();
					write!(
						stdin,
						" }} while (!r{}); r{} = {};\n       ",
						inst.c,
						ipreg,
						j + 3
					)
					.unwrap();
					inside_loop_end = Some(j + 3);
					num_loops -= 1;
				} else {
					write!(stdin, " \"{}\"; {};", j, inst.translate()).unwrap();
					write!(stdin, " ++r{};\n       ", ipreg).unwrap();
					if inst.c == ipreg {
						break;
					}
				}
			}
			for _ in 0..num_loops {
				write!(stdin, " }} while (false);\n       ").unwrap();
			}
			write!(stdin, " break;\n").unwrap();
		}
		write!(stdin, "      default:\n").unwrap();
		write!(stdin, "        std::cout << r0;\n").unwrap();
		write!(stdin, "        return 0;\n").unwrap();
		write!(stdin, "    }}\n").unwrap();
		write!(stdin, "  }}\n").unwrap();
		write!(stdin, "}}\n").unwrap();
	}
	assert!(child.wait().unwrap().success());
	std::process::Command::new(temp_file)
		.output()
		.unwrap()
		.stdout
		.iter()
		.map(|&b| b as char)
		.collect::<String>()
		.parse()
		.unwrap()
	/*	let mut program = "#include <iostream>\n".to_owned();
		program.add_assign("int main() {\n");
		program.add_assign("}\n");
		loop {
		let ip = reg.get(ipreg);
		if ip < 0 || ip as usize >= program.len() {
		break;
		}
		program[ip as usize].exec(&mut reg);
	//		println!("{:?}", &reg);
	reg.set(reg.get(ipreg) + 1, ipreg);
	}*/
	//	reg.get(0)
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
}
