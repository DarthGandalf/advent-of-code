use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Map {
	wall: Vec<Vec<bool>>,
	start: (usize, usize),
	keys: fnv::FnvHashMap<(usize, usize), char>,
	doors: fnv::FnvHashMap<(usize, usize), char>,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> anyhow::Result<Map> {
	let mut keys = fnv::FnvHashMap::default();
	let mut doors = fnv::FnvHashMap::default();
	let mut start = (0, 0);
	let mut error = None;
	let wall = input
		.trim()
		.lines()
		.enumerate()
		.map(|(y, line)| {
			line.chars()
				.enumerate()
				.map(|(x, c)| {
					if c == '#' {
						true
					} else {
						match c {
							'@' => start = (y, x),
							'.' => {}
							'a'..='z' => {
								keys.insert((y, x), c);
							}
							'A'..='Z' => {
								doors.insert((y, x), c.to_ascii_lowercase());
							}
							_ if error.is_none() => {
								error = Some(anyhow::anyhow!(
									"unknown input {} at row {} col {}",
									c,
									y,
									x
								))
							}
							_ => {}
						}
						false
					}
				})
				.collect()
		})
		.collect();
	if let Some(error) = error {
		Err(error)
	} else {
		Ok(Map {
			wall,
			keys,
			doors,
			start,
		})
	}
}

#[aoc(day18, part1)]
fn part1(map: &Map) -> Option<usize> {
	#[derive(Eq, Hash, Clone, PartialEq, Debug)]
	struct Node {
		pos: (usize, usize),
		need_keys: std::collections::BTreeSet<char>,
	}
	let path = pathfinding::directed::dijkstra::dijkstra(
		&Node {
			pos: map.start,
			need_keys: map.keys.values().cloned().collect(),
		},
		|n: &Node| {
			let mut neigh = vec![];
			let mut try_add = |pos: (usize, usize)| {
				if map.wall[pos.0][pos.1] {
					return;
				}
				if let Some(door) = map.doors.get(&pos) {
					if n.need_keys.contains(&door) {
						return;
					}
				}
				let mut need_keys = n.need_keys.clone();
				if let Some(key) = map.keys.get(&pos) {
					need_keys.remove(&key);
				}
				neigh.push((Node { pos, need_keys }, 1));
			};
			try_add((n.pos.0, n.pos.1 + 1));
			try_add((n.pos.0, n.pos.1 - 1));
			try_add((n.pos.0 + 1, n.pos.1));
			try_add((n.pos.0 - 1, n.pos.1));
			neigh
		},
		|n: &Node| n.need_keys.is_empty(),
	)?;
	//println!("{:#?}", path);
	Some(path.1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		let input = parse(
			"
#########
#b.A.@.a#
#########",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 8);
	}

	#[test]
	fn test2() {
		let input = parse(
			"
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 86);
	}

	#[test]
	fn test3() {
		let input = parse(
			"
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 132);
	}

	#[test]
	fn test4() {
		let input = parse(
			"
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 136);
	}

	#[test]
	fn test5() {
		let input = parse(
			"
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 81);
	}

	#[test]
	fn answers() {
		let input = include_str!("../input/2019/day18.txt").trim();
		assert_eq!(part1(&input).unwrap(), 4270);
		//assert_eq!(part2(&input).unwrap(), "32749588");
	}
}
