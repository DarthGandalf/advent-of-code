import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day20';
import { readFileSync } from 'fs';

describe('Day 20', () => {
  it('works', () => {
    const input = `..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###`;
    expect(solution(input)).toStrictEqual([35, 3351]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day20.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([4873, 16394]);
  })
});
