use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug)]
struct Registers([i32; 6]);

impl Registers {
	fn set(&mut self, v: i32, r: i32) {
		self.0[r as usize] = v;
	}

	fn get(&self, r: i32) -> i32 {
		self.0[r as usize]
	}
}

#[derive(Debug, Clone, Copy)]
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
	fn exec(self, reg: &mut Registers, a: i32, b: i32, c: i32) {
		match self {
			Op::Addr => reg.set(reg.get(a) + reg.get(b), c),
			Op::Addi => reg.set(reg.get(a) + b, c),
			Op::Mulr => reg.set(reg.get(a) * reg.get(b), c),
			Op::Muli => reg.set(reg.get(a) * b, c),
			Op::Banr => reg.set(reg.get(a) & reg.get(b), c),
			Op::Bani => reg.set(reg.get(a) & b, c),
			Op::Borr => reg.set(reg.get(a) | reg.get(b), c),
			Op::Bori => reg.set(reg.get(a) | b, c),
			Op::Setr => reg.set(reg.get(a), c),
			Op::Seti => reg.set(a, c),
			Op::Gtir => reg.set((a > reg.get(b)) as i32, c),
			Op::Gtri => reg.set((reg.get(a) > b) as i32, c),
			Op::Gtrr => reg.set((reg.get(a) > reg.get(b)) as i32, c),
			Op::Eqir => reg.set((a == reg.get(b)) as i32, c),
			Op::Eqri => reg.set((reg.get(a) == b) as i32, c),
			Op::Eqrr => reg.set((reg.get(a) == reg.get(b)) as i32, c),
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

	fn exec(&self, mut reg: &mut Registers) {
		self.op.exec(&mut reg, self.a, self.b, self.c);
	}
}

fn _solve1(input: &str) -> i32 {
	let mut lines = input.lines();
	let ipreg: i32 = (lines.next().unwrap().chars().skip(4).next().unwrap() as u8 - b'0') as i32;
	let program: Vec<_> = lines.map(|l| Instruction::new(l)).collect();
	let mut reg = Registers([0; 6]);
	while reg.get(ipreg) != 28 {
		let ip = reg.get(ipreg);
		if ip < 0 || ip as usize >= program.len() {
			break;
		}
		program[ip as usize].exec(&mut reg);
		//		println!("{:?}", &reg);
		reg.set(reg.get(ipreg) + 1, ipreg);
	}
	reg.get(1)
}

fn _solve2(input: &str) -> i32 {
	let mut lines = input.lines();
	let ipreg: i32 = (lines.next().unwrap().chars().skip(4).next().unwrap() as u8 - b'0') as i32;
	let program: Vec<_> = lines.map(|l| Instruction::new(l)).collect();
	let mut reg = Registers([0; 6]);
	let mut set = HashSet::new();
	let mut prev = 0;
	loop {
		let ip = reg.get(ipreg);
		if ip < 0 || ip as usize >= program.len() {
			break;
		}
		program[ip as usize].exec(&mut reg);
		//		println!("{:?}", &reg);
		reg.set(reg.get(ipreg) + 1, ipreg);
		if reg.get(ipreg) == 28 {
			if set.contains(&reg.get(1)) {
				return prev;
			}
			prev = reg.get(1);
			set.insert(prev);
		}
	}
	0
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
