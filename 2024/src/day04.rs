use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
	let m: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	let mut num = 0;
	for y in 0..m.len() {
		for x in 0..m[0].len() - 3 {
			if m[y][x] == 'X' && m[y][x + 1] == 'M' && m[y][x + 2] == 'A' && m[y][x + 3] == 'S' {
				num += 1;
			}
			if m[y][x] == 'S' && m[y][x + 1] == 'A' && m[y][x + 2] == 'M' && m[y][x + 3] == 'X' {
				num += 1;
			}
		}
	}
	for y in 0..m.len() - 3 {
		for x in 0..m[0].len() {
			if m[y][x] == 'X' && m[y + 1][x] == 'M' && m[y + 2][x] == 'A' && m[y + 3][x] == 'S' {
				num += 1;
			}
			if m[y][x] == 'S' && m[y + 1][x] == 'A' && m[y + 2][x] == 'M' && m[y + 3][x] == 'X' {
				num += 1;
			}
		}
		for x in 0..m[0].len() - 3 {
			if m[y][x] == 'X'
				&& m[y + 1][x + 1] == 'M'
				&& m[y + 2][x + 2] == 'A'
				&& m[y + 3][x + 3] == 'S'
			{
				num += 1;
			}
			if m[y][x] == 'S'
				&& m[y + 1][x + 1] == 'A'
				&& m[y + 2][x + 2] == 'M'
				&& m[y + 3][x + 3] == 'X'
			{
				num += 1;
			}
			if m[y][x + 3] == 'X'
				&& m[y + 1][x + 2] == 'M'
				&& m[y + 2][x + 1] == 'A'
				&& m[y + 3][x] == 'S'
			{
				num += 1;
			}
			if m[y][x + 3] == 'S'
				&& m[y + 1][x + 2] == 'A'
				&& m[y + 2][x + 1] == 'M'
				&& m[y + 3][x] == 'X'
			{
				num += 1;
			}
		}
	}
	num
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
	let m: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	let mut num = 0;
	for y in 0..m.len() - 2 {
		for x in 0..m[0].len() - 2 {
			if m[y + 1][x + 1] == 'A'
				&& (m[y][x] == 'M' && m[y + 2][x + 2] == 'S'
					|| m[y][x] == 'S' && m[y + 2][x + 2] == 'M')
				&& (m[y][x + 2] == 'M' && m[y + 2][x] == 'S'
					|| m[y][x + 2] == 'S' && m[y + 2][x] == 'M')
			{
				num += 1;
			}
		}
	}
	num
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

	#[test]
	fn test1() {
		assert_eq!(part1(INPUT), 18);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(INPUT), 9);
	}
}
