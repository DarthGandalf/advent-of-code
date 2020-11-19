defmodule Day3 do
  @behaviour Day

  defp parse(input) do
    {num, _} = input |> Integer.parse
    num
  end

  defp coords(1) do
    {0, 0}
  end

  defp coords(num) do
    square = num |> :math.sqrt |> :math.ceil
    radius = round(:math.ceil((square - 1) / 2))
    edge = 2 * radius
    previous = (edge-1)*(edge-1)
    {x, y} = case num do
      z when z in previous+1..previous+edge -> {radius, z-previous-radius}
      z when z in previous+1..previous+2*edge -> {previous+edge-z+radius, radius}
      z when z in previous+1..previous+3*edge -> {-radius, previous-z+2*edge+radius}
      z -> {z-previous-3*edge-radius, -radius}
    end
    {x, y}
  end

  @impl Day
  def part1(input) do
    num = input |> parse
    {x, y} = coords(num)
    abs(x) + abs(y)
  end

  @impl Day
  def part2(input) do
    input = input |> parse
    {:ok, m} = Agent.start_link(fn -> %{{0, 0} => 1} end)
    Stream.iterate(2, &(&1+1))
    |> Stream.map(fn num ->
      {x, y} = coords(num)
      new = [
        Agent.get(m, fn m -> Map.get(m, {x-1, y}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x+1, y}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x, y-1}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x, y+1}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x-1, y-1}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x-1, y+1}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x+1, y-1}, 0) end),
        Agent.get(m, fn m -> Map.get(m, {x+1, y+1}, 0) end),
      ] |> Enum.reduce(0, &+/2)
      Agent.update(m, fn m -> Map.put(m, {x, y}, new) end)
      new
    end)
    |> Enum.find(fn n -> n > input end)
  end
end
