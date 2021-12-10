import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day10';
import { readFileSync } from 'fs';

describe('Day 10', () => {
  it('works', () => {
    const input = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`;
    expect(solution(input)).toStrictEqual([26397, 288957]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day10.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([369105, 3999363569]);
  })
});
