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
  const result = [];
  if (row > 0) {
    result.push([row - 1, col]);
  }
  if (row < map.length - 1) {
    result.push([row + 1, col]);
  }
  if (col > 0) {
    result.push([row, col - 1]);
  }
  if (col < map[row].length - 1) {
    result.push([row, col + 1]);
  }
  return result as [number, number][];
}

export function part1(input: string): number {
  const map = parse(input);
  let sum = 0;
  for (let row = 0; row < map.length; ++row) {
    for (let col = 0; col < map[row].length; ++col) {
      let min = true;
      const here = map[row][col];
      for (const [r, c] of neigh(map, row, col)) {
        if (here >= map[r][c]) {
          min = false;
        }
      }
      if (min) {
        sum += here + 1;
      }
    }
  }
  return sum;
}

export function part2(input: string): number {
  const map = parse(input);
  const basins = [];
  for (let row = 0; row < map.length; ++row) {
    for (let col = 0; col < map[row].length; ++col) {
      if (map[row][col] >= 9) {
        continue;
      }
      const queue = [[row, col]];
      map[row][col] = 20;
      let size = 1;
      while (queue.length > 0) {
        const [ro, co] = queue.pop()!;
        for (const [r, c] of neigh(map, ro, co)) {
          if (map[r][c] < 9) {
            queue.push([r, c]);
            map[r][c] = 20;
            size++;
          }
        }
      }
      basins.push(size);
    }
  }
  basins.sort((a, b) => a - b);
  let result = 1;
  result *= basins.pop() || 1;
  result *= basins.pop() || 1;
  result *= basins.pop() || 1;
  return result;
}

export function solution(input: string): number[] {
  input = input.trim();
  return [part1(input), part2(input)];
}
