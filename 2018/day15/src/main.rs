use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
struct Race(char);

#[derive(Clone)]
struct Unit {
	x: usize,
	y: usize,
	hp: i16,
	race: Race,
	power: i16,
}

impl Unit {
	fn walk(&mut self, field: &Field, units_map: &BTreeMap<(usize, usize), usize>, units: &[Unit]) {
		//println!("walking");
		let can_walk =
			|y: usize, x: usize| field.0[y][x] == Square::Empty && !units_map.contains_key(&(y, x));
		let enemies_adjacent: HashSet<_> = units_map
			.iter()
			.filter(|(_, &e)| units[e].race != self.race && units[e].hp > 0)
			.flat_map(|(&(y, x), _)| vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)])
			.filter(|&(y, x)| can_walk(y, x))
			.collect();
		if enemies_adjacent.is_empty() {
			return;
		}
		if enemies_adjacent.contains(&(self.y, self.x)) {
			return;
		}
		let mut prev = vec![vec![(0, 0); field.0[0].len()]; field.0.len()];
		prev[self.y][self.x] = (self.y, self.x);
		let mut queue = VecDeque::new();
		queue.push_back((self.y, self.x));
		let mut targets_found = Vec::new();
		while targets_found.len() == 0 && queue.len() > 0 {
			let mut next_queue = VecDeque::new();
			for (y, x) in queue {
				if enemies_adjacent.contains(&(y, x)) {
					targets_found.push((y, x));
				}
				if can_walk(y - 1, x) && prev[y - 1][x] == (0, 0) {
					prev[y - 1][x] = (y, x);
					next_queue.push_back((y - 1, x));
				}
				if can_walk(y, x - 1) && prev[y][x - 1] == (0, 0) {
					prev[y][x - 1] = (y, x);
					next_queue.push_back((y, x - 1));
				}
				if can_walk(y, x + 1) && prev[y][x + 1] == (0, 0) {
					prev[y][x + 1] = (y, x);
					next_queue.push_back((y, x + 1));
				}
				if can_walk(y + 1, x) && prev[y + 1][x] == (0, 0) {
					prev[y + 1][x] = (y, x);
					next_queue.push_back((y + 1, x));
				}
			}
			/*println!("{:?}", next_queue);
			for row in prev.iter() {
			for col in row.iter() {
			print!("{:?}", col)
			}
			println!();
			}*/
			queue = next_queue;
		}
		targets_found.sort();
		if let Some(target) = targets_found.into_iter().next() {
			//println!("target={:?}", target);
			let mut next = target;
			while prev[next.0][next.1] != (self.y, self.x) {
				next = prev[next.0][next.1];
				//println!("next={:?}", next);
			}
			self.y = next.0;
			self.x = next.1;
		}
	}

	fn attack(&self, units_map: &mut BTreeMap<(usize, usize), usize>, units: &mut [Unit]) {
		let mut enemies: Vec<_> = [
			(self.y - 1, self.x),
			(self.y, self.x - 1),
			(self.y, self.x + 1),
			(self.y + 1, self.x),
		]
		.iter()
		.filter_map(|&coord| units_map.get(&coord))
		.filter(|&&n| units[n].race != self.race && units[n].hp > 0)
		.map(|&n| n)
		.collect();
		enemies.sort_by_key(|&n| units[n].hp);
		if let Some(enemy) = enemies.into_iter().next() {
			let enemy = &mut units[enemy];
			enemy.hp -= self.power;
			if enemy.hp <= 0 {
				units_map.remove(&(enemy.y, enemy.x));
			}
		}
	}
}

#[derive(PartialEq)]
enum Square {
	Empty,
	Wall,
}

struct Field(Vec<Vec<Square>>);

struct Battle {
	field: Field,
	units: Vec<Unit>,
	units_map: BTreeMap<(usize, usize), usize>,
}

impl Battle {
	fn parse(input: &str, elfpower: i16) -> Self {
		let mut units = Vec::new();
		let mut field = Field(Vec::new());
		for (y, line) in input.lines().enumerate() {
			let row: Vec<Square> = line
				.chars()
				.map(|c| match c {
					'#' => Square::Wall,
					_ => Square::Empty,
				})
				.collect();
			field.0.push(row);
			for (x, c) in line.chars().enumerate() {
				match c {
					'G' | 'E' => units.push(Unit {
						x,
						y,
						hp: 200,
						race: Race(c),
						power: if c == 'G' { 3 } else { elfpower },
					}),
					_ => {}
				}
			}
		}
		let units_map = units
			.iter()
			.enumerate()
			.map(|(i, u)| ((u.y, u.x), i))
			.collect();
		Battle {
			field,
			units,
			units_map,
		}
	}

	fn parse_elf3(input: &str) -> Self {
		Self::parse(&input, 3)
	}

	#[cfg(test)]
	fn debug_string(&self) -> String {
		let mut v: Vec<Vec<char>> = self
			.field
			.0
			.iter()
			.map(|row| {
				row.iter()
					.map(|q| match q {
						Square::Empty => '.',
						Square::Wall => '#',
					})
					.collect()
			})
			.collect();
		for unit in &self.units {
			v[unit.y][unit.x] = unit.race.0;
		}
		v.iter().flat_map(|s| s.iter().chain(&['\n'])).collect()
	}

	fn over(&self) -> bool {
		let have = |r| {
			self.units
				.iter()
				.filter(|u| u.race == r && u.hp > 0)
				.next()
				.is_some()
		};
		!have(Race('E')) || !have(Race('G'))
	}

	fn round(&mut self) -> bool {
		for i in 0..self.units.len() {
			if self.over() {
				return true;
			}
			self.turn(i);
		}
		self.units.retain(|u| u.hp > 0);
		self.units.sort_by_key(|u| (u.y, u.x));
		self.units_map = self
			.units
			.iter()
			.enumerate()
			.filter(|&(_, u)| u.hp > 0)
			.map(|(i, u)| ((u.y, u.x), i))
			.collect();
		return false;
	}

	fn turn(&mut self, i: usize) {
		let mut unit = self.units[i].clone();
		if unit.hp <= 0 {
			return;
		}
		self.units_map.remove(&(unit.y, unit.x));
		unit.walk(&self.field, &self.units_map, &self.units);
		unit.attack(&mut self.units_map, &mut self.units);
		self.units_map.insert((unit.y, unit.x), i);
		self.units[i] = unit;
	}

	fn outcome(&mut self) -> usize {
		let mut counter = 0;
		loop {
			if self.round() {
				break;
			}
			counter += 1;
			println!("counter={}", counter);
			#[cfg(test)]
			println!("{}", self.debug_string());
			println!("{:?}", self.units.iter().map(|u| u.hp).collect::<Vec<_>>());
		}
		self.units.retain(|u| u.hp > 0);
		self.units.iter().map(|u| u.hp as usize).sum::<usize>() * counter
	}
}

fn _solve1(input: &str) -> usize {
	let mut battle = Battle::parse_elf3(&input);
	battle.outcome()
}

fn _solve2(input: &str) -> usize {
	let battle = Battle::parse_elf3(&input);
	let elves = battle.units.iter().filter(|u| u.race == Race('E')).count();
	for power in 4.. {
		let mut battle = Battle::parse(&input, power);
		let outcome = battle.outcome();
		if battle.units.iter().filter(|u| u.race == Race('E')).count() == elves {
			return outcome;
		}
	}
	0
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
	fn test_parse_elf3() {
		let field = include_str!("../ex1-1.txt");
		let battle = Battle::parse_elf3(&field);
		assert_eq!(battle.debug_string(), field);
	}

	#[test]
	fn test_walk() {
		let mut battle = Battle::parse_elf3(include_str!("../ex1-1.txt"));
		battle.turn(0);
		assert_eq!(battle.debug_string(), include_str!("../ex1-2.txt"));
	}

	#[test]
	fn test_walk_2() {
		let mut battle = Battle::parse_elf3(include_str!("../ex2-1.txt"));
		battle.turn(0);
		assert_eq!(battle.debug_string(), include_str!("../ex2-2.txt"));
	}

	#[test]
	fn test_walk_larger() {
		let mut battle = Battle::parse_elf3(include_str!("../ex3-1.txt"));
		battle.round();
		assert_eq!(battle.debug_string(), include_str!("../ex3-2.txt"));
		battle.round();
		assert_eq!(battle.debug_string(), include_str!("../ex3-3.txt"));
		battle.round();
		assert_eq!(battle.debug_string(), include_str!("../ex3-4.txt"));
	}

	#[test]
	fn test_attack() {
		let mut battle = Battle::parse_elf3(include_str!("../ex4-0.txt"));
		battle.round();
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[200, 197, 197, 200, 197, 197]
		);
		battle.round();
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[200, 200, 188, 194, 194, 194]
		);
		for _ in 2..23 {
			battle.round();
		}
		assert_eq!(battle.debug_string(), include_str!("../ex4-23.txt"));
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[200, 200, 131, 131, 131]
		);
	}

	#[test]
	fn test_outcome_1() {
		let mut battle = Battle::parse_elf3(include_str!("../ex4-0.txt"));
		assert_eq!(battle.outcome(), 27730);
		assert_eq!(battle.debug_string(), include_str!("../ex4-47.txt"));
	}

	#[test]
	fn test_outcome_2() {
		let mut battle = Battle::parse_elf3(include_str!("../ex5-0.txt"));
		assert_eq!(battle.outcome(), 36334);
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[200, 197, 185, 200, 200]
		);
		assert_eq!(battle.debug_string(), include_str!("../ex5-37.txt"));
	}

	#[test]
	fn test_outcome_3() {
		let mut battle = Battle::parse_elf3(include_str!("../ex6-0.txt"));
		assert_eq!(battle.outcome(), 39514);
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[164, 197, 200, 98, 200]
		);
		assert_eq!(battle.debug_string(), include_str!("../ex6-46.txt"));
	}

	#[test]
	fn test_outcome_4() {
		let mut battle = Battle::parse_elf3(include_str!("../ex7-0.txt"));
		assert_eq!(battle.outcome(), 27755);
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[200, 98, 200, 95, 200]
		);
		assert_eq!(battle.debug_string(), include_str!("../ex7-35.txt"));
	}

	#[test]
	fn test_outcome_5() {
		let mut battle = Battle::parse_elf3(include_str!("../ex8-0.txt"));
		assert_eq!(battle.outcome(), 28944);
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[200, 98, 38, 200]
		);
		assert_eq!(battle.debug_string(), include_str!("../ex8-54.txt"));
	}

	#[test]
	fn test_outcome_6() {
		let mut battle = Battle::parse_elf3(include_str!("../ex9-0.txt"));
		assert_eq!(battle.outcome(), 18740);
		assert_eq!(
			battle.units.iter().map(|u| u.hp).collect::<Vec<_>>(),
			&[137, 200, 200, 200, 200]
		);
		assert_eq!(battle.debug_string(), include_str!("../ex9-20.txt"));
	}

	#[test]
	fn test_power() {
		let input = include_str!("../ex4-0.txt");
		assert_eq!(_solve2(&input), 4988);
	}

	#[test]
	fn test_power_2() {
		let input = include_str!("../ex6-0.txt");
		assert_eq!(_solve2(&input), 31284);
	}

	#[test]
	fn test_power_3() {
		let input = include_str!("../ex7-0.txt");
		assert_eq!(_solve2(&input), 3478);
	}

	#[test]
	fn test_power_4() {
		let input = include_str!("../ex8-0.txt");
		assert_eq!(_solve2(&input), 6474);
	}

	#[test]
	fn test_power_5() {
		let input = include_str!("../ex9-0.txt");
		assert_eq!(_solve2(&input), 1140);
	}
}
