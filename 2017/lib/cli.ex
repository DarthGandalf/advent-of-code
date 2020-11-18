defmodule Day do
  @callback part1(String.t) :: term
  @callback part2(String.t) :: term
end

defmodule CLI do
  def main(args \\ []) do
    {out1, out2} = solve(args)

    IO.puts("part1: " <> to_string(out1))
    IO.puts("part2: " <> to_string(out2))
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

    out1 = day.part1(input)
    out2 = day.part2(input)

    {out1, out2}
  end
end
