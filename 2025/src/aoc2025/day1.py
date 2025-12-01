import io
from aoc2025.utils import numbers

class Solver:
    def parse(self, f: io.TextIOBase):
        self.lines = [(l[0], numbers(l)[0]) for l in f.readlines()]

    def part1(self):
        zeros = 0
        now = 50
        for char, off in self.lines:
            match char:
                case 'L':
                    now -= off
                case 'R':
                    now += off
            now %= 100
            if now == 0:
                zeros += 1
        return zeros

    def part2(self):
        zeros = 0
        now = 50
        for char, off in self.lines:
            if off == 0:
                continue
            match char:
                case 'L':
                    now -= 1
                    zeros += now // 100 - (now - off) // 100
                    now += 1
                    now -= off
                case 'R':
                    zeros += (now + off) // 100 - now // 100
                    now += off
        return zeros
