from aoc2016.day14 import Solver
from io import StringIO

def test_example():
    solver = Solver()
    solver.parse(StringIO("abc"))
    assert solver.part1() == 22728
    assert solver.part2() == 22551
