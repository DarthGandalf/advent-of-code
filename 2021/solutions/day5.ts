import { ints } from './util';

type Line = {
  a: [number, number];
  b: [number, number];
}

class State {
  map1 = new Map<string, number>();
  map2 = new Map<string, number>();

  static #key(point: [number, number]): string {
    const [x, y] = point;
    return `${x},${y}`;
  }

  static #addPoint(maps: Map<string, number>[], point: [number, number]) {
    const key = State.#key(point);
    for (const map of maps) {
      const val = map.get(key) || 0;
      map.set(key, val + 1);
    }
  }

  add(line: Line) {
    const [x1, y1] = line.a;
    const [x2, y2] = line.b;
    if (y1 == y2) {
      const mix = Math.min(x1, x2);
      const max = Math.max(x1, x2);
      for (let x = mix; x <= max; x++) {
        State.#addPoint([this.map1, this.map2], [x, y1]);
      }
    } else if (x1 == x2) {
      const miy = Math.min(y1, y2);
      const may = Math.max(y1, y2);
      for (let y = miy; y <= may; y++) {
        State.#addPoint([this.map1, this.map2], [x1, y]);
      }
    } else if (x1 - x2 == y1 - y2) {
      const mix = Math.min(x1, x2);
      const max = Math.max(x1, x2);
      for (let x = mix, y = Math.min(y1, y2); x <= max; x++, y++) {
        State.#addPoint([this.map2], [x, y]);
      }
    } else if (x1 - x2 == y2 - y1) {
      const mix = Math.min(x1, x2);
      const max = Math.max(x1, x2);
      for (let x = mix, y = Math.max(y1, y2); x <= max; x++, y--) {
        State.#addPoint([this.map2], [x, y]);
      }
    } else {
      throw new Error('invalid line')
    }
  }
}

function parse(input: string): Line[] {
  const lines = input.split('\n');
  const result = [];
  for (const line of lines) {
    const [x1, y1, x2, y2] = ints(line);
    if (y2 === undefined) {
      throw new Error("wrong input line " + line)
    }
    result.push({
      a: [x1, y1] as [number, number],
      b: [x2, y2] as [number, number],
    });
  }
  return result;
}

function count(map: Map<string, number>): number {
  let n = 0;
  for (const z of map.values()) {
    if (z >= 2) {
      n++;
    }
  }
  return n;
}

export function solution(input: string): number[] {
  input = input.trim();
  const state = new State();
  const lines = parse(input);
  for (const line of lines) {
    state.add(line);
  }
  return [count(state.map1), count(state.map2)]
}
