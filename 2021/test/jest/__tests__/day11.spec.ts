import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day11';
import { readFileSync } from 'fs';

describe('Day 11', () => {
  it('works', () => {
    const input = `5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`;
    expect(solution(input)).toStrictEqual([1656, 195]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day11.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([1655, 337]);
  })
});
