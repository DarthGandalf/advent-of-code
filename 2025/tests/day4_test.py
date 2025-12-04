from aoc2025.day4 import Solver
from io import StringIO
from textwrap import dedent

def test_example():
    solver = Solver()
    solver.parse(StringIO(dedent("""
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    """).strip()))
    assert solver.part1() == 13
    assert solver.part2() == 43
