from aoc2025.day6 import Solver
from io import StringIO
from textwrap import dedent

def test_example():
    solver = Solver()
    solver.parse(StringIO(dedent("""
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   + 
    """).strip()))
    assert solver.part1() == 4277556
    assert solver.part2() == 3263827
