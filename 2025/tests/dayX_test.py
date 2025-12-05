from aoc2025.day5 import Solver
from io import StringIO
from textwrap import dedent

def test_example():
    return
    solver = Solver()
    solver.parse(StringIO(dedent("""
        .
    """).strip()))
    assert solver.part1() == 13
    assert solver.part2() == 43
