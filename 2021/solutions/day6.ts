import { ints } from './util';

function solve(input: string, days: number): number {
  const state = [];
  for (let i = 0; i < 9; ++i) {
    state.push(0);
  }
  for (const n of ints(input)) {
    state[n]++;
  }
  for (let day = 0; day < days; ++day) {
    const today: number = state.shift()!;
    state.push(today);
    state[6] += today;
  }
  let sum = 0;
  for (const n of state) {
    sum += n;
  }
  return sum;
}

export function solution(input: string): number[] {
  return [solve(input, 80), solve(input, 256)]
}
