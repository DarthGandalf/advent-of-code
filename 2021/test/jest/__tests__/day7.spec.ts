import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day7';
import { readFileSync } from 'fs';

describe('Day 7', () => {
  it('works', () => {
  const input = `16,1,2,0,4,2,7,1,2,14`;
    expect(solution(input)).toStrictEqual([37, 168]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day7.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([356179, 1]);
  })
});
