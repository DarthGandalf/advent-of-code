import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day3';
import { readFileSync } from 'fs';

describe('Day 3', () => {
  it('works', () => {
  const input = `00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010`;
    expect(solution(input)).toStrictEqual([198, 230]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day3.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([852500, 1007985]);
  })
});
