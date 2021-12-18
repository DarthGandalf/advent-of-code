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

export function solution1(input: string): number[] {
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

// -----------------------------------------------------------------

function simplify2(a: string): string {
  const foo = {a};
  function try_explode() {
    let index = 0;
    let depth = 0;
    for (let c of a) {
      if (c == '[') {
        depth++;
      } else if (c == ']') {
        depth--;
      }
      if (depth > 4) {
        const close = a.indexOf(']', index);
        let prefix = a.substring(0, index);
        let suffix = a.substring(close + 1);
        const [x, y] = a.substring(index + 1, close).split(',').map(Number);
        const prefix_matches = [...prefix.matchAll(/\d+/g)];
        if (prefix_matches.length > 0) {
          const lastmatch = prefix_matches[prefix_matches.length - 1];
          const num = Number(lastmatch[0]);
          const pr = prefix.substring(0, lastmatch.index);
          const suf = prefix.substring(lastmatch.index! + lastmatch[0].length)
          prefix = pr + Number(num + x) + suf;
        }
        const suffix_match = suffix.match(/\d+/);
        if (suffix_match) {
          const firstmatch = suffix_match;
          const num = Number(firstmatch[0]);
          const pr = suffix.substring(0, firstmatch.index);
          const suf = suffix.substring(firstmatch.index! + firstmatch[0].length);
          suffix = pr + Number(num + y) + suf;
        }
        a = `${prefix}0${suffix}`;
        return true;
      }
      index++;
    }
    return false;
  }
  function try_split() {
    const match = a.match(/\d\d+/);
    if (!match) {
      return false;
    }
    const num = Number(match[0]);
    const prefix = a.substring(0, match.index);
    const suffix = a.substring(match.index! + match[0].length);
    a = `${prefix}[${Math.floor(num/2)},${Math.floor((num+1)/2)}]${suffix}`;
    return true;
  }
  while (true) {
    if (try_explode()) continue;
    if (try_split()) continue;
    break;
  }
  return a;
}

function add2(a: string, b: string): string {
  return simplify2(`[${a},${b}]`);
}

function magnitude2(a: string): number {
  type Data = number | Data[]
  const data = JSON.parse(a) as Data;

  function mag2(b: Data): number {
    if (typeof b == 'number') {
      return b;
    } else {
      return mag2(b[0]) * 3 + mag2(b[1]) * 2
    }
  }
  return mag2(data);
}

function solution2(input: string): number[] {
  input = input.trim();
  const numbers = input.split('\n');
  const result = numbers.reduce(add2);
  let max = -Infinity;
  for (let i = 0; i < numbers.length; ++i) {
    for (let j = 0; j < numbers.length; ++j) {
      const summag = magnitude2(add2(numbers[i], numbers[j]));
      if (summag > max) {
        max = summag;
      }
    }
  }
  return [magnitude2(result), max];
}

export const solution = solution1;
