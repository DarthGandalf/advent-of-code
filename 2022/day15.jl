using Test
using Pkg
Pkg.activate(".")
include("utils.jl")

real_input = readchomp("input/2022/day15.txt")
test_input = raw"""
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"""

struct Sensor
    sx::Int
    sy::Int
    bx::Int
    by::Int
end

struct IntervalSet
    a::Vector{Int}
    b::Vector{Int}

    IntervalSet() = new([], [])
end

function Base.show(io::IO, s::IntervalSet)
    if isempty(s.a)
        print(io, "empty")
    end
    join(io, ("[$a, $b]" for (a, b) in zip(s.a, s.b)), "+")
end

@enum Edge start finish

function add!(s::IntervalSet, a::Int, b::Int)
    push!(s.a, a)
    push!(s.b, b)
end

function normalize!(s::IntervalSet)
    line = []
    append!(line, [(x, start) for x in s.a], [(x, finish) for x in s.b])
    sort!(line)
    empty!(s.a)
    empty!(s.b)
    counter = 0
    for (point, edge) in line
        if edge == start
            if counter == 0
                if !isempty(s.b) && s.b[end] + 1 == point
                    pop!(s.b)
                else
                    push!(s.a, point)
                end
            end
            counter += 1
        else
            counter -= 1
            if counter == 0
                push!(s.b, point)
            end
        end
    end
end

count(s) = sum((b - a + 1) for (a, b) in zip(s.a, s.b); init=0)

function parse_input(input)
    [Sensor(a...) for a in signed_ints.(eachsplit(input, '\n'))]
end

function fill!(I, S, y)
    for s in S
        dy = abs(s.sy-y)
        l = abs(s.sx-s.bx) + abs(s.sy-s.by) - dy
        if l >= 0
            a = s.sx - l
            b = s.sx + l
            add!(I, a, b)
        end
    end
end

function part1(input, y=2000000)
    S = parse_input(input)
    B = Set((s.bx, s.by) for s in S)
    I = IntervalSet()
    fill!(I, S, y)
    normalize!(I)
    count(I) - Base.count(b->b[2]==y, B)
end
solve(part1, real_input) do f
    @test f(test_input, 10) == 26
end

function part2(input, y1=0, y2=4000000)
    S = parse_input(input)
    results = []
    for y = y1:y2
        I = IntervalSet()
        add!(I, y1-10, y1-1)
        add!(I, y2+1, y2+10)
        fill!(I, S, y)
        normalize!(I)
        if y % 100000 == 0
            @show y, I
        end
        if length(I.a) > 2
            error("y=$y too long: $I")
        end
        if length(I.a) == 2
            for x = I.b[1]+1:I.a[2]-1
                push!(results, (x, y))
            end
        end
    end
    (X, Y) = (results |> only)
    X * 4000000 + Y
end
solve(part2, real_input) do f
    @test f(test_input, 0, 20) == 56000011
end
