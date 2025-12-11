import io
from collections import defaultdict

class Solver:
    def parse(self, f: io.TextIOBase):
        self.adj = dict()
        self.rev = defaultdict(list)
        for l in f:
            l = l.strip().split(' ')
            src = l[0].strip(':')
            self.adj[src] = l[1:]
            for r in l[1:]:
                self.rev[r].append(src)

    def sort(self):
        topo = []
        vis = set()
        def dfs(node):
            vis.add(node)
            for neigh in self.adj.get(node, []):
                if neigh not in vis:
                    dfs(neigh)
            topo.append(node)
        if "svr" in self.adj:
            dfs("svr")
        dfs("you")
        return topo

    def get(self, topo, src, tgt):
        nums = {src: 1}
        for t in reversed(topo):
            if t != src:
                nums[t] = sum(nums.get(neigh, 0) for neigh in self.rev[t])
        return nums[tgt]

    def part1(self):
        topo = self.sort()
        return self.get(topo, "you", "out")

    def part2(self):
        topo = self.sort()
        if topo.index("fft") > topo.index("dac"):
            return self.get(topo, "svr", "fft") * self.get(topo, "fft", "dac") * self.get(topo, "dac", "out")
        else:
            return self.get(topo, "svr", "dac") * self.get(topo, "dac", "fft") * self.get(topo, "fft", "out")
