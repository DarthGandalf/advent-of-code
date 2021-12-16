import { describe, expect, it } from '@jest/globals';
import { solution } from '../../../solutions/day16';
import { readFileSync } from 'fs';

describe('Day 16', () => {
  it('works for part 1', () => {
    let input = 'D2FE28';
    input = '38006F45291200'
    input = 'EE00D40C823060'
    input = '8A004A801A8002F478'
    expect(solution(input)[0]).toBe(16);
    input = '620080001611562C8802118E34'
    expect(solution(input)[0]).toBe(12);
    input = 'C0015000016115A2E0802F182340'
    expect(solution(input)[0]).toBe(23);
    input = 'A0016C880162017C3686B18A3D4780'
    expect(solution(input)[0]).toBe(31);
  });

  it('works for part 2', () => {
    let input = 'C200B40A82';
    expect(solution(input)[1]).toBe(3);
    input = '04005AC33890';
    expect(solution(input)[1]).toBe(54);
    input = '880086C3E88112';
    expect(solution(input)[1]).toBe(7);
    input = 'CE00C43D881120';
    expect(solution(input)[1]).toBe(9);
    input = 'D8005AC2A8F0';
    expect(solution(input)[1]).toBe(1);
    input = 'F600BC2D8F';
    expect(solution(input)[1]).toBe(0);
    input = '9C005AC2F8F0';
    expect(solution(input)[1]).toBe(0);
    input = '9C0141080250320F1802104A08';
    expect(solution(input)[1]).toBe(1);
  });

  it('answers', () => {
    const input = readFileSync('public/input/day16.txt', {encoding:'utf8', flag:'r'});
    expect(solution(input)).toStrictEqual([945, 10637009915279]);
  })
});
