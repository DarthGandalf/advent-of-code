function* bits(input: string): Iterable<0|1> {
  for (const c of input) {
    const num = parseInt(c, 16);
    let str = num.toString(2);
    while (str.length < 4) {
      str = '0' + str;
    }
    yield* str.split('').map(Number) as Array<0|1>;
  }
}

function readnum(input: Iterator<0|1>, len: number): number {
  let result = 0;
  for (let i = 0; i < len; ++i) {
    const { done, value } = input.next();
    if (done) {
      throw true;
    }
    result *= 2;
    result += value;
  }
  return result;
}

type Packet = {
  version: number,
  type: number,
  literal?: number,
  subpackets: Packet[],
};

function parse(input: Iterator<0|1>): Packet {
  const version = readnum(input, 3);
  const type = readnum(input, 3);
  const subpackets: Packet[] = [];
  if (type == 4) {
    let more = 1;
    let literal = 0;
    while (more) {
      more = readnum(input, 1);
      literal *= 0b10000;
      literal += readnum(input, 4);
    }
    return {
      version,
      type,
      literal,
      subpackets,
    };
  } else {
    const len_type = readnum(input, 1);
    if (len_type == 0) {
      const bit_len = readnum(input, 15);
      const data = [];
      for (let i = 0; i < bit_len; ++i) {
        data.push(input.next().value);
      }
      try {
        let iter = data[Symbol.iterator]();
        while (true) {
          subpackets.push(parse(iter));
        }
      } catch (e) {}
    } else {
      const sub_num = readnum(input, 11);
      for (let i = 0; i < sub_num; ++i) {
        subpackets.push(parse(input));
      }
    }
    return {
      version,
      type,
      subpackets,
    };
  }
}

function visual(tree: Packet, offset: number = 0): string {
  let result = '';
  for (let i = 0; i < offset; ++i) {
    result += ' ';
  }
  result += `V=${tree.version} T=${tree.type} `;
  if (tree.literal !== undefined) {
    result += `L=${tree.literal}`;
  }
  result += '\n';
  for (const p of tree.subpackets) {
    result += visual(p, offset + 4);
  }
  return result;
}

function version_sum(tree: Packet): number {
  let sum = tree.version;
  for (const p of tree.subpackets) {
    sum += version_sum(p);
  }
  return sum;
}

function eval_tree(tree: Packet): number {
  switch (tree.type) {
    case 0:
      return tree.subpackets.map(eval_tree).reduce((a, b) => a + b);
    case 1:
      return tree.subpackets.map(eval_tree).reduce((a, b) => a * b);
    case 2:
      return tree.subpackets.map(eval_tree).reduce((a, b) => Math.min(a, b));
    case 3:
      return tree.subpackets.map(eval_tree).reduce((a, b) => Math.max(a, b));
    case 4:
      return tree.literal!;
    case 5:
      return eval_tree(tree.subpackets[0]) > eval_tree(tree.subpackets[1]) ? 1 : 0;
    case 6:
      return eval_tree(tree.subpackets[0]) < eval_tree(tree.subpackets[1]) ? 1 : 0;
    case 7:
      return eval_tree(tree.subpackets[0]) == eval_tree(tree.subpackets[1]) ? 1 : 0;
  }
  throw new Error("wrong type")
}

export function solution(input: string): number[] {
  input = input.trim();
  const tree = parse(bits(input)[Symbol.iterator]());
  //console.log(visual(tree));
  return [version_sum(tree), eval_tree(tree)];
}
