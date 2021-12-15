import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day15';
import { readFileSync } from 'fs';

describe('Day 15', () => {
  it('works', () => {
    const input = `
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`;
    expect(solution(input)).toStrictEqual([40, 315]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day15.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([702, 2955]);
  })
});
