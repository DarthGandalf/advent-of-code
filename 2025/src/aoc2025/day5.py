import io
from aoc2025.utils import numbers

class Solver:
    def parse(self, f: io.TextIOBase):
        self.intervals = []
        self.ingredients = []
        for line in f:
            if not line.strip():
                break
            start, end = numbers(line)
            assert start <= end
            self.intervals.append(range(int(start), int(end) + 1))
        for line in f:
            self.ingredients.append(int(line))
        self.prepare()

    def part1(self):
        count = 0
        for ing in self.ingredients:
            if any(ing in r for r in self.intervals):
                count += 1
        return count

    def part2(self):
        return sum(len(r) for r in self.intervals)

    def prepare(self):
        edges = []
        for r in self.intervals:
            edges.append((r.start, 1))
            edges.append((r.stop, -1))
        edges.sort()

        state = 0
        current = 0
        result = []
        for e, change in edges:
            if state == 0:
                current = e
            state += change
            assert state >= 0
            if state == 0:
                if result and result[-1].stop == current:
                    result[-1] = range(result[-1].start, e)
                else:
                    result.append(range(current, e))
        self.intervals = result
