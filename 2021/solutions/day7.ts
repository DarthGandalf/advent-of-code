import { ints } from './util';

function part1(input: string): number {
  const crabs = ints(input);
  crabs.sort((a, b) => a - b);
  const mincrab = crabs[0];
  const maxcrab = crabs[crabs.length-1];
  const left = [];
  const right = [];
  for (let i = mincrab; i <= maxcrab; ++i) {
    left.push(-1);
    right.push(-1);
  }
  let index = 0;
  let leftfuel = 0;
  for (let i = mincrab; i <= maxcrab; ++i) {
    while (index < crabs.length && crabs[index] < i) {
      index++;
    }
    leftfuel += index;
    left[i] = leftfuel;
  }
  index = crabs.length - 1;
  let rightfuel = 0;
  for (let i = maxcrab; i >= mincrab; --i) {
    while (index >= 0 && crabs[index] > i) {
      index--;
    }
    rightfuel += crabs.length - 1 - index;
    right[i] = rightfuel;
  }
  const sum = [];
  for (let i = mincrab; i <= maxcrab; ++i) {
    sum.push(left[i] + right[i]);
  }
  return Math.min(...sum);
}

function part2(input: string): number {
  const crabs = ints(input);
  crabs.sort((a, b) => a - b);
  const mincrab = crabs[0];
  const maxcrab = crabs[crabs.length-1];
  const left = [];
  const right = [];
  for (let i = mincrab; i <= maxcrab; ++i) {
    left.push(-1);
    right.push(-1);
  }
  let index = 0;
  let leftfuel = 0;
  let leftinc = 0;
  for (let i = mincrab; i <= maxcrab; ++i) {
    while (index < crabs.length && crabs[index] < i) {
      index++;
    }
    leftfuel += index;
    leftfuel += leftinc;
    left[i] = leftfuel;
    leftinc += index;
  }
  index = crabs.length - 1;
  let rightfuel = 0;
  let rightinc = 0;
  for (let i = maxcrab; i >= mincrab; --i) {
    while (index >= 0 && crabs[index] > i) {
      index--;
    }
    rightfuel += crabs.length - 1 - index;
    rightfuel += rightinc;
    right[i] = rightfuel;
    rightinc += crabs.length - 1 - index;
  }
  const sum = [];
  for (let i = mincrab; i <= maxcrab; ++i) {
    sum.push(left[i] + right[i]);
  }
  return Math.min(...sum);
}

export function solution(input: string): number[] {
  return [part1(input), part2(input)];
}
