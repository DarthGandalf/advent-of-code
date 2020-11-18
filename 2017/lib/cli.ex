defmodule Day do
  @callback part1(String.t) :: term
  @callback part2(String.t) :: term
end

defmodule CLI do
  def main(args \\ []) do
    {out1, out2, time1, time2} = solve(args)

    IO.puts("part1: " <> to_string(out1))
    IO.puts("took " <> to_string(time1) <> "ms")
    IO.puts("part2: " <> to_string(out2))
    IO.puts("took " <> to_string(time2) <> "ms")
  end

  def solve(args \\ []) do
    day = case args do
      [day] -> day
      _ -> raise("Usage: ./aoc2017 <day>")
    end
    input = File.read!("input/2017/day" <> day <> ".txt")
    {day, ""} = Integer.parse(day)

    day = case day do
      1 -> Day1
      2 -> Day2
    end

    {time1, out1} = :timer.tc(day, :part1, [input])
    {time2, out2} = :timer.tc(day, :part2, [input])

    {out1, out2, time1/1000, time2/1000}
  end

  def solve_test(args \\ []) do
    {out1, out2, _, _} = solve(args)
    {out1, out2}
  end
end
