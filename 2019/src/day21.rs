use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
//use rayon::prelude::*;

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

fn brute(program: &[crate::intcode::Type], attempt: usize) -> Option<crate::intcode::Type> {
	const ALL_INSTRUCTIONS: &[&str] = &[
		"", "NOT A J", "NOT A T", "NOT B J", "NOT B T", "NOT C J", "NOT C T", "NOT D J", "NOT D T",
		"NOT T J", "NOT T T", "NOT J J", "NOT J T", "AND A J", "AND A T", "AND B J", "AND B T",
		"AND C J", "AND C T", "AND D J", "AND D T", "AND T J", "AND T T", "AND J J", "AND J T",
		"OR A J", "OR A T", "OR B J", "OR B T", "OR C J", "OR C T", "OR D J", "OR D T", "OR T J",
		"OR T T", "OR J J", "OR J T",
	];
	let mut number: usize = attempt;
	let mut input = String::new();
	while number > 0 {
		input.extend(ALL_INSTRUCTIONS[number % ALL_INSTRUCTIONS.len()].chars());
		input.push('\n');
		number /= ALL_INSTRUCTIONS.len();
	}
	println!("Attempt {}:\n{}", attempt, input);
	input.extend("WALK\n".chars());
	let (ti, ri) = crossbeam::channel::unbounded();
	let (to, ro) = crossbeam::channel::unbounded();
	let (tw, _) = crossbeam::channel::unbounded();
	let (te, _) = crossbeam::channel::unbounded();
	let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	for c in input.chars() {
		let _ = ti.send(c as u8 as crate::intcode::Type);
	}
	for c in ro.into_iter() {
		if c > 200 {
			return Some(c);
		}
		//print!("{}", c as u8 as char);
	}
	None
}

#[aoc(day21, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	/*let succ_attempt = (0..10000000000i64)
		.into_par_iter()
		.find_any(|attempt| brute(program, *attempt as usize).is_some())
		.none_err()?;
	Ok(succ_attempt) */
	/*
	NOT D T
	OR C T
	AND A T
	NOT T J
	*/
	Ok(brute(program, 476161).none_err()?)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day21.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 19352638);
		//assert_eq!(part2(&input).unwrap(), 1045393);
	}
}
