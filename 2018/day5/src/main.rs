use std::time::Instant;

fn annihilate(input: &str, next: &mut Vec<i32>, size: &mut i32) {
	let mut prev : Vec<i32> = (-1..*size-1).collect();
	let mut it = 0;
	while next[it] != -1 {
		let itn = next[it] as usize;
		let curch = input[it..].chars().next().unwrap();
		let nextch = input[itn..].chars().next().unwrap();
		if curch != nextch && (curch.to_ascii_uppercase() == nextch || curch.to_ascii_lowercase() == nextch) {
			if prev[it] != -1 {
				next[prev[it] as usize] = next[itn];
			}
			if next[itn] != -1 {
				prev[next[itn] as usize] = prev[it];
			}
			*size -= 2;
			it = prev[it] as usize;
		} else {
			it = itn;
		}
	}
}

fn compact(input: &str, next: &Vec<i32>) -> String {
	let mut compacted = String::new();
	let mut it = 0 as i32;
	loop {
		compacted.push(input[it as usize..].chars().next().unwrap());
		it = next[it as usize] as i32;
		if it == -1 {
			return compacted;
		}
	}
}

fn solve(input: &str) -> (i32, i32) {
	let input = format!("!{}!", input.trim_end());
	let mut size = input.chars().count() as i32;
	let mut next : Vec<_> = (1..size+1).collect();
	next[size as usize - 1] = -1;
	annihilate(&input, &mut next, &mut size);
	let compacted = compact(&input, &next);
	let mut minsize2 = size + 100;
	for ch in (b'a'..=b'z').map(|c| c as char) {
		let newinput : String = compacted.chars().filter(|c| c.to_ascii_lowercase() != ch).collect();
		let mut size2 = newinput.chars().count() as i32;
		let mut next2 : Vec<_> = (1..size2+1).collect();
		next2[size2 as usize - 1] = -1;
		annihilate(&newinput, &mut next2, &mut size2);
		if size2 < minsize2 {
			minsize2 = size2;
		}
	}
	(size - 2, minsize2 - 2)
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{:?}", solve(input));
	println!("{:?}", time.elapsed());
}
