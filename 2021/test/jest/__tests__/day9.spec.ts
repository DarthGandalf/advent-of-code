import { describe, expect, it } from '@jest/globals';
import { part1, part2, solution } from '../../../solutions/day9';
import { readFileSync } from 'fs';

describe('Day 9', () => {
  it('works', () => {
    const input = `2199943210
3987894921
9856789892
8767896789
9899965678`;
    expect(part1(input)).toBe(15);
    expect(part2(input)).toBe(1134);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day9.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([564, 1038240]);
  })
});
