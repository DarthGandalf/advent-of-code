export function ints(input: string): number[] {
  const result: number[] = [];
  for (const i of input.matchAll(/-?\d+/g)) {
    result.push(parseInt(i[0]));
  }
  return result;
}
