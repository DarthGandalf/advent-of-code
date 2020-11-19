defmodule Day5 do
  @behaviour Day

  defp parse(input) do
    input
    |> String.split("\n")
    |> Stream.filter(fn s -> s != "" end)
    |> Stream.map(&Integer.parse/1)
    |> Stream.map(fn {a, ""} -> a end)
    |> Enum.to_list
  end

  defp solve1(mem, pos, steps) do
    if tuple_size(mem) <= pos do
      steps
    else
      off = elem(mem, pos)
      mem = put_elem(mem, pos, off + 1)
      pos = pos + off
      solve1(mem, pos, steps + 1)
    end
  end

  def part1(input) do
    mem = input |> parse
    solve1(List.to_tuple(mem), 0, 0)
  end

  def part12(input) do
    mem = input |> parse |> List.to_tuple
    size = tuple_size(mem)
    {:ok, mem} = Agent.start_link(fn -> mem end)
    {:ok, pos} = Agent.start_link(fn -> 0 end)
    hd(Stream.iterate(0, &(&1+1))
    |> Stream.filter(fn _step ->
      current_pos = Agent.get(pos, fn pos -> pos end)
      if size <= current_pos do
        true
      else
        off = Agent.get(mem, fn mem -> elem(mem, current_pos) end)
        Agent.update(mem, fn mem -> put_elem(mem, current_pos, off + 1) end)
        Agent.update(pos, fn _ -> current_pos + off end)
        false
      end
    end)
    |> Enum.take(1))
  end

  @impl Day
  def part2(input) do
  end
end
