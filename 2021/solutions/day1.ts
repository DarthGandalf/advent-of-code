import { ints } from './util';

export function solution(input: string): string[] {
  const z = ints(input);

  let result = 0;
  for (let i = 1; i < z.length; ++i) {
    if (z[i-1]<z[i]) result++;
  }

  let result2 = 0;
  for (let i = 3; i < z.length; ++i) {
    if (z[i-3] < z[i]) result2++;
  }

  return [`${result}`, `${result2}`];
}


// 1: 1387
// 2: 1362
