using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using JSON

real_input = readchomp("input/2022/day13.txt")
test_input = raw"""
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""

myless(a::Int, b::Int) = isless(a, b)
myless(a::Vector, b::Int) = myless(a, [b])
myless(a::Int, b::Vector) = myless([a], b)
function myless(a::Vector, b::Vector)
    for (x, y) in zip(a, b)
        if myless(x, y)
            return true
        end
        if myless(y, x)
            return false
        end
    end
    myless(length(a), length(b))
end

function part1(input)
    sum = 0
    for (i, pair) in enumerate(eachsplit(input, "\n\n"))
        if cmp(myless, (split(pair, '\n') .|> JSON.parse)...) <= 0
            sum += i
        end
    end
    sum
end
solve(part1, real_input) do f
    @test f(test_input) == 13
end

function part2(input)
    v = [JSON.parse(line) for line in eachsplit(input, '\n') if !isempty(line)]
    push!(v, [[2]], [[6]])
    sort!(v, lt=myless)
    two = findfirst(==([[2]]), v)
    six = findfirst(==([[6]]), v)
    two * six
end
solve(part2, real_input) do f
    @test f(test_input) == 140
end
