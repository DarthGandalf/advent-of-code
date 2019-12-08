use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day8.pest"]
struct Day8Parser;

struct Row(Vec<u8>);
struct Layer(Vec<Row>);
struct Image(Vec<Layer>);

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Image, crate::Error> {
	let input = Day8Parser::parse(Rule::input, input.trim())?.next()?;
	let image: Result<Vec<Layer>, crate::Error> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::layer)
		.map(|pair| -> Result<Layer, crate::Error> {
			let layer: Result<Vec<Row>, crate::Error> = pair
				.into_inner()
				.map(|pa| {
					let row: Result<Vec<u8>, crate::Error> = pa
						.into_inner()
						.map(|p| -> Result<u8, crate::Error> { Ok(p.as_str().parse()?) })
						.collect();
					Ok(Row(row?))
				})
				.collect();
			Ok(Layer(layer?))
		})
		.collect();
	Ok(Image(image?))
}

fn digits_in_layer(layer: &Layer, digit: u8) -> usize {
	layer
		.0
		.iter()
		.map(|row| row.0.iter().filter(|&&pixel| pixel == digit).count())
		.sum()
}

#[aoc(day8, part1)]
fn part1(input: &Image) -> usize {
	let idx = input
		.0
		.iter()
		.enumerate()
		.map(|(i, layer)| (i, digits_in_layer(layer, 0)))
		.min_by_key(|&(_, num)| num)
		.unwrap_or_default()
		.0;
	let layer = &input.0[idx];
	digits_in_layer(&layer, 1) * digits_in_layer(&layer, 2)
}

#[aoc(day8, part2)]
fn part2(input: &Image) -> String {
	let mut image = vec![vec![2; 25]; 6];
	for layer in &input.0 {
		for (y, row) in layer.0.iter().enumerate() {
			for (x, pixel) in row.0.iter().enumerate() {
				if image[y][x] == 2 {
					image[y][x] = *pixel;
				}
			}
		}
	}
	itertools::join(
		image.into_iter().map(|row| {
			std::iter::once('\n')
				.chain(row.iter().map(|p| match p {
					0 => ' ',
					_ => 'X',
				}))
				.collect::<String>()
		}),
		"",
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day8.txt")).unwrap();
		assert_eq!(part1(&input), 1703);
		assert_eq!(
			part2(&input),
			"
X  X  XX   XX  XXXX XXXX 
X  X X  X X  X X    X    
XXXX X    X    XXX  XXX  
X  X X    X XX X    X    
X  X X  X X  X X    X    
X  X  XX   XXX X    XXXX "
		);
	}
}
