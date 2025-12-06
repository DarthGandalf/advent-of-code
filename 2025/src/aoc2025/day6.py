import io
import math
import itertools

class Solver:
    def parse(self, f: io.TextIOBase):
        self.lines = f.readlines()

    def part1(self):
        result = 0
        for x in zip(*(l.split() for l in self.lines)):
            x = list(x)
            op = x.pop()
            x = [int(y) for y in x]
            if op == '+':
                result += sum(x)
            else:
                result += math.prod(x)
        return result

    def part2(self):
        result = 0
        op = None
        nums = []
        def calc():
            if op == '+':
                return sum(nums)
            else:
                return math.prod(nums)
        for x in itertools.zip_longest(*(list(l) for l in self.lines)):
            if set(x) == set([' ']):
                result += calc()
                op = None
                nums = []
                continue
            if x[-1] is not None and x[-1] != ' ':
                op = x[-1]
            s = ''.join(x[:-1]).strip()
            if s:
                nums.append(int(''.join(x[:-1])))
        result += calc()
        return result
