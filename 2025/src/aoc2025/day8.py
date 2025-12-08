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
        self.edges = edges
        self.s = s

    def part1(self, limit=1000):
        self.prepare()
        for _, i, j in self.edges[:limit]:
            self.s.union(i, j)
        self.edges = self.edges[limit:]
        return math.prod(sorted(len(u) for u in self.s.itersets())[-3:])

    def part2(self):
        while len(list(self.s.itersets())) > 1:
            _, i, j = self.edges.pop(0)
            self.s.union(i, j)
        return self.boxes[i][0] * self.boxes[j][0]
