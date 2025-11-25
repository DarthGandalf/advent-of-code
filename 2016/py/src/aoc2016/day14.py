import io
import functools
from hashlib import md5
import itertools
import re
from typing import Callable

TRIPLE = re.compile(r'(.)\1\1')

class Solver:
    def parse(self, f: io.TextIOBase):
        self.salt = f.read().strip()

    @functools.cache
    def hash(self, index):
        return md5(data=f"{self.salt}{index}".encode()).hexdigest()

    @functools.cache
    def hash2(self, index):
        h = self.hash(index)
        for _ in range(0, 2016):
            h = md5(h.encode()).hexdigest()
        return h

    def partX(self, hash: Callable):
        count = 0
        for i in itertools.count(0):
            if matches := TRIPLE.search(hash(i)):
                search = matches.group(1) * 5
                good = any(hash(j).find(search) != -1 for j in range(i+1, i+1001))
                if good:
                    count += 1
                    if count == 64:
                        return i

    def part1(self):
        return self.partX(self.hash)
    
    def part2(self):
        return self.partX(self.hash2)
