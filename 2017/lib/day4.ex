defmodule Day4 do
  @behaviour Day

  defp parse(input) do
    input
    |> String.split("\n")
    |> Stream.map(fn line ->
      line
      |> String.split(~r{\s}, trim: true)
      |> Enum.to_list()
    end)
    |> Stream.filter(fn line -> line != [] end)
  end

  @impl Day
  def part1(input) do
    input
    |> parse
    |> Enum.count(fn phrase ->
      phrase
      |> Enum.frequencies
      |> Enum.all?(fn
        {_, 1} -> true
        _ -> false
      end)
    end)
  end

  @impl Day
  def part2(input) do
    input
    |> parse
    |> Enum.count(fn phrase ->
      phrase
      |> Stream.map(fn word -> word |> to_charlist |> Enum.sort end)
      |> Enum.frequencies
      |> Enum.all?(fn
        {_, 1} -> true
        _ -> false
      end)
    end)
  end
end
