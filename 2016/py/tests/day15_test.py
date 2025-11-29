from aoc2016.day15 import Solver
from io import StringIO
import textwrap

def test_example():
    solver = Solver()
    solver.parse(StringIO(textwrap.dedent('''
        Disc #1 has 5 positions; at time=0, it is at position 4.
        Disc #2 has 2 positions; at time=0, it is at position 1.
    ''').strip()))
    assert solver.part1() == 5
