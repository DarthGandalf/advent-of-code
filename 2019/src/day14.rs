use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day14.pest"]
struct Day14Parser;

type Resource = u64;
#[derive(Debug)]
struct Amount {
	num: u64,
	resource: Resource,
}
impl Amount {
	fn parse(pair: pest::iterators::Pair<Rule>) -> anyhow::Result<Self> {
		let mut amount = pair.into_inner();
		let num = amount.next().none_err()?.as_str().trim().parse()?;
		let resource = hash_resource(amount.next().none_err()?.as_str().trim());
		Ok(Amount { num, resource })
	}
}
struct Recipes(Vec<(Amount, Vec<Amount>)>);

fn hash_resource(input: &str) -> u64 {
	let mut hasher = std::collections::hash_map::DefaultHasher::new();
	std::hash::Hash::hash(input, &mut hasher);
	std::hash::Hasher::finish(&hasher)
}

#[aoc_generator(day14)]
fn parse(input: &str) -> anyhow::Result<Recipes> {
	let input = Day14Parser::parse(Rule::input, input.trim())?
		.next()
		.none_err()?;
	let recipes: anyhow::Result<fnv::FnvHashMap<_, _>> = input
		.into_inner()
		.filter(|pair| pair.as_rule() == Rule::recipe)
		.map(|pair| {
			let mut recipe = pair.into_inner();
			let ingredients: anyhow::Result<Vec<Amount>> = recipe
				.next()
				.none_err()?
				.into_inner()
				.map(Amount::parse)
				.collect();
			let result = Amount::parse(recipe.next().none_err()?)?;
			Ok((result.resource, (result.num, ingredients?)))
		})
		.collect();
	let mut recipes = recipes?;
	let nodes: Vec<Resource> = recipes
		.keys()
		.cloned()
		.chain(std::iter::once(hash_resource("ORE")))
		.collect();
	Ok(Recipes(
		pathfinding::prelude::topological_sort(&nodes, |node| {
			recipes
				.get(node)
				.map(|ingredients| {
					ingredients
						.1
						.iter()
						.map(|i| i.resource)
						.collect::<Vec<Resource>>()
				})
				.unwrap_or_default()
		})
		.map_err(|_| anyhow::anyhow!("cycle"))?
		.into_iter()
		.map(|i| {
			let (num, ingredients) = recipes.remove(&i).unwrap_or_else(|| (1, vec![]));
			(Amount { num, resource: i }, ingredients)
		})
		.collect(),
	))
}

fn process_n_fuel(recipes: &Recipes, fuel: u64) -> anyhow::Result<u64> {
	let fuel_resource = hash_resource("FUEL");
	let ore_resource = hash_resource("ORE");
	let mut requirements = fnv::FnvHashMap::default();
	requirements.insert(fuel_resource, fuel);
	for (Amount { num, resource }, ingrs) in &recipes.0 {
		let requirement = requirements.remove(resource).unwrap_or_default();
		if *resource == ore_resource {
			return Ok(requirement);
		}
		// Round up
		let times = (requirement + num - 1) / num;
		for ingredient in ingrs {
			*requirements.entry(ingredient.resource.clone()).or_insert(0) += ingredient.num * times;
		}
	}
	Err(anyhow::anyhow!("unreachable"))
}

#[aoc(day14, part1)]
fn part1(recipes: &Recipes) -> anyhow::Result<u64> {
	Ok(process_n_fuel(recipes, 1)?)
}

#[aoc(day14, part2)]
fn part2(recipes: &Recipes) -> anyhow::Result<u64> {
	let mut lower = 1;
	let mut upper = 1_000_000_000;
	let mut good_attempt = lower;
	while lower < upper {
		let attempt = (upper + lower) / 2;
		if process_n_fuel(recipes, attempt)? <= 1_000_000_000_000 {
			good_attempt = attempt;
			lower = attempt + 1;
		} else {
			upper = attempt;
		}
	}
	Ok(good_attempt)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		let input = parse(
			"
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 31);
	}

	#[test]
	fn test2() {
		let input = parse(
			"
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 165);
	}

	#[test]
	fn test3() {
		let input = parse(
			"
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 13312);
		assert_eq!(part2(&input).unwrap(), 82892753);
	}

	#[test]
	fn test4() {
		let input = parse(
			"
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 180697);
		assert_eq!(part2(&input).unwrap(), 5586022);
	}

	#[test]
	fn test5() {
		let input = parse(
			"
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 2210736);
		assert_eq!(part2(&input).unwrap(), 460664);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day14.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 907302);
		assert_eq!(part2(&input).unwrap(), 1670299);
	}
}
