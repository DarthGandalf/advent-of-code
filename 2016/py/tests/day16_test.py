from aoc2016.day16 import Solver
from io import StringIO
import textwrap

def test_example():
    solver = Solver()
    assert solver.checksum("110010110100") == "100"

    solver.parse(StringIO("10000"))
    assert solver.part1(20) == "01100"
