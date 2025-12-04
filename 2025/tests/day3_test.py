from aoc2025.day3 import Solver
from io import StringIO
from textwrap import dedent

def test_example():
    solver = Solver()
    solver.parse(StringIO(dedent("""
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    """).strip()))
    assert solver.part1() == 357
    assert solver.part2() == 3121910778619
