use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<Vec<crate::intcode::Type>, std::num::ParseIntError> {
	input.trim().split(',').map(|l| l.parse()).collect()
}

fn parse_camera(program: &[crate::intcode::Type]) -> Vec<Vec<char>> {
	let (_, ri) = crossbeam::channel::unbounded();
	let (to, ro) = crossbeam::channel::unbounded();
	let (tw, _) = crossbeam::channel::unbounded();
	let (te, _) = crossbeam::channel::unbounded();
	let mut camera = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || camera.run(None));
	let grid: String = ro.into_iter().map(|x| x as u8 as char).collect();
	println!("{}", grid);
	grid.lines()
		.map(|line| line.chars().collect::<Vec<char>>())
		.filter(|line| !line.is_empty())
		.collect()
}

#[aoc(day17, part1)]
fn part1(program: &[crate::intcode::Type]) -> usize {
	let grid = parse_camera(program);
	(1..grid.len() - 1)
		.map(|y| -> usize {
			(1..grid[0].len() - 1)
				.map(|x| -> usize {
					if grid[y][x] == '#'
						&& grid[y - 1][x] == '#' && grid[y + 1][x] == '#'
						&& grid[y][x - 1] == '#' && grid[y][x + 1] == '#'
					{
						y * x
					} else {
						0
					}
				})
				.sum()
		})
		.sum()
}

#[aoc(day17, part2)]
fn part2(program: &[crate::intcode::Type]) -> anyhow::Result<crate::intcode::Type> {
	let mut grid = parse_camera(program);
	for line in &mut grid {
		line.push('.');
	}
	grid.push((0..grid[0].len()).map(|_| '.').collect());
	let mut commands: Vec<String> = vec![];
	let mut ry = 0;
	let mut rx = 0;
	for y in 0..grid.len() {
		for x in 0..grid.len() {
			if grid[y][x] == '^' {
				ry = y;
				rx = x;
			}
		}
	}
	let mut dir = '^';
	loop {
		println!("{:?} {} {} {}", commands, ry, rx, dir);
		grid[ry][rx] = dir;
		for line in &grid {
			println!("{}", line.iter().collect::<String>());
		}
		grid[ry][rx] = '.';
		if grid[ry][rx + 1] == '#' {
			commands.push(if dir == '^' { "R" } else { "L" }.into());
			dir = '>';
			let mut new_x = rx;
			while grid[ry][new_x + 1] == '#' {
				new_x += 1;
				if grid[ry + 1][new_x] == '.' {
					grid[ry][new_x] = '.';
				}
			}
			commands.push(format!("{}", new_x - rx));
			rx = new_x;
		} else if rx > 0 && grid[ry][rx - 1] == '#' {
			commands.push(if dir == '^' { "L" } else { "R" }.into());
			dir = '<';
			let mut new_x = rx;
			while new_x > 0 && grid[ry][new_x - 1] == '#' {
				new_x -= 1;
				if grid[ry + 1][new_x] == '.' {
					grid[ry][new_x] = '.';
				}
			}
			commands.push(format!("{}", rx - new_x));
			rx = new_x;
		} else if grid[ry + 1][rx] == '#' {
			commands.push(if dir == '<' { "L" } else { "R" }.into());
			dir = 'v';
			let mut new_y = ry;
			while grid[new_y + 1][rx] == '#' {
				new_y += 1;
				if grid[new_y][rx + 1] == '.' {
					grid[new_y][rx] = '.';
				}
			}
			commands.push(format!("{}", new_y - ry));
			ry = new_y;
		} else if ry > 0 && grid[ry - 1][rx] == '#' {
			commands.push(if dir == '>' { "L" } else { "R" }.into());
			dir = '^';
			let mut new_y = ry;
			while new_y > 0 && grid[new_y - 1][rx] == '#' {
				new_y -= 1;
				if grid[new_y][rx + 1] == '.' {
					grid[new_y][rx] = '.';
				}
			}
			commands.push(format!("{}", ry - new_y));
			ry = new_y;
		} else {
			println!("{:?}", commands);
			[
				"R", "10", "L", "8", "R", "10", "R", "4", "L", "6", "L", "6", "R", "10", "R", "10",
				"L", "8", "R", "10", "R", "4", "L", "6", "R", "12", "R", "12", "R", "10", "L", "6",
				"L", "6", "R", "10", "L", "6", "R", "12", "R", "12", "R", "10", "R", "10", "L",
				"8", "R", "10", "R", "4", "L", "6", "L", "6", "R", "10", "R", "10", "L", "8", "R",
				"10", "R", "4", "L", "6", "R", "12", "R", "12", "R", "10",
			];
			for line in grid {
				println!("{}", line.into_iter().collect::<String>());
			}
			break;
			//return Ok(0);
		}
	}
	println!("Running robot");
	let mut program = program.to_vec();
	program[0] = 2;
	let (ti, ri) = crossbeam::channel::unbounded();
	let (to, ro) = crossbeam::channel::unbounded();
	let (tw, _) = crossbeam::channel::unbounded();
	let (te, _) = crossbeam::channel::unbounded();
	let mut robot = crate::intcode::Computer::new(program.to_vec(), ri, tw, to, te);
	std::thread::spawn(move || robot.run(None));
	let input = "A,C,A,B,C,B,A,C,A,B\nR,10,L,8,R,10,R,4\nL,6,R,12,R,12,R,10\nL,6,L,6,R,10\nn\n";
	for c in input.chars() {
		ti.send(c as u8 as crate::intcode::Type)?;
	}
	for c in ro.into_iter() {
		if c > 200 {
			return Ok(c);
		}
		print!("{}", c as u8 as char);
	}
	Ok(0)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day17.txt")).unwrap();
		assert_eq!(part1(&input), 7280);
		assert_eq!(part2(&input).unwrap(), 1045393);
	}
}
