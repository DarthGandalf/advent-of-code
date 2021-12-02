import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day2';
import { readFileSync } from 'fs';

describe('Day 2', () => {
  it('works', () => {
  const input = `forward 5
down 5
forward 8
up 3
down 8
forward 2`;
    expect(solution(input)).toStrictEqual([150, 900]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day2.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([1561344, 1848454425]);
  })
});
