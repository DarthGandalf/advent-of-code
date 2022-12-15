using Test
include("utils.jl")

real_input = readchomp("input/2022/day14.txt")
test_input = raw"""
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"""

@enum Space free sand wall

function parse_input(input)
    M = Dict()
    for line in eachsplit(input, '\n')
        ((px, py), rest) = Iterators.partition(ints(line), 2) |> Iterators.peel
        for (nx, ny) in rest
            for x in min(px,nx):max(px,nx), y in min(py,ny):max(py,ny)
                M[(x, y)] = wall
            end
            (px, py) = (nx, ny)
        end
    end
    M
end

function showM(M)
    minx = minimum(a->a[1], keys(M))
    miny = minimum(a->a[2], keys(M))
    maxx = maximum(a->a[1], keys(M))
    maxy = maximum(a->a[2], keys(M))
    miny = min(0, miny)
    for y in miny:maxy
        println()
        for x in minx:maxx
            a = get(M, (x, y), free)
            if a == wall
                print('#')
            elseif a == sand
                print('o')
            else
                print('.')
            end
        end
    end
    println()
end

function do1(M)
    maxy = maximum(a->a[2], keys(M))
    place = (500, 0)
    while place[2] < maxy
        tries = [place .+ (0, 1), place .+ (-1, 1), place .+ (1, 1)]
        spaces = [t for t in tries if get(M, t, free) == free]
        if length(spaces) == 0
            M[place] = sand
            return true
        end
        place = spaces[1]
    end
    return false
end
function do2(M, maxy)
    place = (500, 0)
    while true
        tries = [place .+ (0, 1), place .+ (-1, 1), place .+ (1, 1)]
        spaces = [t for t in tries if get(M, t, free) == free]
        if length(spaces) == 0 || place[2] >= maxy
            M[place] = sand
            return
        end
        place = spaces[1]
    end
end

function part1(input)
    M = parse_input(input)
    while do1(M)
    end
    count(==(sand), values(M))
end
function part2(input)
    M = parse_input(input)
    maxy = maximum(a->a[2], keys(M))
    while get(M, (500, 0), free) == free
        do2(M, maxy + 1)
    end
    count(==(sand), values(M))
end
solve(part1, real_input) do f
    @test f(test_input) == 24
end
solve(part2, real_input) do f
    @test f(test_input) == 93
end
