from aoc2016.day17 import Solver
from io import StringIO

def test_example():
    solver = Solver()
    solver.parse(StringIO("ihgpwlah"))
    assert solver.part1() == "DDRRRD"
    assert solver.part2() == 370

    solver.parse(StringIO("kglvqrro"))
    assert solver.part1() == "DDUDRLRRUDRD"
    assert solver.part2() == 492

    solver.parse(StringIO("ulqzkmiv"))
    assert solver.part1() == "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
    assert solver.part2() == 830
