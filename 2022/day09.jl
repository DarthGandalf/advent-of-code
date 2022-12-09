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

function part1(input)
    was = Set([(0, 0)])
    tx = 0
    ty = 0
    hx = 0
    hy = 0
    function adjust_tail()
        if abs(hx - tx) <= 1 && abs(hy - ty) <= 1
            return
        elseif hx == tx || hy == ty
            if hx == tx
                if hy > ty + 1
                    ty += 1
                elseif hy < ty - 1
                    ty -= 1
                else
                    error("assert 1")
                end
            else
                if hx > tx + 1
                    tx += 1
                elseif hx < tx - 1
                    tx -= 1
                else
                    error("assert 2")
                end
            end
        else
            # move diagonally
            if hx > tx
                tx += 1
            end
            if hx < tx
                tx -= 1
            end
            if hy > ty
                ty += 1
            end
            if hy < ty
                ty -= 1
            end
        end
        if abs(hx - tx) > 1 || abs(hy - ty) > 1
            error("assert 3")
        end
        push!(was, (tx, ty))
    end
    for line in eachsplit(input, '\n')
        dir, amounts = split(line, ' ')
        amount = parse(Int, amounts)
        for i=1:amount
            if dir == "R"
                hx += 1
            elseif dir == "L"
                hx -= 1
            elseif dir == "D"
                hy += 1
            elseif dir == "U"
                hy -= 1
            else
                error("wrong dir " * dir)
            end
            adjust_tail()
        end
    end
    length(was)
end

solve(part1, real_input) do f
    @test f(test_input) == 13
end
