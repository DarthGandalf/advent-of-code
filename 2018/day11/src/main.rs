use std::time::Instant;

fn power(x: usize, y: usize, serial: usize) -> i32 {
	let rack = x + 10;
	let p = (rack * y + serial) as i32 * rack as i32;
	(p / 100) % 10 - 5
}

fn _solve1(serial: usize) -> (usize, usize) {
	let mut max = -100;
	let mut fx = 0;
	let mut fy = 0;
	for x in 1..297 {
		for y in 1..297 {
			let mut sum = 0;
			for x in x..x+3 {
				for y in y..y+3 {
					sum += power(x, y, serial);
				}
			}
			if sum > max {
				max = sum;
				fx = x;
				fy = y;
			}
		}
	}
	(fx, fy)
}

fn solve2(serial: usize) -> (i32, usize, usize, usize) {
	let mut parts = vec![vec![0; 301]; 301];
	for x in 1..=300 {
		for y in 1..=300 {
			parts[x][y] = parts[x][y-1] + parts[x-1][y] + power(x, y, serial) - parts[x-1][y-1];
		}
	}
	let mut max = -100;
	let mut fx = 0;
	let mut fy = 0;
	let mut fs = 0;
	for s in 1..=300 {
		for x in 0..=300-s {
			for y in 0..=300-s {
				let sum = parts[x+s][y+s] + parts[x][y] - parts[x+s][y] - parts[x][y+s];
				if sum > max {
					max = sum;
					fs = s;
					fx = x;
					fy = y;
				}
			}
		}
	}
	(max, fx + 1, fy + 1, fs)
}

fn main() {
	let time = Instant::now();
	println!("{:?}", solve2(9221));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_power() {
		assert_eq!(power(3, 5, 8), 4);
		assert_eq!(power(122, 79, 57), -5);
		assert_eq!(power(217, 196, 39), 0);
		assert_eq!(power(101, 153, 71), 4);
	}

	#[test]
	fn test_1() {
		assert_eq!(_solve1(18), (33, 45));
		assert_eq!(_solve1(42), (21, 61));
	}

	#[test]
	fn test_2() {
		assert_eq!(solve2(18), (113, 90, 269, 16));
		assert_eq!(solve2(42), (119, 232, 251, 12));
	}
}
