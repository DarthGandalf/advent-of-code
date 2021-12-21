import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day21';
import { readFileSync } from 'fs';

describe('Day 21', () => {
  it('works', () => {
    const input = `
Player 1 starting position: 4
Player 2 starting position: 8`;
    expect(solution(input)).toStrictEqual([739785, 444356092776315]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day21.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([920079, 56852759190649]);
  })
});
