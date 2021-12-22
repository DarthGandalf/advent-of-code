import { ints } from './util';

type Line = {
  state: boolean,
  x1: number,
  x2: number,
  y1: number,
  y2: number,
  z1: number,
  z2: number,
}

function parse(input: string): Line[] {
  const result = [];
  for (const line of input.split('\n')) {
    const [on, nums] = line.split(' ');
    const coords = ints(nums);
    result.push({
      state: on == 'on',
      x1: Math.min(coords[0], coords[1]),
      x2: Math.max(coords[0], coords[1]),
      y1: Math.min(coords[2], coords[3]),
      y2: Math.max(coords[2], coords[3]),
      z1: Math.min(coords[4], coords[5]),
      z2: Math.max(coords[4], coords[5]),
    });
  }
  return result;
}

function part1(lines: Line[]): number {
  const s = new Set();
  for (const {state, x1, x2, y1, y2, z1, z2} of lines) {
    if (Math.abs(x1) > 50) continue;
    if (Math.abs(x2) > 50) continue;
    if (Math.abs(y1) > 50) continue;
    if (Math.abs(y2) > 50) continue;
    if (Math.abs(z1) > 50) continue;
    if (Math.abs(z2) > 50) continue;
    for (let x = x1; x <= x2; ++x) {
      for (let y = y1; y <= y2; ++y) {
        for (let z = z1; z <= z2; ++z) {
          const key = `${x} ${y} ${z}`;
          if (state) {
            s.add(key);
          } else {
            s.delete(key);
          }
        }
      }
    }
  }
  return s.size;
}

function part2(lines: Line[]): number {
  const xs = new Set<number>();
  const ys = new Set<number>();
  const zs = new Set<number>();
  for (const {x1, x2, y1, y2, z1, z2} of lines) {
    xs.add(x1);
    xs.add(x2+1);
    ys.add(y1);
    ys.add(y2+1);
    zs.add(z1);
    zs.add(z2+1);
  }
  const X: number[] = [...xs.values()];
  const Y: number[] = [...ys.values()];
  const Z: number[] = [...zs.values()];
  X.sort((a: number, b: number) => a - b);
  Y.sort((a: number, b: number) => a - b);
  Z.sort((a: number, b: number) => a - b);
  let sum = 0;
  X.forEach((_, ix) => {
    if (ix == X.length - 1) return;
    Y.forEach((_, iy) => {
      if (iy == Y.length - 1) return;
      Z.forEach((_, iz) => {
        if (iz == Z.length - 1) return;
        let on = false;
        for (const {state, x1, x2, y1, y2, z1, z2} of lines) {
          if (X[ix] >= x1 && X[ix + 1] <= x2+1 && Y[iy] >= y1 && Y[iy + 1] <= y2+1 && Z[iz] >= z1 && Z[iz + 1] <= z2+1) {
            on = state;
          }
        }
        if (on) {
          sum += (X[ix + 1] - X[ix]) * (Y[iy + 1] - Y[iy]) * (Z[iz + 1] - Z[iz]);
        }
      })
    })
  });
  return sum;
}

export function solution(input: string): number[] {
  input = input.trim();
  const lines = parse(input);
  return [part1(lines), part2(lines)];
}
