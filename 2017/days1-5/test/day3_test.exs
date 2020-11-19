defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  test "part 1" do
    assert Day3.part1("1") == 0
    assert Day3.part1("12") == 3
    assert Day3.part1("23") == 2
    assert Day3.part1("1024") == 31
  end

  test "result" do
    assert CLI.solve_test(["3"]) == {552, 330785}
  end
end
