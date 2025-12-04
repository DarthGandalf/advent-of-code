import io

class Solver:
    def parse(self, f: io.TextIOBase):
        self.lines = [['.'] + [c for c in l.strip()] + ['.'] for l in f.readlines()]
        self.lines.insert(0, ['.'] * len(self.lines[0]))
        self.lines.append(['.'] * len(self.lines[0]))

    def part1(self):
        count = 0
        for i, line in enumerate(self.lines):
            for j, c in enumerate(line):
                if c == '.':
                    continue
                subcount = 0
                for ni in (i-1, i, i+1):
                    for nj in (j-1, j, j+1):
                        if self.lines[ni][nj] == '@':
                            subcount += 1
                if subcount <= 4:
                    count += 1
        return count

    def part2(self):
        count = 0
        oldcount = -1
        while count is not oldcount:
            oldcount = count
            for i, line in enumerate(self.lines):
                for j, c in enumerate(line):
                    if c != '@':
                        continue
                    subcount = 0
                    for ni in (i-1, i, i+1):
                        for nj in (j-1, j, j+1):
                            if self.lines[ni][nj] == '@':
                                subcount += 1
                    if subcount <= 4:
                        self.lines[i][j] = 'x'
                        count += 1
        return count
