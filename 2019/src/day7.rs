use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

#[aoc(day7, part1)]
fn part1(program: &[i32]) -> Result<i32, crate::Error> {
	use fallible_iterator::FallibleIterator;
	let result = fallible_iterator::convert(permute::permutations_of(&[0, 1, 2, 3, 4]).map(
		|x| -> Result<i32, crate::Error> {
			let mut signal = vec![0];
			for &amp in x {
				signal.insert(0, amp);
				let new_signal = crate::intcode::run_copy(program, &signal, None)?.0;
				signal = new_signal;
			}
			Ok(signal[0])
		},
	))
	.max()??;
	Ok(result)
}

#[aoc(day7, part2)]
fn part2(program: &[i32]) -> Result<i32, crate::Error> {
	use fallible_iterator::FallibleIterator;
	let result = fallible_iterator::convert(permute::permutations_of(&[5, 6, 7, 8, 9]).map(
		|mut x| -> Result<i32, crate::Error> {
			let (txa, rxa) = std::sync::mpsc::channel();
			let (txab, rxab) = std::sync::mpsc::channel();
			let (txbc, rxbc) = std::sync::mpsc::channel();
			let (txcd, rxcd) = std::sync::mpsc::channel();
			let (txde, rxde) = std::sync::mpsc::channel();
			let (txe, rxe) = std::sync::mpsc::channel();
			txa.send(*x.next()?)?;
			txab.send(*x.next()?)?;
			txbc.send(*x.next()?)?;
			txcd.send(*x.next()?)?;
			txde.send(*x.next()?)?;
			txa.send(0)?;
			let mut ampa = crate::intcode::Computer::new(program.to_vec(), rxa, txab);
			let mut ampb = crate::intcode::Computer::new(program.to_vec(), rxab, txbc);
			let mut ampc = crate::intcode::Computer::new(program.to_vec(), rxbc, txcd);
			let mut ampd = crate::intcode::Computer::new(program.to_vec(), rxcd, txde);
			let mut ampe = crate::intcode::Computer::new(program.to_vec(), rxde, txe);
			std::thread::spawn(move || ampa.run(None));
			std::thread::spawn(move || ampb.run(None));
			std::thread::spawn(move || ampc.run(None));
			std::thread::spawn(move || ampd.run(None));
			std::thread::spawn(move || ampe.run(None));
			let mut last = -1;
			for z in rxe {
				last = z;
				let _ = txa.send(z);
			}
			Ok(last)
		},
	))
	.max()??;
	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(
			part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
			Ok(43210)
		);
		assert_eq!(
			part1(&[
				3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
				23, 99, 0, 0
			]),
			Ok(54321)
		);
		assert_eq!(
			part1(&[
				3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
				1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
			]),
			Ok(65210)
		);
	}

	#[test]
	fn test_part2() {
		assert_eq!(
			part2(&[
				3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
				-1, 28, 1005, 28, 6, 99, 0, 0, 5
			]),
			Ok(139629729)
		);
		assert_eq!(
			part2(&[
				3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
				54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
				55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
			]),
			Ok(18216)
		);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day7.txt")).unwrap();
		assert_eq!(part1(&input), Ok(929800));
		assert_eq!(part2(&input), Ok(15432220));
	}
}
