function parse(input: string): Map<string, string[]> {
  const neigh = new Map<string, string[]>();
  for (const line of input.split('\n')) {
    const [a, b] = line.split('-');
    if (neigh.has(a)) {
      neigh.get(a)!.push(b);
    } else {
      neigh.set(a, [b]);
    }
    if (neigh.has(b)) {
      neigh.get(b)!.push(a);
    } else {
      neigh.set(b, [a]);
    }
  }
  return neigh;
}

export function part1(input: string): number {
  const neigh = parse(input);
  let numpaths = 0;
  const queue: string[][] = [['start']];
  while (queue.length > 0) {
    const current_path = queue.pop()!;
    const last_part = current_path[current_path.length - 1];
    if (last_part == 'end') {
      numpaths++;
      continue;
    }
    for (const next_part of neigh.get(last_part)!) {
      if (next_part.toUpperCase() == next_part || !current_path.includes(next_part)) {
        const new_path = JSON.parse(JSON.stringify(current_path)) as string[];
        new_path.push(next_part);
        queue.push(new_path);
      }
    }
  }
  return numpaths;
}

function count(path: string[], next: string): number {
  let num = 0;
  let dupe = 0;
  const s = new Set();
  for (const x of path) {
    if (x.toLowerCase() == x) {
      if (s.has(x)) {
        dupe = 1;
      }
      s.add(x);
    }
    if (x == next) {
      num++;
    }
  }
  return num + dupe;
}

export function part2(input: string): number {
  const neigh = parse(input);
  let numpaths = 0;
  const queue: string[][] = [['start']];
  while (queue.length > 0) {
    const current_path = queue.pop()!;
    const last_part = current_path[current_path.length - 1];
    for (const next_part of neigh.get(last_part)!) {
      if (next_part == 'start') {
        continue;
      }
      if (next_part == 'end') {
        numpaths++;
        continue;
      }
      if (next_part.toUpperCase() == next_part || count(current_path, next_part) < 2) {
        const new_path = JSON.parse(JSON.stringify(current_path)) as string[];
        new_path.push(next_part);
        queue.push(new_path);
      }
    }
  }
  return numpaths;
}

export function solution(input: string): number[] {
  input = input.trim();
  return [part1(input), part2(input)];
}
