use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug,Clone,Copy)]
enum LastDirection {
	Left,
	Straight,
	Right,
}

impl LastDirection {
	fn next(self) -> LastDirection {
		match self {
			LastDirection::Left => LastDirection::Straight,
			LastDirection::Straight => LastDirection::Right,
			LastDirection::Right => LastDirection::Left,
		}
	}
}

#[derive(Debug)]
struct CurrentDirection(char);

#[derive(Debug)]
struct Cart {
	x: usize,
	y: usize,
	curdir: CurrentDirection,
	ldir: LastDirection,
}

impl Cart {
	fn step(&mut self, field: &Field) {
		let adjusted_track = match field.0[self.y][self.x] {
			'+' => {
				self.ldir = self.ldir.next();
				match (self.ldir, self.curdir.0) {
					(LastDirection::Left, '<') => '/',
					(LastDirection::Left, '>') => '/',
					(LastDirection::Left, '^') => '\\',
					(LastDirection::Left, 'v') => '\\',
					(LastDirection::Straight, ..) => 'o',
					(LastDirection::Right, '<') => '\\',
					(LastDirection::Right, '>') => '\\',
					(LastDirection::Right, '^') => '/',
					(LastDirection::Right, 'v') => '/',
					_ => 'o',
				}
			},
			'-'|'|' => 'o',
			_ => field.0[self.y][self.x],
		};
		match (adjusted_track, self.curdir.0) {
			('o', '<') => self.x -= 1,
			('o', '>') => self.x += 1,
			('o', '^') => self.y -= 1,
			('o', 'v') => self.y += 1,
			('/', '<') => {
				self.y += 1;
				self.curdir.0 = 'v';
			},
			('/', '>') => {
				self.y -= 1;
				self.curdir.0 = '^';
			},
			('/', '^') => {
				self.x += 1;
				self.curdir.0 = '>';
			},
			('/', 'v') => {
				self.x -= 1;
				self.curdir.0 = '<';
			},
			('\\', '<') => {
				self.y -= 1;
				self.curdir.0 = '^';
			},
			('\\', '>') => {
				self.y += 1;
				self.curdir.0 = 'v';
			},
			('\\', '^') => {
				self.x -= 1;
				self.curdir.0 = '<';
			},
			('\\', 'v') => {
				self.x += 1;
				self.curdir.0 = '>';
			},
			_ => {}
		}
	}
}

struct Field(Vec<Vec<char>>);

fn parse(input: &str) -> (Field, Vec<Cart>) {
	let mut field = Field(Vec::new());
	let mut carts = Vec::new();
	for (y, line) in input.lines().enumerate() {
		field.0.push(line.chars().enumerate().map(|(x, c)| match c {
			'>'|'<' => {carts.push(Cart { x, y, curdir: CurrentDirection(c), ldir: LastDirection::Right}); '-'},
			'^'|'v' => {carts.push(Cart { x, y, curdir: CurrentDirection(c), ldir: LastDirection::Right}); '|'},
			_ => c,
		}).collect());
	}
	(field, carts)
}

fn _solve1(input: &str) -> (usize, usize) {
	let (field, mut carts) = parse(&input);
	let mut positions : HashSet<_> = carts.iter().map(|c| (c.x, c.y)).collect();
	loop {
		for c in &mut carts {
			positions.remove(&(c.x, c.y));
			c.step(&field);
			if positions.contains(&(c.x, c.y)) {
				return (c.x, c.y);
			}
			positions.insert((c.x, c.y));
		}
		carts.sort_by_key(|c| (c.y, c.x));
/*		println!("{:?}", &positions);
		let mut output = field.0.clone();
		for c in &carts {
			output[c.y][c.x] = c.curdir.0;
		}
		for l in output {
			let s : String = l.iter().collect();
			println!("{}", s);
		}*/
	}
}

fn _solve2(input: &str) -> (usize, usize) {
	let (field, mut carts) = parse(&input);
	loop {
		let mut positions : HashMap<_, _> = carts.iter().enumerate().map(|(i, c)| ((c.x, c.y), i)).collect();
		let mut enabled_carts = vec![true; carts.len()];
		for (i, c) in carts.iter_mut().enumerate() {
			if !enabled_carts[i] {
				continue;
			}
			positions.remove(&(c.x, c.y));
			c.step(&field);
			if let Some(&other) = positions.get(&(c.x, c.y)) {
				enabled_carts[i] = false;
				enabled_carts[other] = false;
				continue;
			}
			positions.insert((c.x, c.y), i);
		}
		let mut remaining_carts = Vec::new();
		for (i, c) in carts.into_iter().enumerate() {
			if enabled_carts[i] {
				remaining_carts.push(c);
			}
		}
		carts = remaining_carts;
		if carts.len() == 1 {
			return (carts[0].x, carts[0].y);
		}
		carts.sort_by_key(|c| (c.y, c.x));
	}
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{:?}", _solve2(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let input = include_str!("../example.txt");
		assert_eq!(_solve1(input), (7, 3));
	}

	#[test]
	fn test_2() {
		let input = include_str!("../example2.txt");
		assert_eq!(_solve2(input), (6, 4));
	}
}
