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
    maxy = isempty(cup) ? 0 : last(cup)[1]
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
    vault = 0
    function shift()
        m = last(cup)[1]
        if m > 200
            d = m - 200
            vault += d
            newcup = SortedSet()
            for (y, x) in cup
                y -= d
                if y > 0
                    push!(newcup, (y, x))
                end
            end
            cup = newcup
        end
    end
    k = 0
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
        k += 1
        if k > 50
            shift()
            k = 0
        end
    end
    vault + last(cup)[1]
end
solve(part1, real_input) do f
    @test f(test_input) == 3068
end

function part2(input)
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
    vault = 0
    function shift()
        m = last(cup)[1]
        if m > 1000
            d = m - 1000
            vault += d
            newcup = SortedSet()
            for (y, x) in cup
                y -= d
                if y > 0
                    push!(newcup, (y, x))
                end
            end
            cup = newcup
        end
    end
    jets = nextjet(input)
    long_step_len = lcm(length(input), 5)
    function longstep(L)
        k = 0
        for rock in Iterators.take(nextrock(), L)
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
            k += 1
            if k > 5000
                shift()
                k = 0
            end
        end
        shift()
    end
    seen = Dict()
    N = 0
    cycle_len = 0
    cycle_height = 0
    while true
        N += 1
        @show N
        longstep(long_step_len)
        here = vault + last(cup)[1]
        scup = "$cup"
        prev = get(seen, scup, nothing)
        if prev != nothing
            @show prev
            cycle_len = (N - prev[1]) * long_step_len
            cycle_height = vault + last(cup)[1] - prev[2]
            break
        end
        push!(seen, "$cup" => (N, here))
    end
    steps_already = N * long_step_len
    @show N, cycle_len, cycle_height, steps_already
    steps_needed = 1000000000000 - steps_already
    cycles_needed = steps_needed ÷ cycle_len
    @show cycles_needed
    vault += cycle_height * cycles_needed
    longstep(steps_needed % cycle_len)
    vault + last(cup)[1]
end
solve(part2, real_input) do f
    @test f(test_input) == 1514285714288
end
