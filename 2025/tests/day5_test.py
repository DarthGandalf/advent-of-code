from aoc2025.day5 import Solver
from io import StringIO
from textwrap import dedent

def test_example():
    solver = Solver()
    solver.parse(StringIO(dedent("""
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    """).strip()))
    assert solver.part1() == 3
    assert solver.part2() == 14
