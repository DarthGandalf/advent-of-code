import {ints} from './util'

type Point = [number, number];

type Fold = {
  axis: number, // 1=y, 0=x
  num: number,
}

function parse(input: string): [Point[], Fold[]] {
  const points: Point[] = [];
  const folds: Fold[] = [];
  const [points_str, folds_str] = input.split('\n\n');
  for (const line of points_str.split('\n')) {
    const [x, y] = ints(line);
    points.push([x, y]);
  }
  for (const line of folds_str.split('\n')) {
    const [a, num] = line.split('=');
    folds.push({
      axis: a[a.length - 1] == 'y' ? 1 : 0,
      num: Number(num),
    });
  }
  return [points, folds];
}

function vis(points: Set<string>): string {
  let mix = 0;
  let miy = 0;
  let max = 0;
  let may = 0;
  const ps = [];
  for (const s of points) {
    const [x, y] = s.split(' ').map(Number);
    if (x > max) max = x;
    if (y > may) may = y;
    if (x < mix) mix = x;
    if (y < miy) miy = y;
    ps.push([x, y]);
  }
  const map = [];
  for (let y = 0; y <= may - miy; ++y) {
    const line = [];
    for (let x = 0; x <= max - mix; ++x) {
      line.push('.');
    }
    map.push(line);
  }
  for (const [x, y] of ps) {
    map[y - miy][x - mix] = '#';
  }
  return map.map((line) => line.join('')).join('\n');
}

export function part1(input: string): number {
  const [points_array, folds] = parse(input);
  const { axis, num } = folds[0];
  const new_points = new Set<string>();
  for (const p of points_array) {
    if (p[axis] > num) {
      p[axis] = 2*num - p[axis];
    }
    new_points.add(`${p[0]} ${p[1]}`);
  }
  return new_points.size;
}

export function part2(input: string): string {
  const [points_array, folds] = parse(input);
  let points = new Set<string>();
  for (const [x, y] of points_array) {
    points.add(`${x} ${y}`);
  }
  for (const { axis, num } of folds) {
    const new_points = new Set<string>();
    for (const s of points) {
      const p = s.split(' ').map(Number);
      if (p[axis] > num) {
        p[axis] = 2*num - p[axis];
      }
      new_points.add(`${p[0]} ${p[1]}`);
    }
    points = new_points;
  }
  return vis(points);
}

export function solution(input: string): [number, string] {
  input = input.trim();
  return [part1(input), part2(input)];
}
