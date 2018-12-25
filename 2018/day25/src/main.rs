use disjoint_sets::UnionFind;
use std::collections::HashSet;
use std::time::Instant;

struct Point(i32, i32, i32, i32);

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Point> + 'a {
	input.lines().map(|line| {
		let mut numbers = line.split(',');
		Point(
			numbers.next().unwrap().parse().unwrap(),
			numbers.next().unwrap().parse().unwrap(),
			numbers.next().unwrap().parse().unwrap(),
			numbers.next().unwrap().parse().unwrap(),
		)
	})
}

fn manh(a: &Point, b: &Point) -> i32 {
	(a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn _solve1(input: &str) -> usize {
	let points: Vec<Point> = parse(input).collect();
	let mut dis = UnionFind::new(points.len());
	for i in 0..points.len() {
		for j in 0..i {
			if manh(&points[i], &points[j]) <= 3 {
				dis.union(i, j);
			}
		}
	}
	let consellations: HashSet<_> = dis.to_vec().into_iter().collect();
	consellations.len()
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", _solve1(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let input = include_str!("../example.txt");
		assert_eq!(_solve1(&input), 2);
	}

	#[test]
	fn test_2() {
		let input = include_str!("../example2.txt");
		assert_eq!(_solve1(&input), 4);
	}

	#[test]
	fn test_3() {
		let input = include_str!("../example3.txt");
		assert_eq!(_solve1(&input), 3);
	}

	#[test]
	fn test_4() {
		let input = include_str!("../example4.txt");
		assert_eq!(_solve1(&input), 8);
	}
}
