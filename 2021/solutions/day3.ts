function* parse(input: string): Iterable<string> {
  for (const line of input.split('\n')) {
    yield line;
  }
}

function part1(input: string): number {
  let counter = 0;
  const ones_by_pos = [];
  for (const line of parse(input)) {
    if (ones_by_pos.length === 0) {
      for (const b of line) {
        ones_by_pos.push(Number(b))
      }
    } else {
      let i = 0;
      for (const b of line) {
        if (b === '1') {
          ones_by_pos[i]++;
        }
        i++;
      }
    }
    counter += 1;
  }
  let gamma = 0;
  let epsilon = 0;
  for (const freq of ones_by_pos) {
    gamma *= 2;
    epsilon *= 2;
    if (freq >= counter / 2) {
      gamma += 1;
    } else {
      epsilon += 1;
    }
  }
  return gamma * epsilon;
}

function find_common(list: string[], choose: (list0: string[], list1: string[]) => string[]): number {
  for (let pos = 0;; pos++) {
    const list0 = [];
    const list1 = [];
    for (const line of list) {
      if (line.charAt(pos) == '1') {
        list1.push(line);
      } else {
        list0.push(line);
      }
    }

    list = choose(list0, list1);

    if (list.length === 1) {
      return parseInt(list[0], 2);
    }
  }
}

function part2(input: string): number {
  const lines = [];
  for (const line of parse(input)) {
    lines.push(line);
  }
  const oxygen = find_common(lines, (list0, list1) => {
    if (list0.length > list1.length) {
      return list0;
    } else {
      return list1;
    }
  });
  const co2 = find_common(lines, (list0, list1) => {
    if (list0.length <= list1.length) {
      return list0;
    } else {
      return list1;
    }
  });
  return oxygen * co2;
}

export function solution(input: string): number[] {
  input = input.trim();
  return [part1(input), part2(input)]
}
