import io
from aoc2025.utils import numbers
import itertools
import bisect

class Solver:
    def parse(self, f: io.TextIOBase):
        self.tiles = [tuple(numbers(l)) for l in f]

    def part1(self):
        return max((abs(a[0]-b[0])+1)*(abs(a[1]-b[1])+1) for a, b in itertools.combinations(self.tiles, 2))

    def part2(self):
        xs = set()
        ys = set()
        for x, y in self.tiles:
            xs.add(x)
            xs.add(x+1)
            ys.add(y)
            ys.add(y+1)
        xs = sorted(xs)
        ys = sorted(ys)

        grid = [['.'] * (len(ys)+1) for _ in range(0, len(xs)+1)]
        for x, y in self.tiles:
            xi = bisect.bisect_right(xs, x)
            yi = bisect.bisect_right(ys, y)
            grid[xi][yi] = '#'
        self.tiles.append(self.tiles[0])
        for (x1, y1), (x2, y2) in itertools.pairwise(self.tiles):
            xi1 = bisect.bisect_right(xs, x1)
            yi1 = bisect.bisect_right(ys, y1)
            xi2 = bisect.bisect_right(xs, x2)
            yi2 = bisect.bisect_right(ys, y2)
            if xi2 < xi1:
                xi1, xi2 = xi2, xi1
            if yi2 < yi1:
                yi1, yi2 = yi2, yi1
            for x in range(xi1, xi2):
                if grid[x][yi1] == '.':
                    grid[x][yi1] = 'X'
            for y in range(yi1, yi2):
                if grid[xi1][y] == '.':
                    grid[xi1][y] = 'X'
        self.tiles.pop()
        q = [(0, 0)]
        grid[0][0] = '-'
        while q:
            x, y = q.pop(0)
            for xx in range(x-1, x+2):
                if xx < 0:
                    continue
                if xx > len(xs):
                    continue
                for yy in range(y-1, y+2):
                    if yy < 0:
                        continue
                    if yy > len(ys):
                        continue
                    if grid[xx][yy] == '.':
                        grid[xx][yy] = '-'
                        q.append((xx, yy))
        def good(x1, y1, x2, y2):
            xi1 = bisect.bisect_right(xs, x1)
            yi1 = bisect.bisect_right(ys, y1)
            xi2 = bisect.bisect_right(xs, x2)
            yi2 = bisect.bisect_right(ys, y2)
            if xi2 < xi1:
                xi1, xi2 = xi2, xi1
            if yi2 < yi1:
                yi1, yi2 = yi2, yi1
            for x in range(xi1, xi2+1):
                for y in range(yi1, yi2+1):
                    if grid[x][y] == '-':
                        return False
            return True
        maxarea = 0
        for (x1, y1), (x2, y2) in itertools.combinations(self.tiles, 2):
            if good(x1, y1, x2, y2):
                maxarea = max([maxarea, (abs(x1-x2)+1)*(abs(y1-y2)+1)])
        return maxarea
