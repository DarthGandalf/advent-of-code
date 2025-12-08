import io
from aoc2025.utils import numbers
from disjoint_set import DisjointSet
import math

class Solver:
    def parse(self, f: io.TextIOBase):
        self.boxes = [tuple(numbers(l)) for l in f]

    def prepare(self):
        s = DisjointSet()
        for i, _ in enumerate(self.boxes):
            s.find(i)
        edges = []
        for i, b1 in enumerate(self.boxes):
            for j in range(i+1, len(self.boxes)):
                b2 = self.boxes[j]
                dist = sum((x - y) ** 2 for x, y in zip(b1, b2))
                edges.append((dist, i, j))
        edges.sort()
        return s, edges

    def part1(self, limit=1000):
        s, edges = self.prepare()
        for _, i, j in edges[:limit]:
            s.union(i, j)
        return math.prod(sorted(len(u) for u in s.itersets())[-3:])

    def part2(self):
        s, edges = self.prepare()
        while len(list(s.itersets())) > 1:
            _, i, j = edges.pop(0)
            s.union(i, j)
        return self.boxes[i][0] * self.boxes[j][0]
