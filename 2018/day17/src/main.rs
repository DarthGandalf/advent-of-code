use std::time::Instant;

// I can't easily return impl Iterator<Item=...>
fn parse_line(l: &str, mut cb: impl FnMut((usize, usize))) {
	let mut split = l.split(", ");
	let a = split.next().unwrap();
	let b = split.next().unwrap();
	let x1 = a.chars().next() == Some('x');
	let a = a.split_at(2).1.parse().unwrap();
	let b = b.split_at(2).1;
	let mut bsplit = b.split("..");
	let b1 = bsplit.next().unwrap().parse().unwrap();
	if let Some(b2) = bsplit.next() {
		let b2 = b2.parse().unwrap();
		if x1 {
			for y in b1..=b2 {
				cb((a, y));
			}
		} else {
			for x in b1..=b2 {
				cb((x, a));
			}
		}
		return;
	}
	panic!("Expected .. in {}", l);
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, usize) {
	let mut minx = 0;
	let mut maxx = 0;
	let mut miny = 0;
	let mut maxy = 0;
	for line in input.lines() {
		parse_line(line, |(x, y)| {
			minx = x;
			maxx = x;
			miny = y;
			maxy = y;
		});
	}
	for line in input.lines() {
		parse_line(line, |(x, y)| {
			if x < minx {
				minx = x;
			}
			if x > maxx {
				maxx = x;
			}
			if y < miny {
				miny = y;
			}
			if y > maxy {
				maxy = y;
			}
		});
	}
	let offset = minx - 1;
	let mut grid = vec![vec!['.'; maxx - minx + 3]; maxy - miny + 2];
	for line in input.lines() {
		parse_line(line, |(x, y)| {
			grid[y - miny + 1][x - offset] = '#';
		});
	}
	(grid, offset)
}

fn _solve(input: &str) -> usize {
	let (mut grid, offset) = parse_grid(&input);
	grid[0][500 - offset] = '+';
	let mut sources = vec![(500 - offset, 0)];
	let mut added_more = true;
	while added_more {
		added_more = false;
		let mut new_sources = Vec::new();
		for source in sources {
			if grid[source.1][source.0] == '~' {
				continue;
			}
			let x = source.0;
			let mut y = source.1 + 1;
			while y < grid.len() && (grid[y][x] == '.' || grid[y][x] == '|') {
				grid[y][x] = '|';
				y += 1;
			}
			if y < grid.len() {
				y -= 1;
				let mut x1 = x;
				while grid[y][x1] != '#' && (grid[y + 1][x1] == '#' || grid[y + 1][x1] == '~') {
					x1 -= 1;
				}
				let mut x2 = x;
				while grid[y][x2] != '#' && (grid[y + 1][x2] == '#' || grid[y + 1][x2] == '~') {
					x2 += 1;
				}
				if grid[y][x1] == '#' && grid[y][x2] == '#' {
					for x in x1 + 1..=x2 - 1 {
						grid[y][x] = '~';
						added_more = true;
					}
				} else {
					for x in x1 + 1..=x2 - 1 {
						grid[y][x] = '|';
					}
					if grid[y][x1] != '#' && grid[y + 1][x1] == '.' {
						grid[y][x1] = '|';
						new_sources.push((x1, y));
						added_more = true;
					}
					if grid[y][x2] != '#' && grid[y + 1][x2] == '.' {
						grid[y][x2] = '|';
						new_sources.push((x2, y));
						added_more = true;
					}
				}
			}
			new_sources.push(source);
		}
		sources = new_sources.into_iter().rev().collect();
		/*		for row in grid.iter() {
			println!("{}", row.iter().collect::<String>());
		}
		println!("{:?}", &sources);*/
	}
	grid.iter()
		.enumerate()
		.map(|(y, row)| {
			if y == 0 {
				0
			} else {
				row.iter().filter(|&&c| /*c == '|' ||*/ c == '~').count()
			}
		})
		.sum()
}

fn main() {
	let time = Instant::now();
	//let input = include_str!("../example.txt");
	let input = include_str!("../input.txt");
	println!("{}", _solve(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	fn parse_line_as_vec(l: &str) -> Vec<(usize, usize)> {
		let mut result = Vec::new();
		parse_line(l, |(x, y)| result.push((x, y)));
		result
	}

	#[test]
	fn test_parse() {
		assert_eq!(
			parse_line_as_vec("x=495, y=2..7"),
			vec![(495, 2), (495, 3), (495, 4), (495, 5), (495, 6), (495, 7)]
		);
		assert_eq!(
			parse_line_as_vec("y=13, x=498..504"),
			vec![
				(498, 13),
				(499, 13),
				(500, 13),
				(501, 13),
				(502, 13),
				(503, 13),
				(504, 13)
			]
		);
	}
}
