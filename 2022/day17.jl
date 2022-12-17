using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using ResumableFunctions
using DataStructures

real_input = readchomp("input/2022/day17.txt")
test_input = raw""">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"""

@resumable function nextjet(input)
    while true
        for c in input
            @yield c
        end
    end
end

@resumable function nextrock()
    while true
        @yield [(0, 0), (0, 1), (0, 2), (0, 3)]
        @yield [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
        @yield [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
        @yield [(0, 0), (1, 0), (2, 0), (3, 0)]
        @yield [(0, 0), (0, 1), (1, 0), (1, 1)]
    end
end

function showcup(cup)
    maxy = maximum(p->p[1], cup; init=0)
    println()
    for y in maxy:-1:1
        print('|')
        for x in 1:7
            if (y, x) ∈ cup
                print('#')
            else
                print('.')
            end
        end
        println('|')
    end
    println("+-------+")
    println()
end

function part1(input)
    jets = nextjet(input)
    cup = SortedSet()
    function trymove(rock, dir)
        for (y, x) in rock
            y += dir[1]
            x += dir[2]
            if x < 1 || x > 7 || y < 1 || (y, x) ∈ cup
                return (false, rock)
            end
        end
        (true, [p .+ dir for p in rock])
    end
    for rock in Iterators.take(nextrock(), 2022)
        maxy = isempty(cup) ? 0 : last(cup)[1]
        r = map(x->x.+(maxy + 4, 3), rock)
        while true
            jet, jets = Iterators.peel(jets)
            dir = jet == '>' ? (0, 1) : (0, -1)
            _, r = trymove(r, dir)
            success, r = trymove(r, (-1, 0))
            if !success
                union!(cup, r)
                break
            end
        end
        #showcup(cup)
    end
    last(cup)[1]
end
solve(part1, real_input) do f
    @test f(test_input) == 3068
end
