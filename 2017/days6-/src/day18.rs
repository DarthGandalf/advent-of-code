use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Val {
	Reg(char),
	Num(i64),
}

impl Val {
	pub fn parse(input: &str) -> Self {
		if let Ok(num) = input.parse::<i64>() {
			Val::Num(num)
		} else {
			Val::Reg(input.chars().next().unwrap())
		}
	}

	pub fn eval(&self, regs: &fnv::FnvHashMap<char, i64>) -> i64 {
		match self {
			Val::Reg(r) => *regs.get(r).unwrap_or(&0),
			Val::Num(n) => *n,
		}
	}
}

#[derive(Debug)]
enum Cmd {
	Snd(Val),
	Set(char, Val),
	Add(char, Val),
	Mul(char, Val),
	Mod(char, Val),
	Rcv(Val),
	Jgz(Val, Val),
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Cmd> {
	input
		.trim()
		.lines()
		.map(|line| {
			let mut split = line.split(' ');
			let cmd = split.next().unwrap();
			let arg1 = split.next().unwrap();
			let reg = arg1.chars().next().unwrap();
			let arg2 = split.next().unwrap_or("0");
			match cmd {
				"snd" => Cmd::Snd(Val::parse(arg1)),
				"set" => Cmd::Set(reg, Val::parse(arg2)),
				"add" => Cmd::Add(reg, Val::parse(arg2)),
				"mul" => Cmd::Mul(reg, Val::parse(arg2)),
				"mod" => Cmd::Mod(reg, Val::parse(arg2)),
				"rcv" => Cmd::Rcv(Val::parse(arg1)),
				"jgz" => Cmd::Jgz(Val::parse(arg1), Val::parse(arg2)),
				_ => panic!("unknown command {}", cmd),
			}
		})
		.collect()
}

#[derive(Clone)]
struct Instance<'a> {
	program: &'a [Cmd],
	regs: fnv::FnvHashMap<char, i64>,
	pc: i64,
	input: VecDeque<i64>,
	output: Option<i64>,
	output_counter: i64,
	waiting: bool,
	terminated: bool,
	part2: bool,
}

impl<'a> Instance<'a> {
	fn step(&mut self) {
		if !(0..self.program.len() as i64).contains(&self.pc) {
			self.terminated = true;
			return;
		}
		self.waiting = false;
		match &self.program[self.pc as usize] {
			Cmd::Snd(val) => {
				self.output = Some(val.eval(&self.regs));
				self.output_counter += 1;
			}
			Cmd::Set(reg, val) => {
				let val = val.eval(&self.regs);
				self.regs.insert(*reg, val);
			}
			Cmd::Add(reg, val) => {
				let val = val.eval(&self.regs);
				*self.regs.entry(*reg).or_default() += val;
			}
			Cmd::Mul(reg, val) => {
				let val = val.eval(&self.regs);
				*self.regs.entry(*reg).or_default() *= val;
			}
			Cmd::Mod(reg, val) => {
				let val = val.eval(&self.regs);
				*self.regs.entry(*reg).or_default() %= val;
			}
			Cmd::Rcv(val) => {
				if self.part2 {
					match val {
						Val::Reg(reg) => {
							if self.input.is_empty() {
								self.waiting = true;
								return;
							} else {
								self.regs.insert(*reg, self.input.pop_front().unwrap());
							}
						}
						Val::Num(_) => panic!(),
					}
				} else {
					let val = val.eval(&self.regs);
					if val != 0 {
						self.waiting = true;
						return;
					}
				}
			}
			Cmd::Jgz(cond, off) => {
				let cond = cond.eval(&self.regs);
				if cond > 0 {
					let off = off.eval(&self.regs);
					self.pc += off;
					self.pc -= 1;
				}
			}
		}
		self.pc += 1;
	}
}

#[aoc(day18, part1)]
fn part1(input: &[Cmd]) -> i64 {
	let mut p = Instance {
		program: input,
		regs: fnv::FnvHashMap::default(),
		pc: 0,
		input: VecDeque::new(),
		output: None,
		output_counter: 0,
		waiting: false,
		terminated: false,
		part2: false,
	};
	while !p.waiting {
		p.step();
	}
	p.output.unwrap()
}

#[aoc(day18, part2)]
fn part2(input: &[Cmd]) -> i64 {
	let mut p = vec![
		Instance {
			program: input,
			regs: fnv::FnvHashMap::default(),
			pc: 0,
			input: VecDeque::new(),
			output: None,
			output_counter: 0,
			waiting: false,
			terminated: false,
			part2: true,
		};
		2
	];
	p[1].regs.insert('p', 1);
	loop {
		for x in &mut p {
			x.step();
		}
		for i in 0..=1 {
			if let Some(x) = p[i].output {
				p[1 - i].input.push_back(x);
				p[i].output = None;
			}
		}
		if p[0].terminated && p[1].terminated
			|| p[0].waiting && p[1].waiting && p[0].input.is_empty() && p[1].input.is_empty()
		{
			return p[1].output_counter;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		let input = parse(
			"
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2",
		);
		assert_eq!(part1(&input), 4);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day18.txt"));
		assert_eq!(part1(&input), 8600);
		assert_eq!(part2(&input), 7239);
	}
}
