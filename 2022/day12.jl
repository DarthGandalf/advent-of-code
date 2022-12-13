using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using Chain

real_input = readchomp("input/2022/day12.txt")
test_input = raw"""
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""

mutable struct Field
    height::Int8
    distance::Int
end

function parse_input(input)
    start = CartesianIndex[]
    finish = CartesianIndex[]
    ifstart(r, (c, ch)) = if ch == 'S'
        push!(start, CartesianIndex(r, c))
        'a'
    else
        ch
    end
    iffinish(r, (c, ch)) = if ch == 'E'
        push!(finish, CartesianIndex(r, c))
        'z'
    else
        ch
    end
    height(c) = c - 'a'
    charify(s) = s[1]
    M = mapreduce(vcat, split(input, '\n') |> enumerate) do (r, l)
        @chain l begin
            split(_, "")
            @. charify
            enumerate
            ifstart.(r, _)
            enumerate
            iffinish.(r, _)
            @. height
            _'
        end
    end .|> (x -> Field(x, -1))
    (M, start |> only, finish |> only)
end

function bfs(M, start, condition)
    M[start].distance = 0
    Q = CartesianIndex[start]
    function attempt(prev, pos)
        if checkbounds(Bool, M, pos)
            if M[pos].distance == -1
                if condition(pos, prev)
                    M[pos].distance = M[prev].distance + 1
                    push!(Q, pos)
                end
            end
        end
    end
    while !isempty(Q)
        current = popfirst!(Q)
        attempt(current, current + CartesianIndex(1, 0))
        attempt(current, current - CartesianIndex(1, 0))
        attempt(current, current + CartesianIndex(0, 1))
        attempt(current, current - CartesianIndex(0, 1))
    end
end

function part1(input)
    M, start, finish = parse_input(input)
    bfs(M, start, (a, b)->M[a].height ≤ M[b].height + 1)
    M[finish].distance
end
solve(part1, real_input) do f
    @test f(test_input) == 31
end

function part2(input)
    M, _, finish = parse_input(input)
    bfs(M, finish, (a, b)->M[b].height ≤ M[a].height + 1)
    @chain M begin
        Iterators.filter(x->x.height==0, _)
        (x->x.distance).(_)
        Iterators.filter(x->x≥0, _)
        minimum
    end
end
solve(part2, real_input) do f
    @test f(test_input) == 29
end
