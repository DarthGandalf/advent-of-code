from aoc2025.day1 import Solver
from io import StringIO

def test_example():
    solver = Solver()
    solver.parse(StringIO("""L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"""))
    assert solver.part1() == 3
    assert solver.part2() == 6
