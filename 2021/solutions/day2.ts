type Dir = 'forward' | 'down' | 'up';
type Line = {
  dir: Dir;
  by: number;
};

function* parse(input: string): Iterable<Line> {
  for (const line of input.split('\n')) {
    const [dir, num] = line.split(' ');
    const by = Number(num);
    yield {dir: dir as Dir, by};
  }
}

function part1(input: string): number {
  let depth = 0;
  let fwd = 0;
  for (const { dir, by } of parse(input)) {
    switch (dir) {
      case 'forward':
        fwd += by;
        break;
      case 'down':
        depth += by;
        break;
      case 'up':
        depth -= by;
        break;
    }
  }

  return depth * fwd;
}

function part2(input: string): number {
  let depth = 0;
  let fwd = 0;
  let aim = 0;
  for (const { dir, by } of parse(input)) {
    switch (dir) {
      case 'forward':
        fwd += by;
        depth += aim * by;
        break;
      case 'down':
        aim += by;
        break;
      case 'up':
        aim -= by;
        break;
    }
  }

  return depth * fwd;
}

export function solution(input: string): number[] {
  return [part1(input), part2(input)]
}
