import { ints } from './util';

type Point = [number, number, number]

const Orientations: Array<(p: Point) => Point> = [
  ([x, y, z]) => [x, y, z],
  ([x, y, z]) => [x, -y, -z],
  ([x, y, z]) => [x, z, -y],
  ([x, y, z]) => [x, -z, y],

  ([x, y, z]) => [-x, -y, z],
  ([x, y, z]) => [-x, y, -z],
  ([x, y, z]) => [-x, z, y],
  ([x, y, z]) => [-x, -z, -y],

  ([x, y, z]) => [y, z, x],
  ([x, y, z]) => [y, -z, -x],
  ([x, y, z]) => [y, x, -z],
  ([x, y, z]) => [y, -x, z],

  ([x, y, z]) => [-y, -z, x],
  ([x, y, z]) => [-y, z, -x],
  ([x, y, z]) => [-y, -x, -z],
  ([x, y, z]) => [-y, x, z],

  ([x, y, z]) => [z, x, y],
  ([x, y, z]) => [z, -x, -y],
  ([x, y, z]) => [z, y, -x],
  ([x, y, z]) => [z, -y, x],

  ([x, y, z]) => [-z, -x, y],
  ([x, y, z]) => [-z, x, -y],
  ([x, y, z]) => [-z, y, x],
  ([x, y, z]) => [-z, -y, -x],
];

function near(a: Point, b: Point): boolean {
  if (Math.abs(a[0] - b[0]) > 1000) return false;
  if (Math.abs(a[1] - b[1]) > 1000) return false;
  if (Math.abs(a[2] - b[2]) > 1000) return false;
  return true;
}

class Scanner {
  private orientation: number = 0;
  private offset: Point = [0, 0, 0];
  public placed: boolean = false;

  constructor(
    public num: number,
    private beacons: Point[],
  ) {}

  get_beacons(): Point[] {
    if (this.placed) {
      return this.beacons;
    }
    const result = [];
    for (const b of this.beacons) {
      result.push(Orientations[this.orientation](b));
    }
    for (const b of result) {
      b[0] += this.offset[0];
      b[1] += this.offset[1];
      b[2] += this.offset[2];
    }
    const sorter = ([x1, y1, z1]: Point, [x2, y2, z2]: Point) => {
      if (x1 != x2) return x1 - x2;
      if (y1 != y2) return y1 - y2;
      return z1 - z2;
    }
    result.sort(sorter);
    return result;
  }

  private beacons_set: Set<string> = new Set();
  get_beacons_set(): Set<string> {
    return this.beacons_set;
  }

  fixate() {
    if (this.placed) throw new Error('placed already');
    const beacons = this.get_beacons();
    this.placed = true;
    this.beacons = beacons;
    for (const [x, y, z] of beacons) {
      this.beacons_set.add(`${x} ${y} ${z}`);
    }
  }

  get_position(): Point {
    return this.offset.map(t => t) as Point;
  }

  overlaps(other: Scanner): boolean {
    const other_beacons = other.get_beacons();
    const theirs = other.get_beacons_set();

    for (let o = 0; o < 24; ++o) {
      this.orientation = o;
      const my_beacons = this.get_beacons();
      for (let i = 0; i <= other_beacons.length - 12; ++i) {
        for (let j = 0; j <= my_beacons.length - 12; ++j) {
          const offset: Point = [0, 0, 0];
          offset[0] = other_beacons[j][0] - my_beacons[i][0];
          offset[1] = other_beacons[j][1] - my_beacons[i][1];
          offset[2] = other_beacons[j][2] - my_beacons[i][2];
          const adjusted_beacons = my_beacons.map(t => [
            t[0] + offset[0],
            t[1] + offset[1],
            t[2] + offset[2],
          ]);
          const shared = new Set<string>();
          for (const [x, y, z] of adjusted_beacons) {
            const key = `${x} ${y} ${z}`;
            if (theirs.has(key)) {
              shared.add(key);
            }
          }
          if (shared.size < 12) continue;
          let others_are_too_close = false;
          for (const [x, y, z] of adjusted_beacons) {
            const key = `${x} ${y} ${z}`;
            if (shared.has(key)) continue;
            if (near([x, y, z], other.get_position())) others_are_too_close = true;
          }
          if (others_are_too_close) continue;
          for (const [x, y, z] of other_beacons) {
            const key = `${x} ${y} ${z}`;
            if (shared.has(key)) continue;
            if (near([x, y, z], offset)) others_are_too_close = true;
          }
          if (others_are_too_close) continue;
          this.offset = offset;
          this.fixate();
          return true;
        }
      }
    }

    return false;
  }
}

function parse(input: string): Scanner[] {
  return input.split('\n\n').map((block) => {
    const i = ints(block);
    const n = i.shift();
    const points = [];
    for (let j = 0; j < i.length; j += 3) {
      points.push([i[j], i[j+1], i[j+2]] as Point);
    }
    return new Scanner(n!, points);
  });
}

export function solution(input: string): number[] {
  input = input.trim();
  const scanners = parse(input);
  scanners[0].fixate();
  const done = [scanners[0].num];
  while (true) {
    let found = false;
    for (const source of scanners) {
      if (!source.placed) continue;
      for (const s of scanners) {
        if (s.placed) continue;
        if (s.overlaps(source)) {
          found = true;
          done.push(s.num);
          break;
        }
      }
    }
    if (!found) {
      break;
    }
  }
  if (scanners.length != done.length) {
    throw new Error('not all scanners are placed')
  }

  const beacons = new Set<string>();
  for (const s of scanners) {
    for (const [x, y, z] of s.get_beacons()) {
      const key = `${x} ${y} ${z}`;
      beacons.add(key);
    }
  }

  let max = -Infinity;
  for (const a of scanners) {
    const ap = a.get_position();
    for (const b of scanners) {
      const bp = b.get_position();
      const dist = Math.abs(ap[0] - bp[0]) + Math.abs(ap[1] - bp[1]) + Math.abs(ap[2] - bp[2]);
      if (dist > max) max = dist;
    }
  }

  return [beacons.size, max];
}
