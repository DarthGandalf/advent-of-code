using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using DataStructures

real_input = readchomp("input/2022/day18.txt")
test_input = raw"""
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"""

function prepare_input(input)
    xx = DefaultDict(0)
    yy = DefaultDict(0)
    zz = DefaultDict(0)
    for (x, y, z) in (eachsplit(input, '\n') .|> ints)
        xx[(x, y, z)] += 1
        yy[(x, y, z)] += 1
        zz[(x, y, z)] += 1
        xx[(x+1, y, z)] += 1
        yy[(x, y+1, z)] += 1
        zz[(x, y, z+1)] += 1
    end
    Filter(tt) = Set(k for (k, v) in tt if v == 1)
    Filter.([xx, yy, zz])
end

function part1(input)
    xx, yy, zz = prepare_input(input)
    sum(length, [xx, yy, zz])
end
solve(part1, real_input) do f
    @test f("1,1,1\n2,1,1") == 10
    @test f(test_input) == 64
end

function part2broken(input)
    xx, yy, zz = prepare_input(input)
    anyedge = nothing
    function searchany()
        minx = minimum(t->t[1], xx)
        x, y, z = first(Iterators.filter(t->t[1]==minx, xx))
        'x', -1, x, y, z
    end
    firstedge = searchany()

    Q = Deque{Tuple{Char, Int8, Int8, Int8, Int8}}()
    visited = Set()
    push!(Q, firstedge)
    push!(visited, firstedge)
    function attempt(dir, sign, x, y, z)
        s = nothing
        if dir == 'x'
            s = xx
        elseif dir == 'y'
            s = yy
        elseif dir == 'z'
            s = zz
        else
            error("impossible dir $dir")
        end
        if (x, y, z) ∈ s
            if !((dir, sign, x, y, z) ∈ visited)
                push!(Q, (dir, sign, x, y, z))
                push!(visited, (dir, sign, x, y, z))
            end
            return true
        else
            return false
        end
    end
    while !isempty(Q)
        dir, sign, x, y, z = popfirst!(Q)
        if dir == 'x' && sign == -1
            attempt('y', -1, x-1, y+1, z) || attempt('x', -1, x, y+1, z) || attempt('y',  1, x, y+1, z)
            attempt('y',  1, x-1, y,   z) || attempt('x', -1, x, y-1, z) || attempt('y', -1, x, y,   z)
        elseif dir == 'x' && sign == 1
        elseif dir == 'y' && sign == -1
        elseif dir == 'y' && sign == 1
        elseif dir == 'z' && sign == -1
        elseif dir == 'z' && sign == 1
        else
            error("impossible: $dir, $sign, $x, $y, $z")
        end
    end
    length(visited)
end

function part2(input)
    xx, yy, zz = prepare_input(input)
    # bounding box for search space
    minx = minimum(t->t[1], xx) - 3
    miny = minimum(t->t[2], xx) - 3
    minz = minimum(t->t[3], xx) - 3
    maxx = maximum(t->t[1], xx) + 3
    maxy = maximum(t->t[2], xx) + 3
    maxz = maximum(t->t[3], xx) + 3
    Q = Deque{Tuple{Int8, Int8, Int8}}()
    visited = Set{Tuple{Int8, Int8, Int8}}()
    push!(Q, (minx, miny, minz))
    push!(visited, (minx, miny, minz))
    used_edges = 0
    function attempt(x, y, z)
        t = (x, y, z)
        if !(t ∈ visited)
            push!(Q, t)
            push!(visited, t)
        end
    end
    while !isempty(Q)
        x, y, z = popfirst!(Q)
        if x < maxx
            if (x+1, y, z) ∈ xx
                used_edges += 1
            else
                attempt(x+1, y, z)
            end
        end
        if x > minx
            if (x, y, z) ∈ xx
                used_edges += 1
            else
                attempt(x-1, y, z)
            end
        end
        if y < maxy
            if (x, y+1, z) ∈ yy
                used_edges += 1
            else
                attempt(x, y+1, z)
            end
        end
        if y > miny
            if (x, y, z) ∈ yy
                used_edges += 1
            else
                attempt(x, y-1, z)
            end
        end
        if z < maxz
            if (x, y, z+1) ∈ zz
                used_edges += 1
            else
                attempt(x, y, z+1)
            end
        end
        if z > minz
            if (x, y, z) ∈ zz
                used_edges += 1
            else
                attempt(x, y, z-1)
            end
        end
    end
    used_edges
end
solve(part2, real_input) do f
    @test f(test_input) == 58
end
