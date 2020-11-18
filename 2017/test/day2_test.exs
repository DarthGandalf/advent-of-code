defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "part 1" do
    assert Day2.part1("5 1 9 5
7 5 3
2 4 6 8") == 18
  end

  test "part 2" do
    assert Day2.part2("5 9 2 8
9 4 7 3
3 8 6 5") == 9
  end

  test "result" do
    assert CLI.solve(["2"]) == {47136, 250}
  end
end
