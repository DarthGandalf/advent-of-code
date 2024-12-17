use aoc_runner_derive::aoc;
use anyhow::{Result, bail};
use itertools::Itertools;

#[derive(Default)]
struct Computer {
	mem: Vec<i64>,
	reg: [i64; 3],
	ip: usize,
	out: Vec<i64>,
	outlen: usize,
	part2: bool,
}

impl Computer {
	fn combo(&self, x: i64) -> Result<i64> {
		match x {
			0..=3 => Ok(x),
			4..=6 => Ok(self.reg[x as usize-4]),
			_ => bail!("combo operand {x}"),
		}
	}

	fn step(&mut self) -> anyhow::Result<()> {
		let op = self.mem[self.ip];
		let operand = self.mem[self.ip+1];
		self.ip += 2;
		match op {
			0 => {
				// adv
				self.reg[0] >>= self.combo(operand)?;
			}
			1 => {
				// bxl
				self.reg[1] ^= operand;
			}
			2 => {
				// bst
				self.reg[1] = self.combo(operand)? % 8;
			}
			3 => {
				// jnz
				if self.reg[0] != 0 {
					self.ip = operand as usize;
				}
			}
			4 => {
				// bxc
				self.reg[1] ^= self.reg[2];
			}
			5 => {
				// out
				let value = self.combo(operand)? % 8;
				if self.part2 {
					if self.outlen >= self.mem.len() {
						bail!("too long output");
					}
					if self.mem[self.outlen] != value {
						bail!("wrong out value");
					}
					self.outlen += 1;
				} else {
					self.out.push(value);
				}
			}
			6 => {
				// bdv
				self.reg[1] = self.reg[0] >> self.combo(operand)?;
			}
			7 => {
				// cdv
				self.reg[2] = self.reg[0] >> self.combo(operand)?;
			}
			_ => bail!("operator {op}"),
		}
		Ok(())
	}

	fn reset(&mut self, a: i64) {
		self.reg[0] = a;
		self.reg[1] = 0;
		self.reg[2] = 0;
		self.outlen = 0;
	}
}

fn parse(input: &str) -> Computer {
	let mut iter = input.lines();
	let mut c = Computer::default();
	for r in &mut c.reg {
		let line = iter.next().unwrap();
		*r = line.split_whitespace().last().unwrap().parse().unwrap();
	}
	c.mem = iter
		.skip(1)
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.split(',')
		.flat_map(str::parse)
		.collect_vec();
	c
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
	let mut c = parse(input);
	while c.ip < c.mem.len() {
		c.step().unwrap();
	}
	c.out.into_iter().join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
	let mut c = parse(input);
	c.part2 = true;
	'loo: for a in 0.. {
		c.reset(a);
		let mut x = 0;
		while c.ip < c.mem.len() && x < 100 {
			x += 1;
			let r = c.step();
			if r.is_err() {
				continue 'loo;
			}
		}
		if c.ip >= c.mem.len() && c.outlen == c.mem.len() {
			return a;
		}
	}
	panic!("no result");
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
	.trim_ascii();

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), "4,6,3,5,6,3,5,2,1,0");
	}

	#[test]
	fn test_adv() {
		let mut c = Computer {
			mem: vec![0, 2],
			reg: [100, 0, 0],
			..Default::default()
		};
		c.step().unwrap();
		assert_eq!(c.reg[0], 25);
	}

	#[test]
	fn test2() {
		//		assert_eq!(part2(INPUT_1), 45);
	}
}
