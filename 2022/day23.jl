using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using DataStructures

real_input = readchomp("input/2022/day23.txt")
test_input = raw"""
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."""

function parse_input(input)
    elves = SortedSet()
    for (y, line) in enumerate(split(input, '\n'))
        for (x, ch) in enumerate(line)
            if ch == '#'
                push!(elves, (x, y))
            end
        end
    end
    elves
end

function round(elves, num)
    proposals = Dict()
    busy = DefaultDict(0)
    function create_consideration(offset, also)
        function consider_from(x, y)
            dest = (x, y) .+ offset
            still_good = true
            if dest ∈ elves
                still_good = false
            end
            for o in also
                if (x, y) .+ o ∈ elves
                    still_good = false
                end
            end
            if still_good
                proposals[(x, y)] = dest
                busy[dest] += 1
            end
            still_good
        end
    end
    considerations = [
        create_consideration(( 0, -1), [(-1, -1), (+1, -1)]),
        create_consideration(( 0, +1), [(-1, +1), (+1, +1)]),
        create_consideration((-1,  0), [(-1, +1), (-1, -1)]),
        create_consideration((+1,  0), [(+1, +1), (+1, -1)]),
    ]
    for (x, y) in elves
        need_move = false
        for dx in -1:1, dy in -1:1
            if dx != 0 || dy != 0
                if (x, y) .+ (dx, dy) ∈ elves
                    need_move = true
                end
            end
        end
        made_proposal = false
        if need_move
            for i in 0:3
                if !made_proposal
                    made_proposal = considerations[mod1(i + num, 4)](x, y)
                end
            end
        end
        if !made_proposal
            proposals[(x, y)] = (x, y)
            busy[(x, y)] += 1
        end
    end
    elves = SortedSet()
    for (e, p) in proposals
        if busy[p] > 1
            push!(elves, e)
        else
            push!(elves, p)
        end
    end
    elves
end

function E(elves)
    minx = minimum(e->e[1], elves)
    miny = minimum(e->e[2], elves)
    maxx = maximum(e->e[1], elves)
    maxy = maximum(e->e[2], elves)
    println("-------")
    for y in miny:maxy
        for x in minx:maxx
            if (x, y) ∈ elves
                print('#')
            else
                print('.')
            end
        end
        println()
    end
end

function part1(input)
    elves = parse_input(input)
    for i in 1:10
        elves = round(elves, i)
    end
    minx = minimum(e->e[1], elves)
    miny = minimum(e->e[2], elves)
    maxx = maximum(e->e[1], elves)
    maxy = maximum(e->e[2], elves)
    (maxx - minx + 1) * (maxy - miny + 1) - length(elves)
end
solve(part1, real_input) do f
    @test f(test_input) == 110
end

function part2(input)
    elves = parse_input(input)
    i = 0
    while true
        i += 1
        next_elves = round(elves, i)
        if elves == next_elves
            return i
        end
        elves = next_elves
    end
end
solve(part2, real_input) do f
    @test f(test_input) == 20
end

