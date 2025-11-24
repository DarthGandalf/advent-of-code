from aoc2016.day13 import Solver
from io import StringIO

def test_example():
    solver = Solver()
    solver.parse(StringIO("10"))
    assert solver.part1(7, 4) == 11
