import io
from aoc2025.utils import numbers
import dataclasses
import z3

@dataclasses.dataclass
class Machine:
    target: int
    buttons: list[int]
    bs: list[list[int]]
    maxmask: int
    joltage: tuple

    def part1(self):
        if self.target == 0:
            return 0
        visited = {0}
        q = [(0, 1)]
        while q:
            state, depth = q.pop(0)
            for b in self.buttons:
                newstate = (state ^ b) & self.maxmask
                if newstate == self.target:
                    return depth
                if newstate not in visited:
                    visited.add(newstate)
                    q.append((newstate, depth+1))
        raise Exception("no way")

    def part2(self):
        s = z3.Optimize()
        su = z3.Int("su")
        variables = []
        for b, _ in enumerate(self.bs):
            v = z3.Int(f"b{b}")
            variables.append(v)
            s.add(v >= 0)
        s.add(sum(variables) == su)
        for i, j in enumerate(self.joltage):
            bs = []
            for bb, b in enumerate(self.bs):
                if i in b:
                    bs.append(variables[bb])
            s.add(sum(bs) == j)
        s.minimize(su)
        s.check()
        m = s.model()
        return m[su].as_long()

class Solver:
    def parse(self, f: io.TextIOBase):
        self.machines: list[Machine] = []
        for line in f:
            groups = line.split(' ')
            target = groups[0][1:-1]
            maxmask = (1 << len(target)) - 1
            target = int(''.join(reversed(target.replace('#', '1').replace('.', '0'))), base=2)
            joltage = tuple(numbers(groups[-1]))
            bs = [sorted(set(numbers(g))) for g in groups[1:-1]]
            buttons = []
            for b in bs:
                nb = 0
                for bb in b:
                    nb += 1 << bb
                buttons.append(nb)
            self.machines.append(Machine(target=target, buttons=buttons, bs=bs, joltage=joltage, maxmask=maxmask))

    def part1(self):
        return sum(m.part1() for m in self.machines)

    def part2(self):
        return sum(m.part2() for m in self.machines)

