using Test

real_input = readchomp("input/2022/day4.txt")
test_input = """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""

ints(str) = parse.(Int, eachmatch(r"(\d+)", str) .|> only)

within(a, b, c, d) = let x = a:b, y = c:d
    x ⊆ y || y ⊆ x
end
overlap(a, b, c, d) = let x = a:b, y = c:d
    !isempty(x ∩ y)
end

part1(input) = eachsplit(input, '\n') .|> ints .|> (x->within(x...)) |> sum
part2(input) = eachsplit(input, '\n') .|> ints .|> (x->overlap(x...)) |> sum

print("Part 1... ")
@test part1(test_input) == 2
println("ok")
@time result = part1(real_input)
println("Result: ", result)

print("Part 2... ")
@test part2(test_input) == 4
println("ok")
@time result = part2(real_input)
println("Result: ", result)
