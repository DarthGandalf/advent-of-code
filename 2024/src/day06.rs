use aoc_runner_derive::aoc;

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
	x: i16,
	y: i16,
}

type Map = fnv::FnvHashSet<Coord>;

struct Input {
	walls: Map,
	her: Coord,
	size: Coord,
}

fn parse(input: &str) -> Input {
	let mut walls = Map::default();
	let mut her = Coord::default();
	let mut size = Coord::default();
	for (y, l) in input.lines().enumerate() {
		let y = y as i16;
		for (x, c) in l.chars().enumerate() {
			let x = x as i16;
			match c {
				'#' => {
					walls.insert(Coord { x, y });
				}
				'^' => her = Coord { x, y },
				_ => {}
			}
			size = Coord { x, y };
		}
	}
	Input { walls, her, size }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
	let Input { walls, her, size } = parse(input);
	let mut her = her;
	let mut was = Map::default();
	let mut dx: i16 = 0;
	let mut dy: i16 = -1;
	while her.x >= 0 && her.y >= 0 && her.x <= size.x && her.y <= size.y {
		was.insert(her);
		let next = Coord {
			x: her.x + dx,
			y: her.y + dy,
		};
		if walls.contains(&next) {
			let d = dx;
			dx = -dy;
			dy = d;
		} else {
			her = next;
		}
	}
	was.len()
}

fn attempt(input: &Input, newwall: Coord) -> bool {
	if newwall == input.her
		|| newwall.x < 0
		|| newwall.y < 0
		|| newwall.x > input.size.x
		|| newwall.y > input.size.y
	{
		return false;
	}
	//println!("Attempt {:?}", newwall);
	let mut her = input.her;
	let mut dx: i16 = 0;
	let mut dy: i16 = -1;
	let mut was = fnv::FnvHashSet::<(Coord, i16, i16)>::default();
	while her.x >= 0 && her.y >= 0 && her.x <= input.size.x && her.y <= input.size.y {
		if was.contains(&(her, dx, dy)) {
			/*		println!("AAAAAAAAAA {:?}", her);
			for y in 0..=input.size.y {
				for x in 0..=input.size.x {
					let here = Coord{x,y};
					if here == newwall {
						print!("O");
					} else if here == her {
						match (dx, dy) {
							(0, 1) => print!("v"),
							(0, -1)=> print!("^"),
							(1, 0)=>print!(">"),
							(-1, 0)=>print!("<"),
							_=>unreachable!(),
						}
					} else if input.walls.contains(&here) {
						print!("#");
					} else {
						print!(".");
					}
				}
				println!();
			}*/
			return true;
		}
		was.insert((her, dx, dy));
		let next = Coord {
			x: her.x + dx,
			y: her.y + dy,
		};
		if input.walls.contains(&next) || next == newwall {
			let d = dx;
			dx = -dy;
			dy = d;
		} else {
			her = next;
		}
	}
	false
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
	let input = parse(input);
	let mut her = input.her;
	let mut dx: i16 = 0;
	let mut dy: i16 = -1;
	let mut result = 0;
	let mut attempted = Map::default();
	while her.x >= 0 && her.y >= 0 && her.x <= input.size.x && her.y <= input.size.y {
		let next = Coord {
			x: her.x + dx,
			y: her.y + dy,
		};
		if input.walls.contains(&next) {
			let d = dx;
			dx = -dy;
			dy = d;
		} else {
			if !attempted.contains(&next) {
				if attempt(&input, next) {
					result += 1;
				}
				attempted.insert(next);
			}
			her = next;
		}
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 41);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 6);
	}
}
