use std::time::Instant;

fn solve(input: &str) -> i32 {
	let mut xs = Vec::new();
	let mut ys = Vec::new();
	let mut dxs = Vec::new();
	let mut dys = Vec::new();
	for line in input.lines() {
		let mut inp = line[10..].splitn(2, ',');
		let x : i32 = inp.next().unwrap().trim().parse().unwrap();
		let mut inp = inp.next().unwrap().split('>');
		let y : i32 = inp.next().unwrap().trim().parse().unwrap();
		let mut inp = inp.next().unwrap().split('<').skip(1);
		let mut inp = inp.next().unwrap().split(',');
		let dx : i32 = inp.next().unwrap().trim().parse().unwrap();
		let mut inp = inp.next().unwrap().split('>');
		let dy : i32 = inp.next().unwrap().trim().parse().unwrap();
		xs.push(x);
		ys.push(y);
		dxs.push(dx);
		dys.push(dy);
	}
	let maxx = xs.iter().max().unwrap();
	let minx = xs.iter().min().unwrap();
	let maxy = ys.iter().max().unwrap();
	let miny = ys.iter().min().unwrap();
	let mut cursize = maxx + maxy - minx - miny;
	let mut counter = 0;
	loop {
		counter += 1;
		for i in 0..xs.len() {
			xs[i] += dxs[i];
			ys[i] += dys[i];
		}
		let maxx = xs.iter().max().unwrap();
		let minx = xs.iter().min().unwrap();
		let maxy = ys.iter().max().unwrap();
		let miny = ys.iter().min().unwrap();
		let newsize = maxx + maxy - minx - miny;
		if cursize > newsize {
			cursize = newsize;
		} else {
			for _ in 0..1 {
				for j in 0..xs.len() {
					xs[j] -= dxs[j];
					ys[j] -= dys[j];
				}
			}
			let maxx = xs.iter().max().unwrap();
			let minx = xs.iter().min().unwrap();
			let maxy = ys.iter().max().unwrap();
			let miny = ys.iter().min().unwrap();
			let mut m = vec![vec![false; (maxy - miny + 1) as usize]; (maxx - minx + 1) as usize];
			for i in 0..xs.len() {
				m[(xs[i] - minx) as usize][(ys[i] - miny) as usize] = true;
			}
			for x in 0..m[0].len() {
				for y in 0..m.len() {
					print!("{}", if m[y][x] {'#'} else {'.'})
				}
				println!()
			}
			break;
		}
	}
	counter - 1
}

fn main() {
	let time = Instant::now();
//	let input = include_str!("../example.txt");
	let input = include_str!("../input.txt");
	println!("{}", solve(input));
	println!("{:?}", time.elapsed());
}
