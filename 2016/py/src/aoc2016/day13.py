import io
import functools
from collections import deque

class Solver:
    def parse(self, f: io.TextIOBase):
        self.officer = int(f.read())

    @functools.cache
    def open(self, x, y):
        z = x*x + 3*x + 2*x*y + y + y*y
        z += self.officer
        return f'{z:b}'.count('1') % 2 == 0

    def part1(self, tgtX=31, tgtY=39):
        q = deque([(1, 1, 1)])
        visited = {(1, 1)}
        while q:
            x, y, dist = q.popleft()
            for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                nx = x + dx
                ny = y + dy
                if nx < 0:
                    continue
                if ny < 0:
                    continue
                if not self.open(nx, ny):
                    continue
                if nx == tgtX and ny == tgtY:
                    return dist
                if (nx, ny) not in visited:
                    q.append((nx, ny, dist + 1))
                    visited.add((nx, ny))
        raise Exception("not found")

    def part2(self):
        q = deque([(1, 1, 1)])
        visited = {(1, 1)}
        while q:
            x, y, dist = q.popleft()
            for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                nx = x + dx
                ny = y + dy
                if nx < 0:
                    continue
                if ny < 0:
                    continue
                if not self.open(nx, ny):
                    continue
                if dist > 50:
                    continue
                if (nx, ny) not in visited:
                    q.append((nx, ny, dist + 1))
                    visited.add((nx, ny))
        return len(visited)
