import { describe, expect, it } from '@jest/globals';
import { part1, solution } from '../../../solutions/day13';
import { readFileSync } from 'fs';

describe('Day 13', () => {
  it('works', () => {
    let input = `6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`;
    expect(part1(input)).toBe(17);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day13.txt', {encoding:'utf8', flag:'r'});
    // EPUELPBR
    const output = `
####.###..#..#.####.#....###..###..###.
#....#..#.#..#.#....#....#..#.#..#.#..#
###..#..#.#..#.###..#....#..#.###..#..#
#....###..#..#.#....#....###..#..#.###.
#....#....#..#.#....#....#....#..#.#.#.
####.#.....##..####.####.#....###..#..#`;
    expect(solution(input)).toStrictEqual([770, output.trim()]);
  })
});
