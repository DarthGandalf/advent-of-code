defmodule Day2 do
  @behaviour Day

  def parse(input) do
    input
    |> String.split("\n")
    |> Stream.map(fn line ->
      line
      |> String.split(~r{\s}, trim: true)
      |> Stream.map(&Integer.parse/1)
      |> Stream.map(fn {a, ""} -> a end)
      |> Enum.to_list()
    end)
    |> Stream.filter(fn line -> line != [] end)
  end

  @impl Day
  def part1(input) do
    input
    |> parse
    |> Stream.map(fn line ->
      {min, max} = line |> Enum.min_max()
      max - min
    end)
    |> Enum.reduce(0, &+/2)
  end

  @impl Day
  def part2(input) do
    input
    |> parse
    |> Enum.flat_map(fn line ->
      for a <- line, b <- line, a != b and rem(a, b) == 0, do: div(a, b)
    end)
    |> Enum.reduce(0, &+/2)
  end
end
