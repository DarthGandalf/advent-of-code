import { describe, expect, it } from '@jest/globals';
import { solution, Area, Result } from '../../../solutions/day17';
import { readFileSync } from 'fs';

describe('Day 17', () => {
  it('area', () => {
    const area = new Area(20, 30, -10, -5);
    expect(area.attempt(7, 2)).toBe(Result.Good)
    expect(area.attempt(8, 2)).toBe(Result.TooFar)
    expect(area.attempt(0, 0)).toBe(Result.TooNear)
    expect(area.attempt(100, 0)).toBe(Result.TooFastX)
    expect(area.attempt(0, 100)).toBe(Result.TooFastY)
  });
  it('works', () => {
    const input = 'target area: x=20..30, y=-10..-5';
    expect(solution(input)).toStrictEqual([45, 112]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day17.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([5995, 3202]);
  })
});
