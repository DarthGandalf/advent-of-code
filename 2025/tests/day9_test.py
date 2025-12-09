from aoc2025.day9 import Solver
from io import StringIO
from textwrap import dedent

def test_example():
    solver = Solver()
    solver.parse(StringIO(dedent("""
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    """).strip()))
    assert solver.part1() == 50
    assert solver.part2() == 24
    assert False
