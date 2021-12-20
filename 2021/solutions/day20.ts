function parse(input: string): [string, Set<string>] {
  const [rules_str, image_str] = input.split('\n\n');
  const rules = rules_str.split('\n').join('');
  const lines = image_str.split('\n');
  const image = new Set<string>();
  for (let y = 0; y < lines.length; ++y) {
    for (let x = 0; x < lines[y].length; ++x) {
      if (lines[y].charAt(x) == '#') {
        image.add(`${x} ${y}`);
      }
    }
  }
  return [rules, image];
}

function dims(image: Set<string>): {
  maxx: number,
  maxy: number,
  minx: number,
  miny: number,
} {
  let minx = Infinity;
  let maxx = -Infinity;
  let miny = Infinity;
  let maxy = -Infinity;
  for (const s of image) {
    const [x, y] = s.split(' ').map(Number);
    minx = Math.min(minx, x);
    maxx = Math.max(maxx, x);
    miny = Math.min(miny, y);
    maxy = Math.max(maxy, y);
  }
  return {minx, maxx, miny, maxy};
}

function apply(image: Set<string>, rules: string): Set<string> {
  const {minx, maxx, miny, maxy} = dims(image);
  const result = new Set<string>();
  for (let y = miny - 10; y <= maxy + 10; ++y) {
    for (let x = minx - 10; x <= maxx + 10; ++x) {
      let num = 0;
      for (let dy = -1; dy <= 1; ++dy) {
        for (let dx = -1; dx <= 1; ++dx) {
          num *= 2;
          if (image.has(`${x + dx} ${y + dy}`)) {
            num++;
          }
        }
      }
      if (rules.charAt(num) == '#') {
        result.add(`${x} ${y}`);
      }
    }
  }
  return result;
}

function apply2(image: Set<string>, rules: string): Set<string> {
  const {minx, maxx, miny, maxy} = dims(image);
  image = apply(image, rules);
  image = apply(image, rules);
  for (let y = miny - 30; y <= maxy + 30; ++y) {
    for (let x = minx - 30; x <= maxx + 30; ++x) {
      if (y <= miny - 7 || y >= maxy + 7 || x <= minx - 7 || x >= maxx + 7) {
        image.delete(`${x} ${y}`);
      }
    }
  }
  return image;
}

function visual(image: Set<string>): string {
  const {minx, maxx, miny, maxy} = dims(image);
  const rows: string[] = [];
  for (let y = miny; y <= maxy; ++y) {
    const row = [];
    for (let x = minx; x <= maxx; ++x) {
      if (image.has(`${x} ${y}`)) {
        row.push('#');
      } else {
        row.push('.');
      }
    }
    rows.push(row.join(''));
  }
  return rows.join('\n');
}

export function solution(input: string): number[] {
  input = input.trim();
  let [rules, image] = parse(input);
  image = apply2(image, rules):
  const part1 = image.size;
  for (let i = 0; i < 24; ++i) {
    image = apply2(image, rules);
  }
  return [part1, image.size];
}
