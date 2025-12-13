import io
from aoc2025.utils import numbers

class Solver:
    def parse(self, f: io.TextIOBase):
        blocks = f.read().strip().split('\n\n')
        self.blocks = blocks[:-1]
        self.configs = blocks[-1]

    def part1(self):
        sizes = [b.count('#') for b in self.blocks]
        count = 0
        for line in self.configs.splitlines():
            x = numbers(line)
            if (x[0] // 3) * (x[1] // 3) >= sum(x[2:]):
                count += 1
            elif sum(a*b for a, b in zip(sizes, x[2:])) > x[0] * x[1]:
                pass
            else:
                raise Exception("need to investigate further and to actually solve this puzzle")
        return count

    def part2(self):
        pass
