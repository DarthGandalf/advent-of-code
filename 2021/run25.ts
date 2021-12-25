import { solution } from './solutions/day25';
import { readFileSync } from 'fs';

let input = `
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>`;
input = readFileSync('public/input/day25.txt', {encoding:'utf8', flag:'r'});
const result = solution(input);
console.log(result)
