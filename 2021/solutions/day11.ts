function parse(input: string): number[][] {
  const result = [];
  for (const line of input.split('\n')) {
    const l = [];
    for (const c of line) {
      l.push(Number(c))
    }
    result.push(l);
  }
  return result;
}

function neigh(map: number[][], row: number, col: number): [number, number][] {
  const result: [number, number][] = [];
  for (const r of [row - 1, row, row + 1]) {
    if (r < 0 || r >= map.length) continue;
    for (const c of [col - 1, col, col + 1]) {
      if (c < 0 || c >= map[r].length) continue;
      result.push([r, c]);
    }
  }
  return result;
}

export function solution(input: string): number[] {
  input = input.trim();
  const map = parse(input);
  let num1 = 0;
  let step = 0;
  const do_step = () => {
    const flashes = new Set<string>();
    const queue: [number, number][] = [];
    const increase = (row: number, col: number) => {
      if (flashes.has(`${row}x${col}`)) return;
      map[row][col]++;
      if (map[row][col] < 10) return;
      queue.push([row, col]);
      flashes.add(`${row}x${col}`);
      if (step < 100) {
        num1++;
      }
    };
    for (let row = 0; row < map.length; ++row) {
      for (let col = 0; col < map[row].length; ++col) {
        increase(row, col);
      }
    }
    while (queue.length > 0) {
      const [row, col] = queue.pop()!;
      for (const [r, c] of neigh(map, row, col)) {
        increase(r, c);
      }
    }
    for (const pos of flashes) {
      const [row, col] = pos.split('x');
      const r = Number(row);
      const c = Number(col);
      map[r][c] = 0;
    }
    return flashes.size;
  }
  while (true) {
    if (do_step() == 100) {
      break;
    }
    ++step;
  }
  return [num1, step + 1];
}
