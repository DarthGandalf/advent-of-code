using Test
include("utils.jl")

real_input = readchomp("input/2022/day4.txt")
test_input = """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""

within(a, b, c, d) = let x = a:b, y = c:d
    x ⊆ y || y ⊆ x
end
overlap(a, b, c, d) = let x = a:b, y = c:d
    !isempty(x ∩ y)
end

part1(input) = eachsplit(input, '\n') .|> ints .|> (x->within(x...)) |> sum
part2(input) = eachsplit(input, '\n') .|> ints .|> (x->overlap(x...)) |> sum

solve(part1, real_input) do f
    @test f(test_input) == 2
end
solve(part2, real_input) do f
    @test f(test_input) == 4
end
