from aoc2016.day14 import Solver
from io import StringIO
import pytest

@pytest.mark.skip(reason="slow")
def test_example():
    solver = Solver()
    solver.parse(StringIO("abc"))
    assert solver.part1() == 22728
    assert solver.part2() == 22551
