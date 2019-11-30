use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
	input.lines().map(|l| l.parse()).collect()
}

palette!(Palette {
	Red = &[0xFF, 0x00, 0x00],
	Blue = &[0x00, 0x00, 0xFF],
});

#[aoc(day1, part1)]
fn part1(freqs: &[i32]) -> Result<i32, crate::Error> {
	#[cfg(feature = "video")]
	let mut video = crate::video::Video::<Palette>::new("day1part1", 1, 2, 10)?;
	#[cfg(feature = "video")]
	for _ in 0..100 {
		use Palette::*;
		video.frame(vec![vec![Blue], vec![Red]].iter())?;
		video.frame(vec![vec![Red], vec![Blue]].iter())?;
	}
	Ok(freqs.len() as i32)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "1\n3\n-1";

	#[test]
	fn part1_example() {
		let parsed = parse(INPUT).unwrap();
		assert_eq!(part1(&parsed), 33);
	}
}
