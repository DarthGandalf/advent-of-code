use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

struct NIC {
	input: crossbeam::channel::Sender<(crate::intcode::Type, crate::intcode::Type)>,
}

impl NIC {
	fn new(
		program: &[crate::intcode::Type],
		address: crate::intcode::Type,
		output: crossbeam::channel::Sender<(u8, crate::intcode::Type, crate::intcode::Type)>,
	) -> Self {
		let (ti, ri) = crossbeam::channel::bounded(2);
		let (tw, rw) = crossbeam::channel::bounded(0);
		let (to, ro) = crossbeam::channel::bounded(0);
		let (e, _) = crossbeam::channel::unbounded();
		let _ = ti.send(address);
		let mut computer = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, e);
		std::thread::spawn(move || computer.run(None));
		let _ = rw.recv();
		let (input_t, input_r) = crossbeam::channel::unbounded();
		std::thread::spawn(move || -> anyhow::Result<()> {
			loop {
				crossbeam::channel::select! {
					recv(rw) -> _ => {
						if let Ok((x, y)) = input_r.recv_timeout(std::time::Duration::from_millis(1)) {
							ti.send(x)?;
							let _ = rw.recv();
							ti.send(y)?;
						} else {
							ti.send(-1)?;
						}
					}
					recv(ro) -> receiver => {
						let x = ro.recv()?;
						let y = ro.recv()?;
						output.send((receiver? as u8, x, y))?;
					}
				}
			}
		});
		Self { input: input_t }
	}
}

#[aoc(day23, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	let (bust, busr) = crossbeam::channel::unbounded();
	let inputs: Vec<_> = (0..50)
		.map(|i| NIC::new(program, i, bust.clone()))
		.collect();
	for (receiver, x, y) in busr.into_iter() {
		println!("{} {} {}", receiver, x, y);
		if receiver == 255 {
			return Ok(y);
		}
		inputs[receiver as usize].input.send((x, y))?;
	}
	anyhow::bail!("no result");
}

#[aoc(day23, part2)]
fn part2(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	let (bust, busr) = crossbeam::channel::unbounded();
	let inputs: Vec<_> = (0..50)
		.map(|i| NIC::new(program, i, bust.clone()))
		.collect();
	let mut last = None;
	let mut sent = None;
	loop {
		if let Ok((receiver, x, y)) = busr.recv_timeout(std::time::Duration::from_millis(100)) {
			println!("{} {} {}", receiver, x, y);
			if receiver == 255 {
				last = Some((x, y));
			} else {
				inputs[receiver as usize].input.send((x, y))?;
			}
		} else {
			println!("NAT, {:?}", sent);
			if sent == last {
				return Ok(last.unwrap_or_default().1);
			}
			inputs[0].input.send(last.unwrap_or_default())?;
			sent = last;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day23.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 18513);
		assert_eq!(part2(&input).unwrap(), 13286);
	}
}
