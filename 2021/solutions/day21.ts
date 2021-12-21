import { ints } from './util'

function* dice(): Iterable<number> {
  let d = 1;
  while (true) {
    yield d;
    d++;
    if (d > 100) d = 1;
  }
}

function part1(p1: number, p2: number): number {
  let s1 = 0;
  let s2 = 0;
  let how = 0;
  let times = 0;
  for (const d of dice()) {
    times++;
    if (how < 3) {
      p1 = (p1 + d + 10 - 1) % 10 + 1;
    } else {
      p2 = (p2 + d + 10 - 1) % 10 + 1;
    }
    if (how == 2) {
      s1 += p1;
      if (s1 >= 1000) return s2 * times;
    } else if (how == 5) {
      s2 += p2;
      if (s2 >= 1000) return s1 * times;
    }
    how = (how + 1) % 6;
  }
  throw new Error("unreachable")
}

function* full(): Iterable<[number, number, number, number]> {
  for (let i = 0; i < 10; ++i) {
    for (let j = 0; j < 10; ++j) {
      for (let k = 0; k < 21; ++k) {
        for (let n = 0; n < 21; ++n) {
          yield [i, j, k, n]
        }
      }
    }
  }
}

function part2(p1: number, p2: number): number {
  let x = [];
  for (let i = 0; i < 10; ++i) {
    const row = [];
    for (let j = 0; j < 10; ++j) {
      const col = [];
      for (let k = 0; k <= 23; ++k) {
        const foo = [];
        for (let n = 0; n <= 23; ++n) {
          foo.push(0);
        }
        col.push(foo);
      }
      row.push(col);
    }
    x.push(row);
  }
  const empty = JSON.stringify(x);
  x[p1-1][p2-1][0][0] = 1;
  let how = 0;
  let won1 = 0;
  let won2 = 0;
  while (true) {
    if (how < 3) {
      let y = JSON.parse(empty);
      for (const [i, j, k, n] of full()) {
        y[i][j][k][n] = x[(i-1+10)%10][j][k][n]+x[(i-2+10)%10][j][k][n]+x[(i-3+10)%10][j][k][n];
      }
      x = y;
    } else {
      let y = JSON.parse(empty);
      for (const [i, j, k, n] of full()) {
        y[i][j][k][n] = x[i][(j-1+10)%10][k][n]+x[i][(j-2+10)%10][k][n]+x[i][(j-3+10)%10][k][n];
      }
      x = y;
    }
    if (how == 2) {
      let y = JSON.parse(empty);
      for (const [i, j, k, n] of full()) {
        if (i + 1 + k >= 21) {
          won1 += x[i][j][k][n];
        } else {
          y[i][j][i+1+k][n] += x[i][j][k][n];
        }
      }
      x = y;
    } else if (how == 5) {
      let y = JSON.parse(empty);
      for (const [i, j, k, n] of full()) {
        if (j + 1 + n >= 21) {
          won2 += x[i][j][k][n];
        } else {
          y[i][j][k][j+1+n] += x[i][j][k][n];
        }
      }
      x = y;
    }
    how = (how + 1) % 6;
    if (JSON.stringify(x) == empty) break;
  }
  return Math.max(won1, won2);
}

export function solution(input: string): number[] {
  input = input.trim();
  const [_unused_1, p1, _unused_2, p2] = ints(input);
  return [part1(p1, p2), part2(p1, p2)];
}
