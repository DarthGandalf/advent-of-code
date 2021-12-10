const openers = new Map([
  [')', '('],
  [']', '['],
  ['}', '{'],
  ['>', '<'],
]);
const score = new Map([
  [')', 3],
  [']', 57],
  ['}', 1197],
  ['>', 25137],
]);
const compscore = new Map([
  ['(', 1],
  ['[', 2],
  ['{', 3],
  ['<', 4],
]);

export function solution(input: string): number[] {
  input = input.trim();
  let sum = 0;
  const completions = [];
  for (const line of input.split('\n')) {
    const stack = [];
    let broken = false;
    for (const char of line) {
      if (broken) {
        break;
      }
      switch (char) {
        case '(':
        case '[':
        case '{':
        case '<':
          stack.push(char);
          break;
        case ')':
        case ']':
        case '}':
        case '>':
          if (stack[stack.length - 1] !== openers.get(char)) {
            broken = true;
            sum += score.get(char)!;
          } else {
            stack.pop();
          }
      }
    }
    if (!broken) {
      let comp = 0;
      stack.reverse();
      for (const char of stack) {
        comp = comp * 5 + compscore.get(char)!;
      }
      completions.push(comp);
    }
  }
  completions.sort((a, b) => a - b);
  return [sum, completions[(completions.length - 1) / 2]];
}
