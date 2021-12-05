import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day5';
import { readFileSync } from 'fs';

describe('Day 5', () => {
  it('works', () => {
  const input = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`;
    expect(solution(input)).toStrictEqual([5, 12]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day5.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([5084, 17882]);
  })
});
