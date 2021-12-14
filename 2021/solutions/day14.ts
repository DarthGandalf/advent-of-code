function parse(input: string): [string, Map<string, string>] {
  const map = new Map<string, string>();
  const [start, rest] = input.split('\n\n');
  for (const line of rest.split('\n')) {
    const [left, right] = line.split(' -> ');
    map.set(left, right);
  }
  return [start, map];
}

export function part1(input: string): number {
  let [str, map] = parse(input);
  for (let step = 0; step < 10; ++step) {
    const newstr = [str.charAt(0)];
    for (let i = 1; i < str.length; ++i) {
      const n = map.get(str.substr(i-1, 2));
      if (n !== undefined) {
        newstr.push(n);
      }
      newstr.push(str.charAt(i));
    }
    str = newstr.join('');
  }
  const freq = new Map<string, number>();
  for (const c of str) {
    const num = freq.get(c) ?? 0;
    freq.set(c, num + 1);
  }
  let min: number | null = null;
  let max: number | null = null;
  for (const n of freq.values()) {
    if (min == null || n < min) {
      min = n;
    }
    if (max == null || n > max) {
      max = n;
    }
  }
  return max! - min!;
}

export function part2(input: string): number {
  const [start, map] = parse(input);
  const chars = new Set<string>();
  for (const c of start) {
    chars.add(c);
  }
  for (const [left, right] of map.entries()) {
    for (const c of left) {
      chars.add(c);
    }
    chars.add(right);
  }
  if (chars.size * chars.size != map.size) {
    throw new Error("unsupported map size");
  }
  let inside = new Map<string, Map<string, number>>();
  for (const a of chars) {
    for (const b of chars) {
      const zeros = new Map<string, number>();
      for (const c of chars) {
        zeros.set(c, 0);
      }
      inside.set(`${a}${b}`, zeros);
    }
  }
  for (let step = 0; step < 40; ++step) {
    const inside_next = new Map<string, Map<string, number>>();
    for (const a of chars) {
      for (const b of chars) {
        const c = map.get(`${a}${b}`)!;
        const result = new Map<string, number>();
        const ac = inside.get(`${a}${c}`)!;
        const cb = inside.get(`${c}${b}`)!;
        for (const d of chars) {
          result.set(d, ac.get(d)! + cb.get(d)! + (c == d ? 1 : 0));
        }
        inside_next.set(`${a}${b}`, result);
      }
    }
    inside = inside_next;
  }
  const freq = new Map<string, number>();
  for (const c of chars) {
    freq.set(c, 0);
  }
  freq.set(start.charAt(0), 1);
  for (let i = 1; i < start.length; ++i) {
    freq.set(start.charAt(i), freq.get(start.charAt(i))! + 1);
    const add = inside.get(start.substr(i - 1, 2))!;
    for (const c of chars) {
      freq.set(c, freq.get(c)! + add.get(c)!);
    }
  }
  let min: number | null = null;
  let max: number | null = null;
  for (const n of freq.values()) {
    if (min == null || n < min) {
      min = n;
    }
    if (max == null || n > max) {
      max = n;
    }
  }
  return max! - min!;
}

export function solution(input: string): number[] {
  input = input.trim();
  return [part1(input), part2(input)];
}
