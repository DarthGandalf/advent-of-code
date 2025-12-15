import io
import hashlib
import dataclasses

@dataclasses.dataclass
class Coord:
    x: int
    y: int
    prefix: str

class Solver:
    def parse(self, f: io.TextIOBase):
        self.passcode = f.read().strip()

    def part1(self):
        q = [Coord(x=0, y=0, prefix="")]
        while q:
            c = q.pop(0)
            if c.x == 3 and c.y == 3:
                return c.prefix
            h = hashlib.md5((self.passcode + c.prefix).encode()).hexdigest()
            if c.y > 0 and h[0] > 'a':
                q.append(Coord(x=c.x, y=c.y-1, prefix=c.prefix+'U'))
            if c.y < 3 and h[1] > 'a':
                q.append(Coord(x=c.x, y=c.y+1, prefix=c.prefix+'D'))
            if c.x > 0 and h[2] > 'a':
                q.append(Coord(x=c.x-1, y=c.y, prefix=c.prefix+'L'))
            if c.x < 3 and h[3] > 'a':
                q.append(Coord(x=c.x+1, y=c.y, prefix=c.prefix+'R'))
        raise Exception("not found")

    def part2(self):
        q = [Coord(x=0, y=0, prefix="")]
        result = 0
        while q:
            c = q.pop(0)
            if c.x == 3 and c.y == 3:
                result = len(c.prefix)
                continue
            h = hashlib.md5((self.passcode + c.prefix).encode()).hexdigest()
            if c.y > 0 and h[0] > 'a':
                q.append(Coord(x=c.x, y=c.y-1, prefix=c.prefix+'U'))
            if c.y < 3 and h[1] > 'a':
                q.append(Coord(x=c.x, y=c.y+1, prefix=c.prefix+'D'))
            if c.x > 0 and h[2] > 'a':
                q.append(Coord(x=c.x-1, y=c.y, prefix=c.prefix+'L'))
            if c.x < 3 and h[3] > 'a':
                q.append(Coord(x=c.x+1, y=c.y, prefix=c.prefix+'R'))
        return result
