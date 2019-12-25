use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

#[aoc(day25, part1)]
fn part1(program: &[crate::intcode::Type]) -> anyhow::Result<i32> {
	let mut rl = rustyline::Editor::<()>::new();
	let start = "
west
take fixed point
north
take sand
south
east
east
take asterisk
north
north
take hypercube
north
take coin
north
take easter egg
south
south
south
west
north
take spool of cat6
north
take shell
west
";
	let items: Vec<&str> = start
		.lines()
		.filter(|s| s.starts_with("take "))
		.map(|s| s.split_at(5).1)
		.collect();
	for mut i in 0..256 {
		let mut input = start.to_string();
		for it in &items {
			if i % 2 == 0 {
				input.push_str(&format!("drop {}\n", it));
			}
			i /= 2;
		}
		input.push_str("inv\nnorth\n");
		let (ti, ri) = crossbeam::channel::unbounded();
		let (to, ro) = crossbeam::channel::unbounded();
		let (tw, _) = crossbeam::channel::unbounded();
		let (te, re) = crossbeam::channel::unbounded();
		for c in input.chars() {
			ti.send(c as u8 as crate::intcode::Type)?;
		}
		let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
		std::thread::spawn(move || robot.run(None));
		std::thread::spawn(move || {
			for c in ro.into_iter() {
				print!("{}", c as u8 as char);
			}
		});
		loop {
			std::thread::sleep(std::time::Duration::from_millis(10));
			if re.try_recv().is_ok() {
				break;
			}
			let readline = rl.readline("> ");
			match readline {
				Ok(line) => {
					rl.add_history_entry(line.as_str());
					for c in line.chars() {
						ti.send(c as u8 as crate::intcode::Type)?;
					}
					ti.send('\n' as u8 as crate::intcode::Type)?;
				}
				Err(rustyline::error::ReadlineError::Interrupted) => {
					println!("CTRL-C");
					break;
				}
				Err(rustyline::error::ReadlineError::Eof) => {
					println!("CTRL-D");
					break;
				}
				Err(err) => {
					println!("Error: {:?}", err);
					break;
				}
			}
		}
	}
	Ok(0)
}

// Items in your inventory:
// - sand
// - fixed point
// - coin
// - spool of cat6
