use aoc_runner_derive::{aoc, aoc_generator};

enum Direction {
	Up,
	Down,
	Right,
	Left,
}

struct Segment {
	dir: Direction,
	len: u32,
}

struct Wire(Vec<Segment>);

fn parse_wire(input: &str) -> Result<Wire, crate::Error> {
	let segments: Result<Vec<Segment>, crate::Error> = input
		.split(',')
		.map(|s| -> Result<Segment, crate::Error> {
			let dir = match s.chars().next()? {
				'R' => Direction::Right,
				'L' => Direction::Left,
				'U' => Direction::Up,
				'D' => Direction::Down,
				_ => return Err(format!("Unknown direction {}", s).into()),
			};
			let len = s.split_at(1).1.parse()?;
			Ok(Segment { dir, len })
		})
		.collect();
	Ok(Wire(segments?))
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<(Wire, Wire), crate::Error> {
	let mut input = input.lines().map(parse_wire);
	let wire1 = input.next()??;
	let wire2 = input.next()??;
	Ok((wire1, wire2))
}

fn iter_wire<'a>(wire: &'a Wire) -> impl Iterator<Item = (i32, i32)> + 'a {
	gen_iter::GenIter(move || {
		let mut x = 0;
		let mut y = 0;
		for s in &wire.0 {
			for _ in 0..s.len {
				match s.dir {
					Direction::Up => {
						y -= 1;
					}
					Direction::Down => {
						y += 1;
					}
					Direction::Left => {
						x -= 1;
					}
					Direction::Right => {
						x += 1;
					}
				}
				yield (x, y);
			}
		}
	})
}

#[aoc(day3, part1, iterator)]
fn part1iter(input: &(Wire, Wire)) -> Option<i32> {
	let mut grid = std::collections::HashSet::<(i32, i32)>::new();
	for (x, y) in iter_wire(&input.0) {
		grid.insert((x, y));
	}
	let mut intersections = Vec::new();
	for (x, y) in iter_wire(&input.1) {
		if grid.contains(&(x, y)) {
			intersections.push(x.abs() + y.abs());
		}
	}
	intersections.into_iter().filter(|&d| d > 0).min()
}

#[aoc(day3, part2, iterator)]
fn part2iter(input: &(Wire, Wire)) -> Option<usize> {
	let mut grid = std::collections::HashMap::<(i32, i32), usize>::new();
	for (dist, (x, y)) in iter_wire(&input.0).enumerate() {
		grid.insert((x, y), dist);
	}
	let mut intersections = Vec::new();
	for (dist, (x, y)) in iter_wire(&input.1).enumerate() {
		if let Some(other) = grid.get(&(x, y)) {
			intersections.push(dist + other);
		}
	}
	intersections
		.into_iter()
		.filter(|&d| d > 0)
		.min()
		// enumerate() counts from 0
		.map(|d| d + 2)
}

palette!(Palette {
	Empty = [0x00, 0x00, 0x00],
	W1 = [0x00, 0x88, 0x00],
	W2 = [0x00, 0x00, 0x88],
	Center = [0xFF, 0xFF, 0xFF],
	Both = [0xFF, 0x00, 0x00],
});

#[cfg(feature = "video")]
fn video(input: &(Wire, Wire), name: &str) -> Result<(), crate::Error> {
	let mut maxx = 0;
	let mut maxy = 0;
	let mut minx = 0;
	let mut miny = 0;
	let mut update_minmax = |x, y| {
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
	};
	let mut grid1 = std::collections::HashSet::<(i32, i32)>::new();
	for (x, y) in iter_wire(&input.0) {
		update_minmax(x, y);
		grid1.insert((x, y));
	}
	let mut grid2 = std::collections::HashSet::<(i32, i32)>::new();
	for (x, y) in iter_wire(&input.1) {
		update_minmax(x, y);
		grid2.insert((x, y));
	}
	let mut video = crate::video::OptionalVideo::<Palette>::new(
		Some(name),
		(maxx - minx + 1) as u16,
		(maxy - miny + 1) as u16,
		1,
	)?;
	video.frame((miny..=maxy).into_iter().map(|y| {
		(minx..=maxx)
			.into_iter()
			.map(|x| {
				let w1 = grid1.contains(&(x, y));
				let w2 = grid2.contains(&(x, y));
				use Palette::*;
				if x == 0 && y == 0 {
					Center
				} else if w1 && w2 {
					Both
				} else if w1 {
					W1
				} else if w2 {
					W2
				} else {
					Empty
				}
			})
			.collect()
	}))?;
	Ok(())
}

#[aoc(day3, part2, video)]
#[cfg(feature = "video")]
fn big_video(input: &(Wire, Wire)) -> Result<i32, crate::Error> {
	video(input, "day3")?;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn input1() {
		let wires = match parse(
			"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
		) {
			Ok(wires) => wires,
			Err(err) => panic!(err),
		};
		assert_eq!(part1iter(&wires), Some(159));
		assert_eq!(part2iter(&wires), Some(610));
		#[cfg(feature = "video")]
		assert_eq!(video(&wires, "day3-1"), Ok(()));
	}

	#[test]
	fn input2() {
		let wires = match parse(
			"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
		) {
			Ok(wires) => wires,
			Err(err) => panic!(err),
		};
		assert_eq!(part1iter(&wires), Some(135));
		assert_eq!(part2iter(&wires), Some(410));
		#[cfg(feature = "video")]
		assert_eq!(video(&wires, "day3-2"), Ok(()));
	}
}
