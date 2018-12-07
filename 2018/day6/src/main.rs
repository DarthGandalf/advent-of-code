use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashMap;

fn _solve1(input: &str) -> i32 {
	let points : Vec<_> = input.lines().map(|line| {
		let mut t = line.split(',');
		let x : usize = t.next().unwrap().trim().parse().unwrap();
		let y : usize = t.next().unwrap().trim().parse().unwrap();
		(x, y)
	}).collect();
	let (minx, _) = points.iter().min_by_key(|(x, _)| x).unwrap();
	let (maxx, _) = points.iter().max_by_key(|(x, _)| x).unwrap();
	let (_, miny) = points.iter().min_by_key(|(_, y)| y).unwrap();
	let (_, maxy) = points.iter().max_by_key(|(_, y)| y).unwrap();
	let points : Vec<_> = points.iter().map(|(x, y)| ((x - minx) as usize, (y - miny) as usize)).collect();
	let maxx = maxx - minx;
	let maxy = maxy - miny;
	let mut field = vec![vec![-1; maxy + 1]; maxx + 1];
	let mut queue = VecDeque::new();
	let mut areas = vec![0; points.len()];
	for (i, p) in points.iter().enumerate() {
		queue.push_back((i, p.0, p.1));
	}
	while queue.len() > 0 {
		let mut visiting = HashMap::new();
		for (p, x, y) in queue {
			if field[x][y] != -1 {
				continue;
			}
			let mut good = true;
			if let Some(vp) = visiting.get(&(x, y)) {
				if &p != vp {
					field[x][y] = -2;
					good = false;
				}
			}
			if good {
				visiting.insert((x, y), p);
			}
		}
		queue = VecDeque::new();
		for ((x, y), p) in visiting {
			if field[x][y] != -2 {
				field[x][y] = p as i32;
				areas[p] += 1;
				if x > 0 {
					queue.push_back((p, x-1, y));
				}
				if x < maxx {
					queue.push_back((p, x+1, y));
				}
				if y > 0 {
					queue.push_back((p, x, y-1));
				}
				if y < maxy {
					queue.push_back((p, x, y+1));
				}
			}
		}
	}
	let mut areas : HashMap<_, _> = areas.iter().enumerate().collect();
	for x in 0..=maxx {
		if field[x][0] >= 0 {
			areas.remove(&(field[x][0] as usize));
		}
		if field[x][maxy] >= 0 {
			areas.remove(&(field[x][maxy] as usize));
		}
	}
	for y in 1..maxy {
		if field[0][y] >= 0 {
			areas.remove(&(field[0][y] as usize));
		}
		if field[maxx][y] >= 0 {
			areas.remove(&(field[maxx][y] as usize));
		}
	}
	**areas.iter().max_by_key(|(_k, v)| *v).unwrap().1
}

fn solve2(input: &str) -> i32 {
	let points : Vec<_> = input.lines().map(|line| {
		let mut t = line.split(',');
		let x : usize = t.next().unwrap().trim().parse().unwrap();
		let y : usize = t.next().unwrap().trim().parse().unwrap();
		(x, y)
	}).collect();
	let (minx, _) = points.iter().min_by_key(|(x, _)| x).unwrap();
	let (maxx, _) = points.iter().max_by_key(|(x, _)| x).unwrap();
	let (_, miny) = points.iter().min_by_key(|(_, y)| y).unwrap();
	let (_, maxy) = points.iter().max_by_key(|(_, y)| y).unwrap();
	let mut count = 0;
	for x in *minx..=*maxx {
		for y in *miny..=*maxy {
			let sx : i32 = points.iter().map(|(px, _)| (*px as i32 - x as i32).abs()).sum();
			let sy : i32 = points.iter().map(|(_, py)| (*py as i32 - y as i32).abs()).sum();
			if sx + sy < 10000 {
				count += 1
			}
		}
	}
	count
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", solve2(input));
	println!("{:?}", time.elapsed());
}
