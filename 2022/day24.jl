using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using DataStructures

real_input = readchomp("input/2022/day24.txt")
test_input = raw"""
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"""

struct Input
    width::Int
    height::Int
    right::BitArray
    left::BitArray
    up::BitArray
    down::BitArray
end

function parse_input(input)
    lines = split(input, '\n')
    height = length(lines) - 2
    width = length(lines[1]) - 2
    right = falses(height, width)
    left = falses(height, width)
    up = falses(height, width)
    down = falses(height, width)
    for row in 1:height, col in 1:width
        ch = lines[row+1][col+1]
        if ch == '>'
            right[row, col] = true
        elseif ch == '<'
            left[row, col] = true
        elseif ch == 'v'
            down[row, col] = true
        elseif ch == '^'
            up[row, col] = true
        end
    end
    Input(width, height, right, left, up, down)
end

struct MyVertex
    row::Int
    col::Int
    time::Int
end

const big = typemax(Int)÷2
function astar(input, start, destination)
    period = lcm(input.height, input.width)
    heur(v) = abs(destination.row - v.row) + abs(destination.col - v.col)
    Q = PriorityQueue()
    Q[start] = heur(start)
    dist = Dict(start => 0)
    neigh(v) = Iterators.filter([
                                 MyVertex(v.row, v.col, (v.time+1)%period),
                                 MyVertex(v.row+1, v.col, (v.time+1)%period),
                                 MyVertex(v.row-1, v.col, (v.time+1)%period),
                                 MyVertex(v.row, v.col+1, (v.time+1)%period),
                                 MyVertex(v.row, v.col-1, (v.time+1)%period),
                                ]) do u
        if u.row == start.row && u.col == start.col
            return true
        end
        if u.row == destination.row && u.col == destination.col
            return true
        end
        if u.row < 1 || u.col < 1 || u.row > input.height || u.col > input.width
            return false
        end
        if input.right[u.row, mod1((u.col-u.time+2*period), input.width)]
            return false
        end
        if input.left[u.row, mod1(u.col+u.time, input.width)]
            return false
        end
        if input.down[mod1(u.row-u.time+2*period, input.height), u.col]
            return false
        end
        if input.up[mod1(u.row+u.time, input.height), u.col]
            return false
        end
        return true
    end

    while !isempty(Q)
        current = dequeue!(Q)
        if current.row == destination.row && current.col == destination.col
            return current, dist[current]
        end
        d = dist[current] + 1
        for towards in neigh(current)
            if d < get(dist, towards, big)
                dist[towards] = d
                Q[towards] = d + heur(towards)
            end
        end
    end
    error("no way")
end

function part1(text)
    input = parse_input(text)
    start = MyVertex(0, 1, 0)
    destination = MyVertex(1 + input.height, input.width, 0)
    astar(input, start, destination)[2]
end
solve(part1, real_input) do f
    @test f(test_input) == 18
end

function part2(text)
    input = parse_input(text)
    start = MyVertex(0, 1, 0)
    destination = MyVertex(1 + input.height, input.width, 0)
    now, t1 = astar(input, start, destination)
    now, t2 = astar(input, now, start)
    _, t3 = astar(input, now, destination)
    t1 + t2 + t3
end
solve(part2, real_input) do f
    @test f(test_input) == 54
end
