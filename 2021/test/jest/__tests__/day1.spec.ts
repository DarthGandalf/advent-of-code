import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day1';

describe('Day 1', () => {
  it('works', () => {
    const input = `199
200
208
210
200
207
240
269
260
263`;
    expect(solution(input)).toStrictEqual(['7', '5']);
  });
});
