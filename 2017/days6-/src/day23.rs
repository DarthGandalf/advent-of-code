use aoc_runner_derive::{aoc, aoc_generator};

type Val = crate::day18::Val;

#[derive(Debug)]
pub enum Cmd {
	Set(char, Val),
	Sub(char, Val),
	Mul(char, Val),
	Jnz(Val, Val),
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Cmd> {
	input
		.trim()
		.lines()
		.map(|line| {
			let mut split = line.split(' ');
			let cmd = split.next().unwrap();
			let arg1 = split.next().unwrap();
			let reg = arg1.chars().next().unwrap();
			let arg2 = split.next().unwrap_or_else(|| "0");
			match cmd {
				"set" => Cmd::Set(reg, Val::parse(arg2)),
				"sub" => Cmd::Sub(reg, Val::parse(arg2)),
				"mul" => Cmd::Mul(reg, Val::parse(arg2)),
				"jnz" => Cmd::Jnz(Val::parse(arg1), Val::parse(arg2)),
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
	muls: usize,
	terminated: bool,
}

impl<'a> Instance<'a> {
	fn step(&mut self) {
		if !(0..self.program.len() as i64).contains(&self.pc) {
			self.terminated = true;
			return;
		}
		match &self.program[self.pc as usize] {
			Cmd::Set(reg, val) => {
				let val = val.eval(&self.regs);
				self.regs.insert(*reg, val);
			}
			Cmd::Sub(reg, val) => {
				let val = val.eval(&self.regs);
				*self.regs.entry(*reg).or_default() -= val;
			}
			Cmd::Mul(reg, val) => {
				let val = val.eval(&self.regs);
				*self.regs.entry(*reg).or_default() *= val;
				self.muls += 1;
			}
			Cmd::Jnz(cond, off) => {
				let cond = cond.eval(&self.regs);
				if cond != 0 {
					let off = off.eval(&self.regs);
					self.pc += off;
					self.pc -= 1;
				}
			}
		}
		self.pc += 1;
	}
}

#[aoc(day23, part1)]
fn part1(input: &[Cmd]) -> usize {
	let mut p = Instance {
		program: input,
		regs: fnv::FnvHashMap::default(),
		pc: 0,
		muls: 0,
		terminated: false,
	};
	while !p.terminated {
		p.step();
	}
	p.muls
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2017/day23.txt"));
		assert_eq!(part1(&input), 9409);
	}
}
