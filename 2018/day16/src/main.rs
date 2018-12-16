use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, PartialEq, Debug)]
struct Registers([i32; 4]);

impl Registers {
	fn set(&mut self, v: i32, r: i32) {
		self.0[r as usize] = v;
	}

	fn get(&self, r: i32) -> i32 {
		self.0[r as usize]
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

const ALL_OPS: [Op; 16] = [
	Op::Addr,
	Op::Addi,
	Op::Mulr,
	Op::Muli,
	Op::Banr,
	Op::Bani,
	Op::Borr,
	Op::Bori,
	Op::Setr,
	Op::Seti,
	Op::Gtir,
	Op::Gtri,
	Op::Gtrr,
	Op::Eqir,
	Op::Eqri,
	Op::Eqrr,
];

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

fn possibilities(before: &Registers, a: i32, b: i32, c: i32, after: &Registers) -> HashSet<Op> {
	// Should return Iter<Item=Op>, but there are no coroutines, and I don't want to implement state...
	ALL_OPS
		.iter()
		.filter(|&op| {
			let mut r = before.clone();
			op.exec(&mut r, a, b, c);
			r == *after
		})
		.map(|&o| o)
		.collect()
}

struct ParsingInput<'a> {
	lines: std::str::Lines<'a>,
	//	lines: &'a mut Iterator<Item = String>,
}

impl<'a> ParsingInput<'a> {
	fn registers(&self, s: &str) -> Registers {
		let v = s
			.chars()
			.skip(9)
			.step_by(3)
			.take(4)
			.map(|c| (c as u8 - b'0') as i32)
			.collect::<Vec<_>>();
		Registers([v[0], v[1], v[2], v[3]])
	}

	fn new(lines: std::str::Lines<'a>) -> Self {
		Self { lines }
	}
}

fn parse_command(numbers: &str) -> (usize, i32, i32, i32) {
	let mut numbers = numbers.split_whitespace().map(|n| n.parse().unwrap());
	let opcode = numbers.next().unwrap() as usize;
	let a = numbers.next().unwrap();
	let b = numbers.next().unwrap();
	let c = numbers.next().unwrap();
	(opcode, a, b, c)
}

impl<'a> Iterator for ParsingInput<'a> {
	type Item = (Registers, usize, i32, i32, i32, Registers);

	fn next(&mut self) -> Option<Self::Item> {
		let before = self.lines.next().unwrap();
		if before.is_empty() {
			return None;
		}
		let numbers = self.lines.next().unwrap();
		let after = self.lines.next().unwrap();
		self.lines.next();

		let before = self.registers(&before);
		let (opcode, a, b, c) = parse_command(numbers);
		let after = self.registers(&after);
		Some((before, opcode, a, b, c, after))
	}
}

fn _solve1(input: &str) -> usize {
	let lines = input.lines();
	let input = ParsingInput::new(lines);
	input
		.filter(|(before, _opcode, a, b, c, after)| {
			possibilities(before, *a, *b, *c, after).len() >= 3
		})
		.count()
}

fn identify_ops(lines: std::str::Lines) -> (Vec<Op>, std::str::Lines) {
	let mut input = ParsingInput::new(lines);
	let mut codes: Vec<HashSet<Op>> = vec![ALL_OPS.iter().map(|&o| o).collect(); 16];
	while let Some((before, opcode, a, b, c, after)) = input.next() {
		codes[opcode] = codes[opcode]
			.intersection(&possibilities(&before, a, b, c, &after))
			.map(|&o| o)
			.collect();
	}
	let mut identified = HashSet::new();
	let mut found = vec![None; 16];
	while identified.len() < 16 {
		let (code, op) = codes
			.iter()
			.enumerate()
			.filter(|&(_, ops)| ops.len() == 1)
			.next()
			.unwrap();
		let op = *op.iter().next().unwrap();
		for c in &mut codes {
			c.remove(&op);
		}
		identified.insert(code);
		found[code] = Some(op);
	}
	(found.into_iter().map(|o| o.unwrap()).collect(), input.lines)
}

fn _solve2(input: &str) -> i32 {
	let lines = input.lines();
	// This should use Iterator::by_ref(), but it doesn't work
	let (ops, mut lines) = identify_ops(lines);
	let mut reg = Registers([0, 0, 0, 0]);
	lines.next();
	for line in lines {
		let (opcode, a, b, c) = parse_command(line);
		ops[opcode].exec(&mut reg, a, b, c);
	}
	reg.0[0]
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
	fn test_possibilities() {
		assert_eq!(
			possibilities(&Registers([3, 2, 1, 1]), 2, 1, 2, &Registers([3, 2, 2, 1])),
			[Op::Addi, Op::Mulr, Op::Seti]
				.into_iter()
				.map(|&o| o)
				.collect::<HashSet<_>>()
		);
	}

	#[test]
	fn test_parse() {
		let input = include_str!("../input.txt");
		let lines = input.lines();
		let mut input = ParsingInput::new(lines);
		assert_eq!(
			input.next(),
			Some((
				Registers([2, 0, 0, 1]),
				15,
				3,
				1,
				3,
				Registers([2, 0, 0, 1])
			))
		);
		assert_eq!(
			input.next(),
			Some((Registers([3, 2, 3, 3]), 4, 3, 3, 0, Registers([3, 2, 3, 3])))
		);
		assert_eq!(
			input.last(),
			Some((Registers([2, 1, 0, 2]), 6, 1, 3, 3, Registers([2, 1, 0, 0])))
		);
	}
}
