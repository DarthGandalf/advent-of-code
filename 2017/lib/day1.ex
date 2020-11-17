defmodule Day1 do
  @moduledoc """
  Documentation for `Day1`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Day1.hello()
      :world

  """
  def hello do
    :world
  end

  def digits(input) do
    input |> Stream.unfold(&String.next_codepoint/1) |> Stream.map(fn <<x>> -> x-0x30 end) |> Stream.filter(fn x -> x >= 0 end) |> Enum.to_list()
  end

  def part1(input) do
    digits = digits(input)
    digits ++ [-1] |> Stream.zip([List.last(digits)] ++ digits) |> Stream.filter(fn {x, y} -> x == y end) |> Stream.map(fn {x, _} -> x end) |> Enum.reduce(0, &+/2)
  end

  def part2(input) do
    digits = digits(input)
    len = length(digits)
    digits |> Stream.zip(digits ++ digits |> Stream.drop(div(len, 2))) |> Stream.take(len) |> Stream.filter(fn {x, y} -> x == y end) |> Stream.map(fn {x, _} -> x end) |> Enum.reduce(0, &+/2)
  end
end
