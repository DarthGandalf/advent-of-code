defmodule Day5Test do
  use ExUnit.Case
  doctest Day5

  test "part 1" do
    assert Day5.part1("0
3
0
1
-3") == 5
  end

  test "part 2" do
    assert Day5.part2("0
3
0
1
-3") == 3333
  end

  test "result" do
    #assert CLI.solve_test(["5"]) == {373543, 0}
  end
end
