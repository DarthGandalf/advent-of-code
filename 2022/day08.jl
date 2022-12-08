using Test
using Pkg
Pkg.activate(".")
using Chain
include("utils.jl")

real_input = readchomp("input/2022/day8.txt")
test_input = raw"""
30373
25512
65332
33549
35390"""

function parse(input)
    mapreduce(vcat, split(input, '\n')) do l
        @chain l begin
            split(_, "")
            Base.parse.(Int8, _)
            _'
        end
    end
end

function part1(input)
    m = parse(input)
    v = falses(axes(m))
    function fillrow(row, vrow)
        current_h = -1
        for i in axes(row, 1)
            h = row[i]
            if h > current_h
                vrow[i] = true
                current_h = h
            end
        end
    end
    function fill_row(how)
        # 5.05 k allocations: 1.202 MiB
        #fillrow(((m, v) .|> how)...)
        # 3.27 k allocations: 1.121 MiB
        fillrow(how.((m, v))...)
    end
    for col in axes(m, 2)
        # down
        fill_row(x -> view(x, axes(x, 1), col))
        # up
        fill_row(x -> view(x, reverse(axes(x, 1)), col))
    end
    for row in axes(m, 1)
        # right
        fill_row(x -> view(x, row, axes(x, 2)))
        # left
        fill_row(x -> view(x, row, reverse(axes(x, 2))))
    end
    sum(v)
end

solve(part1, real_input) do f
    @test f(test_input) == 21
end

# How to make it more Julia?
function part2(input)
    m = parse(input)
    r = -1
    function score(row, col)
        function len(slice)
            h = first(slice)
            for (i, x) in (Iterators.drop(slice, 1) |> enumerate)
                if x ≥ h
                    return i
                end
            end
            return length(slice) - 1
        end
        len(m[row, col:end]) * len(m[row, reverse(1:col)]) * len(m[row:end, col]) * len(m[reverse(1:row), col])
    end
    for row in axes(m, 1)
        for col in axes(m, 2)
            s = score(row, col)
            r = max(r, s)
        end
    end
    r
end

solve(part2, real_input) do f
    @test f(test_input) == 8
end
