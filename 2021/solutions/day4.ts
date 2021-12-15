import { ints } from './util';

class Board {
  numbers: number[][];
  marked: boolean[][];
  mrow: number[];
  mcol: number[];
  where: Map<number, [number, number]>;
  size: number;
  won: boolean;

  constructor(num: number[][]) {
    this.size = num.length;
    this.won = false;
    this.numbers = num;
    this.marked = [];
    this.mrow = [];
    this.mcol = [];
    this.where = new Map();
    for (const [i, row] of num.entries()) {
      const marked = [];
      for (const [j, val] of row.entries()) {
        marked.push(false);
        if (this.where.has(val)) {
          throw new Error('not unique')
        }
        this.where.set(val, [i, j]);
      }
      this.marked.push(marked);
      this.mrow.push(0);
      this.mcol.push(0);
    }
  }

  add(n: number): boolean {
    const where = this.where.get(n);
    if (where === undefined) {
      return false;
    }
    const [i, j] = where;
    if (this.marked[i][j]) {
      return false;
    }
    this.marked[i][j] = true;
    this.mrow[i]++;
    this.mcol[j]++;
    if (this.mrow[i] == this.size || this.mcol[j] == this.size) {
      this.won = true;
    }
    return this.won;
  }

  score(): number {
    let sum = 0;
    for (const [i, row] of this.numbers.entries()) {
      for (const [j, val] of row.entries()) {
        if (!this.marked[i][j]) {
          sum += val;
        }
      }
    }
    return sum;
  }
}

type Input = {
  boards: Board[];
  seq: number[];
}

function parse(input: string): Input {
  const blocks = input.split('\n\n');
  const seq = ints(blocks.shift()!);
  const boards = [];
  for (const block of blocks) {
    const rows = block.split('\n');
    const num = [];
    for (const row of rows) {
      const rown = [];
      for (const n of ints(row)) {
        rown.push(n);
      }
      num.push(rown);
    }
    boards.push(new Board(num));
  }
  return {
    boards,
    seq
  }
}

function part1(input: string): number {
  const {boards, seq} = parse(input);
  for (const num of seq) {
    for (const b of boards) {
      if (b.add(num)) {
        return b.score() * num;
      }
    }
  }
  return 0;
}

function part2(input: string): number {
  const {boards, seq} = parse(input);
  let won = 0;
  for (const num of seq) {
    for (const b of boards) {
      if (b.won) {
        continue;
      }
      if (b.add(num)) {
        won++;
        if (won == boards.length) {
          return b.score() * num;
        }
      }
    }
  }
  return 0;
}

export function solution(input: string): number[] {
  input = input.trim();
  return [part1(input), part2(input)]
}
