defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "greets the world" do
    assert Day1.hello() == :world
  end

  test "part 1" do
    assert Day1.part1("1122") == 3
    assert Day1.part1("1111") == 4
    assert Day1.part1("1234") == 0
    assert Day1.part1("91212129") == 9
  end

  test "part 2" do
    assert Day1.part2("1212") == 6
    assert Day1.part2("1221") == 0
    assert Day1.part2("123425") == 4
    assert Day1.part2("123123") == 12
    assert Day1.part2("12131415") == 4
  end

  test "result" do
    assert CLI.solve_test(["1"]) == {995, 1130}
  end
end
