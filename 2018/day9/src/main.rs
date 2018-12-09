use std::time::Instant;
use std::collections::VecDeque;

fn solve(input: &str) -> usize {
	let mut input = input.split(' ');
	let players : usize = input.next().unwrap().parse().unwrap();
	let marbles : usize = input.skip(5).next().unwrap().parse().unwrap();
	let marbles = marbles * 100;
	let mut score = vec![0; players];
	let mut game = VecDeque::new();
	game.push_back(0);
	for marble in 1..=marbles {
		let player = marble % players;
		if marble % 23 == 0 {
			score[player] += marble;
			for _ in 0..7 {
				let m = game.pop_back().unwrap();
				game.push_front(m);
			}
			score[player] += game.pop_front().unwrap();
		} else {
			for _ in 0..2 {
				let m = game.pop_front().unwrap();
				game.push_back(m);
			}
			game.push_front(marble);
		}
	}
	*score.iter().max().unwrap()
}

fn main() {
	let time = Instant::now();
//	let input = include_str!("../example.txt");
	let input = include_str!("../input.txt");
	println!("{}", solve(input));
	println!("{:?}", time.elapsed());
}
