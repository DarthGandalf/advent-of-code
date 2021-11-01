use pest::Parser;

#[derive(Parser)]
#[grammar = "numbers.pest"]
struct NumbersParser;

pub fn parse<N: std::str::FromStr>(input: &str) -> Vec<N>
where
	<N as std::str::FromStr>::Err: std::fmt::Debug,
{
	let input = NumbersParser::parse(Rule::input, input)
		.unwrap()
		.next()
		.unwrap();
	input
		.into_inner()
		.map(|num| num.as_str().parse::<N>().unwrap())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(parse::<i32>(""), &[]);
		assert_eq!(parse::<i32>("r"), &[]);
		assert_eq!(parse::<i32>("1"), &[1]);
		assert_eq!(parse::<i32>("12"), &[12]);
		assert_eq!(parse::<i32>(" 12 3,4-5 -6 "), &[12, 3, 4, -5, -6]);
		assert_eq!(parse::<i32>("0 <-> 2"), [0, 2]);
	}
}
