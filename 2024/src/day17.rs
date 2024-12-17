use anyhow::{Result, bail};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::VecDeque;
use smallvec::SmallVec;

#[derive(Default)]
struct Computer<'a> {
	mem: &'a [i8],
	reg: [i64; 3],
	ip: usize,
	out: SmallVec<[i8; 16]>,
}

impl Computer<'_> {
	fn combo(&self, x: i8) -> Result<i64> {
		match x {
			0..=3 => Ok(x as i64),
			4..=6 => Ok(self.reg[x as usize - 4]),
			_ => bail!("combo operand {x}"),
		}
	}

	fn step(&mut self) -> Result<()> {
		let op = self.mem[self.ip];
		let operand = self.mem[self.ip + 1];
		self.ip += 2;
		match op {
			0 => {
				// adv
				self.reg[0] >>= self.combo(operand)?;
			}
			1 => {
				// bxl
				self.reg[1] ^= operand as i64;
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
				self.out.push(value as i8);
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

	fn run(mut self) -> Result<SmallVec<[i8; 16]>> {
		while self.ip < self.mem.len() {
			self.step()?;
		}
		Ok(self.out)
	}
}

fn parse(input: &str) -> ([i64; 3], Vec<i8>) {
	let mut iter = input.lines();
	let mut reg = [0; 3];
	for r in &mut reg {
		let line = iter.next().unwrap();
		*r = line.split_whitespace().last().unwrap().parse().unwrap();
	}
	let mem = iter
		.nth(1)
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.split(',')
		.flat_map(str::parse)
		.collect_vec();
	(reg, mem)
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
	let (reg, mem) = parse(input);
	let c = Computer {
		mem: &mem,
		reg,
		..Default::default()
	};
	c.run().unwrap().into_iter().join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
	let (_, mem) = parse(input);
	let mut q: VecDeque<(i64, usize)> = (0..=7).rev().map(|a| (a, 1)).collect();
	while let Some((a, d)) = q.pop_back() {
		if let Ok(out) = (Computer {
			mem: &mem,
			reg: [a, 0, 0],
			..Default::default()
		})
		.run()
		{
			if mem[mem.len() - d..] == *out {
				if d == mem.len() {
					return a;
				}
				for k in (0..=7).rev() {
					q.push_back((a * 8 + k, d + 1));
				}
			}
		}
	}
	0
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
			mem: &[0, 2],
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

// 2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0
//
// 2,4 bst-4: B = A % 8
// 1,7 bxl-7: B ^= 7
// 7,5 cdv-5: C = A >> B
// 1,7 bxl-7: B ^= 7
// 4,6 bxc-6: B ^= C
// 0,3 adv-3: A >>= 3
// 5,5 out-5: out B
// 3,0 jnz-0
