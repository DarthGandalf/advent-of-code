import { color, log } from 'console-log-colors';
const { yellow, cyan } = color;

function parse(input: string): Int8Array[] {
  const result = [];
  for (const line of input.split('\n')) {
    const l = new Int8Array(line.length);
    let i = 0;
    for (const c of line) {
      switch (c) {
        case '.':
          l[i] = 0;
          break;
        case '>':
          l[i] = 1;
          break;
        case 'v':
          l[i] = 2;
          break;
      }
      i++;
    }
    result.push(l);
  }
  return result;
}

function east(map: Int8Array[]): boolean {
  let changed = false;
  map.forEach(line => {
    const moving: number[] = [];
    line.forEach((v, x) => {
      const nx = (x+1)%line.length;
      if (v == 1 && line[nx] == 0) {
        moving.push(x);
        changed = true;
      }
    })
    for (const x of moving) {
      line[x] = 0;
      line[(x+1)%line.length] = 1;
    }
  })
  return changed;
}

function south(map: Int8Array[]): boolean {
  let changed = false;
  for (let x = 0; x < map[0].length; ++x) {
    const moving: number[] = [];
    for (let y = 0; y < map.length; ++y) {
      const ny = (y+1)%map.length;
      if (map[y][x] == 2 && map[ny][x] == 0) {
        moving.push(y);
        changed = true;
      }
    }
    for (const y of moving) {
      map[y][x] = 0;
      map[(y+1)%map.length][x] = 2;
    }
  }
  return changed;
}

function vis(map: Int8Array[]) {
  for (const line of map) {
    console.log([...line].map((x) => ['.', yellow('>'), cyan('v')][x]).join(''))
  }
  console.log()
}

export function solution(input: string): number[] {
  input = input.trim();
  const map = parse(input);
  let steps = 0;
  while (true) {
    steps++;
    const e = east(map);
    const s = south(map);
    if (!e && !s) break;
  }
  vis(map);
  return [steps];
}
