import io
import re
from aoc2025.utils import numbers

inv1 = re.compile(r'^(\d+)\1$')
inv2 = re.compile(r'^(\d+)(:?\1)+$')

class Solvationist:
    def parse(self, f: io.TextIOBase):
        self.ranges = []
        for range in f.readline().split(','):
            s, e = range.split('-')
            s = int(s)
            e = int(e)
            self.ranges.append((s, e))

    def part(self, invalid):
        count = 0
        for s, e in self.ranges:
            for x in range(s, e+1):
                if invalid.match(str(x)):
                    count += x
        return count

    def part1(self):
        return self.part(inv1)

    def part2(self):
        return self.part(inv2)
