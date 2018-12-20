use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Coord {
	x: i32,
	y: i32,
}

fn _display(hor: &HashSet<Coord>, ver: &HashSet<Coord>, front: &HashSet<Coord>) {
	let minx = hor.iter().chain(ver.iter()).map(|&c| c.x).min().unwrap();
	let maxx = hor.iter().chain(ver.iter()).map(|&c| c.x).max().unwrap() + 1;
	let miny = hor.iter().chain(ver.iter()).map(|&c| c.y).min().unwrap();
	let maxy = hor.iter().chain(ver.iter()).map(|&c| c.y).max().unwrap() + 1;
	for _x in minx..=maxx {
		print!("#-");
	}
	println!("#");
	for y in miny..=maxy {
		print!("|");
		for x in minx..=maxx {
			if x == 0 && y == 0 {
				print!("0");
			} else if front.contains(&Coord { x, y }) {
				print!("x");
			} else {
				print!(" ");
			}
			if hor.contains(&Coord { x, y }) {
				print!(" ");
			} else {
				print!("|");
			}
		}
		println!();
		print!("#");
		for x in minx..=maxx {
			if ver.contains(&Coord { x, y }) {
				print!(" ");
			} else {
				print!("-");
			}
			print!("#");
		}
		println!();
	}
}

fn bfs(mut hor: HashSet<Coord>, mut ver: HashSet<Coord>, mut cb: impl FnMut(usize)) {
	let mut q = VecDeque::new();
	q.push_back((Coord { x: 0, y: 0 }, 0));
	while q.len() > 0 {
		let (xy, dist) = q.pop_front().unwrap();
		cb(dist);
		if hor.contains(&Coord { x: xy.x, y: xy.y }) {
			q.push_back((
				Coord {
					x: xy.x + 1,
					y: xy.y,
				},
				dist + 1,
			));
			hor.remove(&Coord { x: xy.x, y: xy.y });
		}
		if hor.contains(&Coord {
			x: xy.x - 1,
			y: xy.y,
		}) {
			q.push_back((
				Coord {
					x: xy.x - 1,
					y: xy.y,
				},
				dist + 1,
			));
			hor.remove(&Coord {
				x: xy.x - 1,
				y: xy.y,
			});
		}
		if ver.contains(&Coord { x: xy.x, y: xy.y }) {
			q.push_back((
				Coord {
					x: xy.x,
					y: xy.y + 1,
				},
				dist + 1,
			));
			ver.remove(&Coord { x: xy.x, y: xy.y });
		}
		if ver.contains(&Coord {
			x: xy.x,
			y: xy.y - 1,
		}) {
			q.push_back((
				Coord {
					x: xy.x,
					y: xy.y - 1,
				},
				dist + 1,
			));
			ver.remove(&Coord {
				x: xy.x,
				y: xy.y - 1,
			});
		}
	}
}

fn _solve1(input: &str) -> usize {
	let (hor, ver) = generate(&input);
	let mut result = 0;
	bfs(hor, ver, |dist| {
		if dist > result {
			result = dist;
		}
	});
	result
}

fn _solve2(input: &str) -> usize {
	let (hor, ver) = generate(&input);
	let mut result = 0;
	bfs(hor, ver, |dist| {
		if dist >= 1000 {
			result += 1;
		}
	});
	result
}

fn generate(input: &str) -> (HashSet<Coord>, HashSet<Coord>) {
	let mut ver: HashSet<Coord> = HashSet::new();
	let mut hor: HashSet<Coord> = HashSet::new();
	let start = Coord { x: 0, y: 0 };
	let text = input.lines().next().unwrap();
	let text = &text[1..text.len() - 1];
	//	let mut visited: HashSet<(Coord, usize)> = HashSet::new();
	//	visited.insert((start, 0));
	let mut front = HashSet::new();
	front.insert(start);
	let mut groups = vec![];
	for symbol in text.chars() {
		/*		println!();
		println!("Groups: {:?}", &groups);
		println!("New symbol: {}", symbol);*/
		match symbol {
			'N' => {
				let mut new = HashSet::new();
				for mut pos in front {
					pos.y -= 1;
					ver.insert(pos);
					new.insert(pos);
				}
				front = new;
			}
			'S' => {
				let mut new = HashSet::new();
				for mut pos in front {
					ver.insert(pos);
					pos.y += 1;
					new.insert(pos);
				}
				front = new;
			}
			'E' => {
				let mut new = HashSet::new();
				for mut pos in front {
					hor.insert(pos);
					pos.x += 1;
					new.insert(pos);
				}
				front = new;
			}
			'W' => {
				let mut new = HashSet::new();
				for mut pos in front {
					pos.x -= 1;
					hor.insert(pos);
					new.insert(pos);
				}
				front = new;
			}
			'(' => {
				groups.push((front.clone(), HashSet::new()));
			}
			'|' => {
				let (prevfront, endfront) = groups.pop().unwrap();
				groups.push((
					prevfront.clone(),
					endfront.union(&front).map(|&c| c).collect(),
				));
				front = prevfront;
			}
			')' => {
				let (_, f) = groups.pop().unwrap();
				front = front.union(&f).map(|&c| c).collect();
			}
			_ => {}
		}
	}
	//		_display(&hor, &ver, &front);
	(hor, ver)
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", _solve2(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let input = include_str!("../example1.txt");
		assert_eq!(_solve1(&input), 10);
	}
	#[test]
	fn test_2() {
		let input = include_str!("../example2.txt");
		assert_eq!(_solve1(&input), 18);
	}
	#[test]
	fn test_3() {
		let input = include_str!("../example3.txt");
		assert_eq!(_solve1(&input), 31);
	}
}
