type Literal = {
  kind: 1;
  num: number;
  parent?: Pair;
}
type Pair = {
  kind: 2;
  nums: [SnailNum, SnailNum];
  parent?: Pair;
}
type SnailNum = Literal | Pair

function dump(input: SnailNum): string {
  switch (input.kind) {
    case 1:
      return `${input.num}`;
    case 2:
      return `[ ${dump(input.nums[0])}, ${dump(input.nums[1])} ]`;
  }
}

function parse_line(line: string): SnailNum {
  function read_char(input: Iterator<string>, error: string): string {
    const { done, value } = input.next();
    if (done) throw new Error(`EOF ${error}`);
    return value;
  }

  function expect_char(input: Iterator<string>, expected: string) {
    const char = read_char(input, `expected '${expected}'`);
    if (char !== expected) throw new Error(`Expected '${expected}', got '${char}'`);
  }

  function parse_num(input: Iterator<string>): SnailNum {
    const value = read_char(input, 'want start');
    if (value == '[') {
      const a = parse_num(input);
      expect_char(input, ',');
      const b = parse_num(input);
      expect_char(input, ']');
      const result: SnailNum = {
        kind: 2,
        nums: [a, b],
      };
      a.parent = result;
      b.parent = result;
      return result;
    }
    return {
      kind: 1,
      num: Number(value)
    }
  }

  return parse_num(line[Symbol.iterator]());
}

function simplify(a: SnailNum) {
  function maybe_add_left(source: Pair, to: SnailNum, from: Pair): boolean {
    if (to.kind == 1) {
      to.num += (source.nums[0] as Literal).num;
      return true;
    }
    if (to.nums[1] == from) {
      return maybe_add_left(source, to.nums[0], to) || (to.parent != undefined && maybe_add_left(source, to.parent, to));
    } else if (to.nums[0] == from) {
      return to.parent != undefined && maybe_add_left(source, to.parent, to);
    } else {
      return maybe_add_left(source, to.nums[1], to) || maybe_add_left(source, to.nums[0], to)
    }
  }
  function maybe_add_right(source: Pair, to: SnailNum, from: Pair): boolean {
    if (to.kind == 1) {
      to.num += (source.nums[1] as Literal).num;
      return true;
    }
    if (to.nums[0] == from) {
      return maybe_add_right(source, to.nums[1], to) || (to.parent != undefined && maybe_add_right(source, to.parent, to));
    } else if (to.nums[1] == from) {
      return to.parent != undefined && maybe_add_right(source, to.parent, to);
    } else {
      return maybe_add_right(source, to.nums[0], to) || maybe_add_right(source, to.nums[1], to);
    }
  }

  function try_explode(b: SnailNum, depth = 0, child = 0): boolean {
    if (b.kind == 1) return false;
    if (depth == 4) {
      const parent = b.parent!;
      maybe_add_left(b, parent, b);
      maybe_add_right(b, parent, b);
      parent.nums[child] = {
        kind: 1,
        num: 0,
        parent,
      };
      return true;
    }
    return try_explode(b.nums[0], depth + 1, 0) || try_explode(b.nums[1], depth + 1, 1);
  }

  function try_split(b: SnailNum): boolean {
    switch (b.kind) {
      case 1:
        if (b.num >= 10) {
          const parent = b.parent!;
          const child = parent.nums[0] == b ? 0 : 1;
          const node: SnailNum = {
            kind: 2,
            nums: [{
              kind: 1,
              num: Math.floor(b.num / 2),
            }, {
              kind: 1,
              num: Math.floor((b.num + 1) / 2),
            }],
            parent,
          };
          node.nums[0].parent = node;
          node.nums[1].parent = node;
          parent.nums[child] = node;
          return true;
        };
        return false;
      case 2:
        return try_split(b.nums[0]) || try_split(b.nums[1]);
    }
  }

  while (true) {
    if (try_explode(a)) {
      continue;
    }

    if (try_split(a)) {
      continue;
    }

    break;
  }
}

function add(a: SnailNum, b: SnailNum): SnailNum {
  const result: SnailNum = {
    kind: 2,
    nums: [a, b]
  };
  a.parent = result;
  b.parent = result;
  simplify(result);
  return result;
}

function magnitude(a: SnailNum): number {
  switch (a.kind) {
    case 1:
      return a.num;
    case 2:
      return magnitude(a.nums[0]) * 3 + magnitude(a.nums[1]) * 2;
  }
}

export function solution(input: string): number[] {
  input = input.trim();
  const numbers = input.split('\n');
  const result = numbers.map(parse_line).reduce(add);
  let max = -Infinity;
  for (let i = 0; i < numbers.length; ++i) {
    for (let j = 0; j < numbers.length; ++j) {
      const summag = magnitude(add(parse_line(numbers[i]), parse_line(numbers[j])));
      if (summag > max) {
        max = summag;
      }
    }
  }
  return [magnitude(result), max]
}
