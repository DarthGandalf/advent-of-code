import io
import functools
import itertools
import re
from typing import Callable
import dataclasses
from aoc2016.utils import numbers

@dataclasses.dataclass
class Disc:
    length: int
    position: int

def inverse(a: int, n: int):
    '''
    https://ru.wikipedia.org/wiki/%D0%9A%D0%B8%D1%82%D0%B0%D0%B9%D1%81%D0%BA%D0%B0%D1%8F_%D1%82%D0%B5%D0%BE%D1%80%D0%B5%D0%BC%D0%B0_%D0%BE%D0%B1_%D0%BE%D1%81%D1%82%D0%B0%D1%82%D0%BA%D0%B0%D1%85#%D0%90%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC%D1%8B_%D0%BF%D0%BE%D0%B8%D1%81%D0%BA%D0%B0_%D1%80%D0%B5%D1%88%D0%B5%D0%BD%D0%B8%D0%B9
    https://ru.wikipedia.org/wiki/%D0%A0%D0%B0%D1%81%D1%88%D0%B8%D1%80%D0%B5%D0%BD%D0%BD%D1%8B%D0%B9_%D0%B0%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC_%D0%95%D0%B2%D0%BA%D0%BB%D0%B8%D0%B4%D0%B0#%D0%92%D1%8B%D1%87%D0%B8%D1%81%D0%BB%D0%B5%D0%BD%D0%B8%D0%B5_%D0%BE%D0%B1%D1%80%D0%B0%D1%82%D0%BD%D0%BE%D0%B3%D0%BE_%D0%B2_%D0%BC%D0%BE%D0%B4%D1%83%D0%BB%D1%8F%D1%80%D0%BD%D1%8B%D1%85_%D1%81%D1%82%D1%80%D1%83%D0%BA%D1%82%D1%83%D1%80%D0%B0%D1%85
    '''
    t = 0
    newt = 1
    r = n
    newr = a
    while newr != 0:
        q = r // newr
        t, newt = newt, t - q * newt
        r, newr = newr, r - q * newr

    if r > 1:
        raise Exception(f"{a} not inversible mod {n}")
    if t < 0:
        t += n
    return t

class Solver:
    def parse(self, f: io.TextIOBase):
        self.discs: list[Disc] = []
        for line in f:
            ns = numbers(line)
            self.discs.append(Disc(ns[1], ns[3]))

    def part1(self):
        m = 1
        for a in self.discs:
            m *= a.length
        mi = []
        mii =[]
        for a in self.discs:
            mi.append(m // a.length)
            mii.append(inverse(mi[-1], a.length))
        x = 0
        for i, a in enumerate(self.discs):
            r = (-a.position - i - 1) % m
            x += (r * mi[i] * mii[i]) % m
            pass
        x %= m
        return x

    def part2(self):
        self.discs.append(Disc(11, 0))
        return self.part1()
