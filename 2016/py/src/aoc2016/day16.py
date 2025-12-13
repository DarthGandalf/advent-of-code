import io
import itertools

class Solver:
    def dragon(self, text):
        out = [text, "0"]
        for c in reversed(text):
            out.append("1" if c == "0" else "0")
        return ''.join(out)

    def checksum(self, text):
        while len(text) % 2 == 0:
            out = []
            for x, y in itertools.batched(text, 2):
                if x == y:
                    out.append("1")
                else:
                    out.append("0")
            text = ''.join(out)
        return text

    def parse(self, f: io.TextIOBase):
        self.input = f.read().strip()

    def part1(self, length=272):
        text = self.input
        while len(text) < length:
            text = self.dragon(text)
        text = text[:length]
        return self.checksum(text)

    def part2(self):
        return self.part1(35651584)
