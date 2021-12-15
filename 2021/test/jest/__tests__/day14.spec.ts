import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day14';
import { readFileSync } from 'fs';

describe('Day 14', () => {
  it('works', () => {
    const input = `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`;
    expect(solution(input)).toStrictEqual([1588, 2188189693529]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day14.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([2509, 2827627697643]);
  })
});
