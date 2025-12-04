import io
import numpy as np

class Solver:
    def parse(self, f: io.TextIOBase):
        self.banks = [[int(c) for c in line.strip()] for line in f.readlines()]

    def part1(self):
        s = 0
        for bank in self.banks:
            first = np.argmax(bank[:-1])
            second = np.argmax(bank[first+1:])
            s += bank[first] * 10 + bank[first+second+1]
        return s

    def part2(self):
        s = 0
        for bank in self.banks:
            subbank = bank[:]
            number = 0
            for i in range(12, 0, -1):
                if i == 1:
                    subset = subbank
                else:
                    subset = subbank[:-i+1]
                where = np.argmax(subset)
                number = number * 10 + subbank[where]
                subbank = subbank[where+1:]
            s += number
        return s
