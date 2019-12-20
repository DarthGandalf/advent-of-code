use crate::NoneError;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Hash, Eq, PartialEq)]
struct Label(u16);

impl Label {
	fn new(c1: char, c2: char) -> Label {
		Label(c1 as u8 as u16 * 256 + c2 as u8 as u16)
	}
}

impl std::fmt::Debug for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			(self.0 / 256) as u8 as char,
			(self.0 % 256) as u8 as char
		)
	}
}

#[derive(Debug, Clone)]
struct Map {
	wall: Vec<Vec<bool>>,
	portals: fnv::FnvHashMap<(usize, usize), (usize, usize)>,
	aa: (usize, usize),
	zz: (usize, usize),
}

#[aoc_generator(day20)]
fn parse(input: &str) -> anyhow::Result<Map> {
	let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
	let mut inner_left = None;
	let mut inner_top = None;
	let mut inner_right = 0;
	let mut inner_bottom = 0;
	let wall: Vec<Vec<bool>> = grid
		.iter()
		.enumerate()
		.map(|(y, line)| {
			line.iter()
				.enumerate()
				.map(|(x, c)| {
					if x > 2 && x < grid[0].len() - 2 && y > 2 && y < grid.len() - 2 && *c == ' ' {
						inner_right = x;
						inner_bottom = y;
						if inner_left.is_none() {
							inner_left = Some(x);
						}
						if inner_top.is_none() {
							inner_top = Some(y);
						}
					}
					*c != '.'
				})
				.collect()
		})
		.collect();
	let inner_left = inner_left.unwrap_or_default();
	let inner_top = inner_top.unwrap_or_default();
	let bottom = grid.len() - 2;
	let right = grid[0].len() - 2;
	let mut positions: fnv::FnvHashMap<Label, Vec<(usize, usize)>> = Default::default();
	for x in 0..grid[0].len() {
		if grid[0][x] != ' ' {
			positions
				.entry(Label::new(grid[0][x], grid[1][x]))
				.or_default()
				.push((2, x));
		}
		if grid[bottom][x] != ' ' {
			positions
				.entry(Label::new(grid[bottom][x], grid[bottom + 1][x]))
				.or_default()
				.push((bottom - 1, x));
		}
	}
	for x in inner_left..=inner_right {
		if grid[inner_top][x] != ' ' {
			positions
				.entry(Label::new(grid[inner_top][x], grid[inner_top + 1][x]))
				.or_default()
				.push((inner_top - 1, x));
		}
		if grid[inner_bottom][x] != ' ' {
			positions
				.entry(Label::new(grid[inner_bottom - 1][x], grid[inner_bottom][x]))
				.or_default()
				.push((inner_bottom + 1, x));
		}
	}
	for y in 0..grid.len() {
		if grid[y][0] != ' ' {
			positions
				.entry(Label::new(grid[y][0], grid[y][1]))
				.or_default()
				.push((y, 2));
		}
		if grid[y][right] != ' ' {
			positions
				.entry(Label::new(grid[y][right], grid[y][right + 1]))
				.or_default()
				.push((y, right - 1));
		}
	}
	for y in inner_top..=inner_bottom {
		if grid[y][inner_left] != ' ' {
			positions
				.entry(Label::new(grid[y][inner_left], grid[y][inner_left + 1]))
				.or_default()
				.push((y, inner_left - 1));
		}
		if grid[y][inner_right] != ' ' {
			positions
				.entry(Label::new(grid[y][inner_right - 1], grid[y][inner_right]))
				.or_default()
				.push((y, inner_right + 1));
		}
	}
	//println!("{:?}", positions);
	let aa = positions
		.remove(&Label::new('A', 'A'))
		.none_err()?
		.into_iter()
		.next()
		.none_err()?;
	let zz = positions
		.remove(&Label::new('Z', 'Z'))
		.none_err()?
		.into_iter()
		.next()
		.none_err()?;
	//println!("{:?}->{:?}", aa, zz);
	let mut portals: fnv::FnvHashMap<(usize, usize), (usize, usize)> = Default::default();
	for ends in positions.values() {
		let a = ends.iter().nth(0).none_err()?;
		let b = ends.iter().nth(1).none_err()?;
		portals.insert(a.clone(), b.clone());
		portals.insert(b.clone(), a.clone());
	}
	//println!("parsed portals {:?}", portals);
	Ok(Map {
		wall,
		portals,
		aa,
		zz,
	})
}

#[aoc(day20, part1)]
fn part1(map: &Map) -> anyhow::Result<usize> {
	Ok(pathfinding::prelude::dijkstra(
		&map.aa,
		|n| {
			let mut neigh = vec![];
			if !map.wall[n.0][n.1 - 1] {
				neigh.push(((n.0, n.1 - 1), 1));
			}
			if !map.wall[n.0][n.1 + 1] {
				neigh.push(((n.0, n.1 + 1), 1));
			}
			if !map.wall[n.0 - 1][n.1] {
				neigh.push(((n.0 - 1, n.1), 1));
			}
			if !map.wall[n.0 + 1][n.1] {
				neigh.push(((n.0 + 1, n.1), 1));
			}
			if let Some(portal) = map.portals.get(&n) {
				neigh.push((*portal, 1));
			}
			neigh
		},
		|n| *n == map.zz,
	)
	.none_err()?
	.1)
}

#[aoc(day20, part2)]
fn part2(map: &Map) -> anyhow::Result<usize> {
	let bottom = map.wall.len() - 3;
	let right = map.wall[0].len() - 3;
	//println!("bottom={} right={}", bottom, right);
	let out_in: fnv::FnvHashMap<(usize, usize), (usize, usize)> = map
		.portals
		.iter()
		.filter(|&(p, _)| p.0 == 2 || p.1 == 2 || p.0 == bottom || p.1 == right)
		.map(|(p1, p2)| (*p1, *p2))
		.collect();
	let in_out: fnv::FnvHashMap<(usize, usize), (usize, usize)> = map
		.portals
		.iter()
		.filter(|&(_, p)| p.0 == 2 || p.1 == 2 || p.0 == bottom || p.1 == right)
		.map(|(p1, p2)| (*p1, *p2))
		.collect();
	//println!("{:?}", out_in);
	//println!("{:?}", in_out);
	Ok(pathfinding::prelude::dijkstra(
		&(0 as usize, map.aa.clone()),
		|node| {
			let level = node.0;
			let n = node.1;
			let mut neigh = vec![];
			if !map.wall[n.0][n.1 - 1] {
				neigh.push(((level, (n.0, n.1 - 1)), 1));
			}
			if !map.wall[n.0][n.1 + 1] {
				neigh.push(((level, (n.0, n.1 + 1)), 1));
			}
			if !map.wall[n.0 - 1][n.1] {
				neigh.push(((level, (n.0 - 1, n.1)), 1));
			}
			if !map.wall[n.0 + 1][n.1] {
				neigh.push(((level, (n.0 + 1, n.1)), 1));
			}
			if level > 0 {
				if let Some(portal) = out_in.get(&n) {
					neigh.push(((level - 1, *portal), 1));
				}
			}
			if let Some(portal) = in_out.get(&n) {
				neigh.push(((level + 1, *portal), 1));
			}
			//println!("{:?}: {:?}", node, neigh);
			neigh
		},
		|n| *n == (0, map.zz),
	)
	.none_err()?
	.1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		let input = parse(
			"
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "
				.trim_matches('\n'),
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 23);
		assert_eq!(part2(&input).unwrap(), 26);
	}

	#[test]
	fn test2() {
		let input = parse(
			"
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "
				.trim_matches('\n'),
		)
		.unwrap();
		assert_eq!(part1(&input).unwrap(), 58);
	}

	#[test]
	fn test3() {
		let input = parse(
			"
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "
				.trim_matches('\n'),
		)
		.unwrap();
		assert_eq!(part2(&input).unwrap(), 396);
	}

	#[test]
	fn answers() {
		let input = parse(include_str!("../input/2019/day20.txt")).unwrap();
		assert_eq!(part1(&input).unwrap(), 464);
		//assert_eq!(part2(&input).unwrap(), 1982);
	}
}
