export function part1(input: string): number {
  let num = 0;
  for (const line of input.split('\n')) {
    const [digits_str, output_str] = line.split(' | ');
    for (const o of output_str.split(' ')) {
      const l = o.length;
      if (l == 2 || l == 3 || l == 4 || l == 7) {
        num++;
      }
    }
  }
  return num;
}

export function part2(input: string): number {
  let sum = 0;
  for (const line of input.split('\n')) {
    const [digits_str, output_str] = line.split(' | ');
    const digits_strings = digits_str.split(' ');
    const output_strings = output_str.split(' ');

    {
      const sort_alpha = (str: string) => [...str].sort((a, b) => a.localeCompare(b)).join('')
      for (let i = 0; i < digits_strings.length; ++i) {
        digits_strings[i] = sort_alpha(digits_strings[i]);
      }
      for (let i = 0; i < output_strings.length; ++i) {
        output_strings[i] = sort_alpha(output_strings[i]);
      }
    }

    const digits = [];
    for (let i = 0; i < 10; ++i) {
      digits.push('');
    }
    const decrypt = new Map<string, number>();
    const new_digit_discovered = (s: string, n: number) => {
      digits[n] = s;
      decrypt.set(s, n);
    };

    for (const d of digits_strings) {
      switch (d.length) {
        case 2:
          new_digit_discovered(d, 1);
          break;
        case 3:
          new_digit_discovered(d, 7);
          break;
        case 4:
          new_digit_discovered(d, 4);
          break;
        case 7:
          new_digit_discovered(d, 8);
          break;
      }
    }

    let segment_a = '';
    {
      const s = new Set(digits[7]);
      for (const x of digits[1]) {
        s.delete(x);
      }
      segment_a = s.values().next().value;
    }

    let segments_bd = [];
    {
      const s = new Set(digits[4]);
      for (const x of digits[1]) {
        s.delete(x);
      }
      segments_bd = [...s];
    }

    let segment_b = '';
    let segment_f = '';
    for (const d of digits_strings) {
      if (d.length == 6) {
        const dd = [...d];
        if (!dd.includes(segments_bd[0]) || !dd.includes(segments_bd[1])) {
          new_digit_discovered(d, 0);
          if (dd.includes(segments_bd[0])) {
            segment_b = segments_bd[0];
          } else {
            segment_b = segments_bd[1];
          }
        }
        if (!dd.includes(digits[1][0]) || !dd.includes(digits[1][1])) {
          new_digit_discovered(d, 6);
          if (dd.includes(digits[1][0])) {
            segment_f = digits[1][0];
          } else {
            segment_f = digits[1][1];
          }
        }
      }
    }

    for (const d of digits_strings) {
      if (d.length == 5) {
        const dd = [...d];
        if (!dd.includes(segment_b) && !dd.includes(segment_f)) {
          new_digit_discovered(d, 2);
        }
      }
    }

    let unknown_digits = digits_strings.filter((s) => !decrypt.has(s));

    for (const d of unknown_digits) {
      if (d.length == 6) {
        new_digit_discovered(d, 9);
      } else if ([...d].includes(segment_b)) {
        new_digit_discovered(d, 5);
      } else {
        new_digit_discovered(d, 3);
      }
    }

    const output =  output_strings.reduce((acc, cur) => acc * 10 + decrypt.get(cur)!, 0);
    sum += output;
  }
  return sum;
}

export function solution(input: string): number[] {
  input = input.trim();
  return [part1(input), part2(input)];
}
