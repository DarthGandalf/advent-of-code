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

function* neigh(L: number, row: number, col: number): Iterable<[number, number]> {
  if (row > 0) {
    yield [row - 1, col];
  }
  if (row < L - 1) {
    yield [row + 1, col];
  }
  if (col > 0) {
    yield [row, col - 1];
  }
  if (col < L - 1) {
    yield [row, col + 1];
  }
}

function solve(L: number, map: (y: number, x: number) => number): number {
  const dist = [];
  const done = [];
  for (let y = 0; y < L; ++y) {
    const done_row = [];
    const dist_row = [];
    for (let x = 0; x < L; ++x) {
      done_row[x] = 1;
      dist_row[x] = Infinity;
    }
    done.push(done_row);
    dist.push(dist_row);
  }
  dist[0][0] = 0;
  done[0][0] = 2;
  const queue = [[0, 0]];
  while (queue.length > 0) {
    let min_dist = Infinity;
    let min_dist_idx = 0;
    for (let i = 0; i < queue.length; ++i) {
      const [yy, xx] = queue[i];
      if (dist[yy][xx] < min_dist) {
        min_dist = dist[yy][xx];
        min_dist_idx = i;
      }
    }
    const [y, x] = queue[min_dist_idx];
    queue[min_dist_idx] = queue[queue.length - 1];
    queue.length--;
    done[y][x] = 3;
    for (const [yy, xx] of neigh(L, y, x)) {
      if (done[yy][xx] == 3) continue;
      dist[yy][xx] = Math.min(dist[yy][xx], dist[y][x] + map(yy, xx));
      if (done[yy][xx] == 1) {
        done[yy][xx]= 2;
        queue.push([yy, xx]);
      }
    }
  }
  return dist[L-1][L-1];
}

export function solution(input: string): number[] {
  const map = parse(input.trim());
  const L = map.length;
  return [
    solve(L, (y, x) => map[y][x]),
    solve(L * 5, (y, x) => (map[y % L][x % L] + Math.floor(x / L) + Math.floor(y / L) + 8) % 9 + 1)
  ];
}
