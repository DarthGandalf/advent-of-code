use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
struct Group {
	units: i32,
	hp: i32,
	dmg: i32,
	dmgtype: String,
	initiative: i32,
	weak: HashSet<String>,
	immune: HashSet<String>,
}

impl Group {
	fn parse(input: &str) -> Option<Self> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points (?:\((.*?)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)$").unwrap();
		}
		let cap = RE.captures(input)?;
		let mut weak = HashSet::new();
		let mut immune = HashSet::new();
		if let Some(m) = cap.get(3) {
			for to in m.as_str().split("; ") {
				let (set, string) = if to.chars().next().unwrap() == 'w' {
					(&mut weak, &to[8..])
				} else {
					(&mut immune, &to[10..])
				};
				*set = string.split(", ").map(|s| s.to_owned()).collect();
			}
		}
		Some(Self {
			units: cap[1].parse().unwrap(),
			hp: cap[2].parse().unwrap(),
			dmg: cap[4].parse().unwrap(),
			dmgtype: cap[5].to_owned(),
			initiative: cap[6].parse().unwrap(),
			weak,
			immune,
		})
	}

	fn effective_power(&self) -> i32 {
		self.units * self.dmg
	}

	fn take_damage(&mut self, dmg: i32) {
/*		print!(
			"taking damage: initiative={} dmg={} hp={} units={}",
			self.initiative, dmg, self.hp, self.units
		);*/
		self.units -= dmg / self.hp;
//		println!(" result: {}", self.units);
		if self.units < 0 {
			self.units = 0;
		}
	}
}

fn parse(input: &str) -> (Vec<Group>, Vec<Group>) {
	let mut infection = Vec::new();
	let mut immunity = Vec::new();
	let mut found_infection = false;
	for line in input.lines().skip(1) {
		if let Some(group) = Group::parse(line) {
			if found_infection {
				&mut infection
			} else {
				&mut immunity
			}
			.push(group);
		} else {
			found_infection = true;
		}
	}
	(immunity, infection)
}

fn calc_damage(attacker: &Group, victim: &Group) -> i32 {
	if victim.immune.contains(&attacker.dmgtype) {
		return 0;
	}
	let dmg = attacker.effective_power();
	if victim.weak.contains(&attacker.dmgtype) {
		return dmg * 2;
	}
	dmg
}

fn find_victims(attackers: &[Group], defenders: &[Group]) -> Vec<Option<usize>> {
	let mut result = vec![None; attackers.len()];
	let mut attackers_choice: Vec<usize> = (0..attackers.len()).collect();
	let mut defenders_choice: HashSet<usize> = (0..defenders.len()).collect();
	attackers_choice.sort_by_key(|&a| (-attackers[a].effective_power(), -attackers[a].initiative));
	for a in attackers_choice {
		if let Some((_, _, _, d)) = defenders_choice
			.iter()
			.map(|&d| {
				(
					calc_damage(&attackers[a], &defenders[d]),
					defenders[d].effective_power(),
					defenders[d].initiative,
					d,
				)
			})
			.filter(|&(dmg, _, _, _)| dmg > 0)
			.max()
		{
			defenders_choice.remove(&d);
			result[a] = Some(d);
		}
	}
	result
}

fn fight(immunity: &mut Vec<Group>, infection: &mut Vec<Group>) {
	let immunity_attacks = find_victims(&immunity, &infection);
	let infection_attacks = find_victims(&infection, &immunity);
/*			println!(
		"------------------\nimmunity: {:?} attacks: {:?}\ninfection: {:?} attacks: {:?}",
		&immunity, &immunity_attacks, &infection, &infection_attacks
	);*/

	let mut groups: Vec<_> = immunity_attacks
		.iter()
		.enumerate()
		.map(|(i, v)| (-immunity[i].initiative, i, v, true))
		.chain(
			infection_attacks
				.iter()
				.enumerate()
				.map(|(i, v)| (-infection[i].initiative, i, v, false)),
		)
		.collect();
	groups.sort();
	for (_, attacker, victim, immunity_team) in groups {
		/*			println!(
			"attack: {} {} -> {:?}",
			if immunity_team {
				"immunity"
			} else {
				"infection"
			},
			attacker,
			victim
		);*/
		if let Some(victim) = victim {
			if immunity_team {
				let attacker = &immunity[attacker];
				let victim = &mut infection[*victim];
				let dmg = calc_damage(&attacker, &victim);
				victim.take_damage(dmg);
			} else {
				let attacker = &infection[attacker];
				let victim = &mut immunity[*victim];
				let dmg = calc_damage(&attacker, &victim);
				victim.take_damage(dmg);
			}
		}
	}

	immunity.retain(|g| g.units > 0);
	infection.retain(|g| g.units > 0);
}

fn _solve1(input: &str) -> i32 {
	let (mut immunity, mut infection) = parse(input);
	while immunity.len() > 0 && infection.len() > 0 {
		fight(&mut immunity, &mut infection);
	}
	immunity
		.into_iter()
		.chain(infection.into_iter())
		.map(|g| g.units)
		.sum()
}

fn boost(army: Vec<Group>, by: i32) -> Vec<Group> {
	army.into_iter()
		.map(|mut g| {
			g.dmg += by;
			g
		})
		.collect()
}

fn _solve2(input: &str) -> i32 {
	let (imm, inf) = parse(input);
	let mut improvement = 0;
	loop {
//		println!("testing {}", improvement);
		let mut immunity = boost(imm.clone(), improvement);
		let mut infection = inf.clone();
		while immunity.len() > 0 && infection.len() > 0 {
//			println!("fight {:?} {:?}", &immunity, &infection);
			let prev: i32 = immunity.iter().chain(infection.iter()).map(|g| g.units).sum();
			fight(&mut immunity, &mut infection);
			let next: i32 = immunity.iter().chain(infection.iter()).map(|g| g.units).sum();
			if prev == next {
				break;
			}
		}
		if infection.into_iter().map(|g| g.units).sum::<i32>() == 0 {
			return immunity.into_iter().map(|g| g.units).sum();
		}
		improvement += 1;
	}
}

fn main() {
	let time = Instant::now();
	let input = include_str!("../input.txt");
	println!("{}", _solve2(input));
	println!("{:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_group() {
		assert_eq!(Group::parse("4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"), Some(Group {units: 4485, hp: 2961, dmg: 12, dmgtype: "slashing".to_owned(), initiative:4, weak: ["fire".to_owned(), "cold".to_owned()].iter().cloned().collect(), immune: ["radiation".to_owned()].iter().cloned().collect()}));
	}

	#[test]
	fn test_parse() {
		let input = include_str!("../example.txt");
		let (immunity, infection) = parse(input);
		assert_eq!(immunity.len(), 2);
		assert_eq!(immunity[0].units, 17);
		assert_eq!(immunity[1].units, 989);
		assert_eq!(infection.len(), 2);
		assert_eq!(infection[0].units, 801);
		assert_eq!(infection[1].units, 4485);
	}

	#[test]
	fn test_1() {
		let input = include_str!("../example.txt");
		assert_eq!(_solve1(&input), 5216);
	}

	#[test]
	fn test_2() {
		let input = include_str!("../example.txt");
		assert_eq!(_solve2(&input), 51);
	}
}
