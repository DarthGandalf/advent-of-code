import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day6';
import { readFileSync } from 'fs';

describe('Day 6', () => {
  it('works', () => {
  const input = '3,4,3,1,2';
    expect(solution(input)).toStrictEqual([5934, 26984457539]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day6.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([374927, 1687617803407]);
  })
});
