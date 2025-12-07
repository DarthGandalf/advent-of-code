import io
from collections import defaultdict

class Solver:
    def parse(self, f: io.TextIOBase):
        self.lines = [l.strip() for l in f]

    def part1(self):
        count = 0
        beams = set([self.lines[0].find('S')])
        for line in self.lines:
            newbeams = set()
            for b in beams:
                if line[b] == '^':
                    newbeams.add(b + 1)
                    newbeams.add(b - 1)
                    count += 1
                else:
                    newbeams.add(b)
            beams = newbeams
        return count

    def part2(self):
        beams = {self.lines[0].find('S'): 1}
        for line in self.lines:
            newbeams = defaultdict(int)
            for b, t in beams.items():
                if line[b] == '^':
                    newbeams[b + 1] += t
                    newbeams[b - 1] += t
                else:
                    newbeams[b] += t
            beams = newbeams
        return sum(beams.values())
