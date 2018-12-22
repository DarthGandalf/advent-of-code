use cached::cached;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::time::Instant;

cached! {
	EROSION;
	fn erosion(x: usize, y: usize, depth: usize, tx: usize, ty: usize) -> usize = {
		if x == 0 && y == 0 || x == tx && y == ty {
			return 0;
		}
		if y == 0 {
			return (x * 16807 + depth) % 20183;
		}
		if x == 0 {
			return (y * 48271 + depth) % 20183;
		}
		(erosion(x - 1, y, depth, tx, ty) * erosion(x, y - 1, depth, tx, ty) + depth) % 20183
	}
}

#[derive(Clone, Copy)]
enum Terrain {
	Rocky,
	Wet,
	Narrow,
}

fn terrain(x: usize, y: usize, depth: usize, tx: usize, ty: usize) -> Terrain {
	match erosion(x, y, depth, tx, ty) % 3 {
		0 => Terrain::Rocky,
		1 => Terrain::Wet,
		_ => Terrain::Narrow,
	}
}

fn parse(input: &str) -> (usize, usize, usize) {
	let mut input = input.lines();
	let depth: usize = input
		.next()
		.unwrap()
		.split_whitespace()
		.nth(1)
		.unwrap()
		.parse()
		.unwrap();
	let mut input = input
		.next()
		.unwrap()
		.split_whitespace()
		.nth(1)
		.unwrap()
		.split(',');
	let tx: usize = input.next().unwrap().parse().unwrap();
	let ty: usize = input.next().unwrap().parse().unwrap();
	(depth, tx, ty)
}

fn _solve1(input: &str) -> usize {
	let (depth, tx, ty) = parse(&input);
	let mut level = 0;
	for y in 0..=ty {
		for x in 0..=tx {
			level += erosion(x, y, depth, tx, ty) % 3;
			print!(
				"{}",
				match erosion(x, y, depth, tx, ty) % 3 {
					0 => '.',
					1 => '=',
					_ => '|',
				}
			);
		}
		println!();
	}
	level
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
enum Tool {
	Neither,
	Torch,
	Climbing,
}

fn compatible(tool: Tool, terrain: Terrain) -> bool {
	match terrain {
		Terrain::Rocky => tool != Tool::Neither,
		Terrain::Wet => tool != Tool::Torch,
		Terrain::Narrow => tool != Tool::Climbing,
	}
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
struct Position {
	x: usize,
	y: usize,
	tool: Tool,
}

#[derive(Eq, PartialEq, Debug)]
struct QueueItem {
	dist: usize,
	position: Position,
}

impl std::cmp::Ord for QueueItem {
	fn cmp(&self, other: &QueueItem) -> std::cmp::Ordering {
		other
			.dist
			.cmp(&self.dist)
			.then_with(|| self.position.cmp(&other.position))
	}
}

impl std::cmp::PartialOrd for QueueItem {
	fn partial_cmp(&self, other: &QueueItem) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn _solve2(input: &str) -> usize {
	let (depth, tx, ty) = parse(&input);
	let mut heap = BinaryHeap::new();
	let start = Position {
		x: 0,
		y: 0,
		tool: Tool::Torch,
	};
	heap.push(QueueItem {
		dist: 0,
		position: start.clone(),
	});
	let mut visited = HashMap::new();
	let target = Position {
		x: tx,
		y: ty,
		tool: Tool::Torch,
	};
	loop {
		let head = heap.pop().unwrap();
		if head.position == target {
			return head.dist;
		}
		if let Some(&dist) = visited.get(&head.position) {
			if dist <= head.dist {
				continue;
			}
		}
		visited.remove(&head.position);
		visited.insert(head.position.clone(), head.dist);
		let mut try_go = |position: Position, dist| {
			if compatible(
				position.tool,
				terrain(position.x, position.y, depth, tx, ty),
			) {
				heap.push(QueueItem { position, dist });
			}
		};
		try_go(
			Position {
				x: head.position.x + 1,
				y: head.position.y,
				tool: head.position.tool,
			},
			head.dist + 1,
		);
		try_go(
			Position {
				x: head.position.x,
				y: head.position.y + 1,
				tool: head.position.tool,
			},
			head.dist + 1,
		);
		if head.position.x > 0 {
			try_go(
				Position {
					x: head.position.x - 1,
					y: head.position.y,
					tool: head.position.tool,
				},
				head.dist + 1,
			);
		}
		if head.position.y > 0 {
			try_go(
				Position {
					x: head.position.x,
					y: head.position.y - 1,
					tool: head.position.tool,
				},
				head.dist + 1,
			);
		}
		let current = terrain(head.position.x, head.position.y, depth, tx, ty);
		for &tool in &[Tool::Torch, Tool::Climbing, Tool::Neither] {
			if compatible(tool, current) {
				try_go(
					Position {
						x: head.position.x,
						y: head.position.y,
						tool,
					},
					head.dist + 7,
				);
			}
		}
	}
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
		let input = include_str!("../example.txt");
		assert_eq!(_solve1(&input), 114);
	}

	#[test]
	fn test_2() {
		let input = include_str!("../example.txt");
		assert_eq!(_solve2(&input), 44);
	}
}
