defmodule CLI do
  def main(args \\ []) do
    day = case args do
      [day] -> day
      _ -> raise("Usage: ./aoc2017 <day>")
    end
    input = File.read!("input/2017/day" <> day <> ".txt")
    {day, ""} = Integer.parse(day)

    {part1, part2} = case day do
      1 -> {&Day1.part1/1, &Day1.part2/1}
      2 -> {&Day2.part1/1, &Day2.part2/1}
    end

    IO.puts("part1: " <> to_string(part1.(input)))
    IO.puts("part2: " <> to_string(part2.(input)))
  end
end
