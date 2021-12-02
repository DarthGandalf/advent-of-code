type Dir = "forward" | "down" | "up";
type Line = {
  dir: Dir;
  by: number;
};
type Input = Line[];

function parse(input: string): Input {
  const result = [];
  for (const line of input.split('\n')) {
    const [dir, num] = line.split(' ');
    const by = Number(num);
    result.push({dir: dir as Dir, by});
  }
  return result;
}

function part1(input: Input): number {
  let depth = 0;
  let fwd = 0;
  for (const { dir, by } of input) {
    switch (dir) {
      case "forward":
        fwd += by;
        break;
      case "down":
        depth += by;
        break;
      case "up":
        depth -= by;
        break;
    }
  }

  return depth * fwd;
}

function part2(input: Input): number {
  let depth = 0;
  let fwd = 0;
  let aim = 0;
  for (const { dir, by } of input) {
    switch (dir) {
      case "forward":
        fwd += by;
        depth += aim * by;
        break;
      case "down":
        aim += by;
        break;
      case "up":
        aim -= by;
        break;
    }
  }

  return depth * fwd;
}

export function solution(input: string): number[] {
  const x = parse(input);

  return [part1(x), part2(x)]
}
