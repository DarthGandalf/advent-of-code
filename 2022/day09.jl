using Test
using Pkg
Pkg.activate(".")
using Chain
include("utils.jl")

real_input = readchomp("input/2022/day9.txt")
test_input = raw"""
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""

mutable struct Knot
    x::Int
    y::Int
end
function adjust_tail(h, t)
    if abs(h.x - t.x) <= 1 && abs(h.y - t.y) <= 1
        return
    elseif h.x == t.x || h.y == t.y
        if h.x == t.x
            if h.y > t.y + 1
                t.y += 1
            elseif h.y < t.y - 1
                t.y -= 1
            else
                error("assert 1")
            end
        else
            if h.x > t.x + 1
                t.x += 1
            elseif h.x < t.x - 1
                t.x -= 1
            else
                error("assert 2")
            end
        end
    else
        # move diagonally
        if h.x > t.x
            t.x += 1
        end
        if h.x < t.x
            t.x -= 1
        end
        if h.y > t.y
            t.y += 1
        end
        if h.y < t.y
            t.y -= 1
        end
    end
    if abs(h.x - t.x) > 1 || abs(h.y - t.y) > 1
        error("assert 3")
    end
end

function part(input, L)
    k = [Knot(0, 0) for i=1:L]
    was = Set([(0, 0)])
    for line in eachsplit(input, '\n')
        dir, amounts = split(line, ' ')
        amount = parse(Int, amounts)
        for i=1:amount
            h = k[1]
            if dir == "R"
                h.x += 1
            elseif dir == "L"
                h.x -= 1
            elseif dir == "D"
                h.y += 1
            elseif dir == "U"
                h.y -= 1
            else
                error("wrong dir " * dir)
            end
            for j=2:L
                adjust_tail(k[j-1], k[j])
            end
            push!(was, (k[end].x, k[end].y))
        end
    end
    length(was)
end

part1(input) = part(input, 2)
part2(input) = part(input, 10)

solve(part1, real_input) do f
    @test f(test_input) == 13
end

solve(part2, real_input) do f
    @test f(test_input) == 1
    @test f("""
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20""") == 36
end
