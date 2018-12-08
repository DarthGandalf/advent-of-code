use std::time::Instant;

/*fn sum(mut input: &mut Iterator<Item=i32>) -> i32 {
	let mut result = 0;
	let children = input.next().unwrap();
	let meta = input.next().unwrap();
	for _ in 0..children {
		result += sum(&mut input);
	}
	for _ in 0..meta {
		result += input.next().unwrap();
	}
	result
}*/

fn sum2(mut input: &mut Iterator<Item=usize>) -> usize {
	let children = input.next().unwrap();
	let meta = input.next().unwrap();
	let children : Vec<_> = (0..children).map(|_| sum2(&mut input)).collect();
	if children.is_empty() {
		(0..meta).map(|_| input.next().unwrap()).sum()
	} else {
		(0..meta).map(|_| children.get(input.next().unwrap() - 1).unwrap_or(&0)).sum()
	}
}

fn solve(input: &str) -> usize {
	let mut input = input.split(' ').map(|s| s.trim().parse().unwrap());
	sum2(&mut input)
}

fn main() {
	let time = Instant::now();
//	let input = include_str!("../example.txt");
	let input = include_str!("../input.txt");
	println!("{}", solve(input));
	println!("{:?}", time.elapsed());
}
