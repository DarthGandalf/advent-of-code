from aoc2025.day1 import Solver
from io import StringIO

def test_example():
    solver = Solver()
    solver.parse(StringIO(""))
    assert solver.part1() is None
