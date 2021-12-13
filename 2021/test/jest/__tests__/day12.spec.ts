import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day12';
import { readFileSync } from 'fs';

describe('Day 12', () => {
  it('works', () => {
    let input = `start-A
start-b
A-c
A-b
b-d
A-end
b-end`;
    expect(solution(input)).toStrictEqual([10, 36]);

    input = `dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc`;
    expect(solution(input)).toStrictEqual([19, 103]);

    input = `fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW`;
    expect(solution(input)).toStrictEqual([226, 3509]);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day12.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([4304, 118242]);
  })
});
