using Test
using Pkg
Pkg.activate(".")
using Chain
include("utils.jl")

real_input = readchomp("input/2022/day10.txt")
test_input = raw"""
noop
addx 3
addx -5"""

test_large = raw"""
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"""

function seq(ch, input)
    x = 1
    for line in eachsplit(input, '\n')
        if line[1] == 'a'
            _, amounts = split(line, ' ')
            amount = parse(Int, amounts)
            put!(ch, x)
            put!(ch, x)
            x += amount
        else
            put!(ch, x)
        end
    end
    put!(ch, x)
end

function part1(input)
    r = 0
    for (i, x) in enumerate(Channel(ch -> seq(ch, input)))
        if i in 20:40:220
            r += i * x
        end
    end
    r
end
solve(part1, real_input) do f
    @test f(test_large) == 13140
end

function part2(input)
    for (i, x) in enumerate(Channel(ch -> seq(ch, input)))
        if i % 40 == 1
            println()
        end
        if abs(i%40-x-1) <= 1
            print('#')
        else
            print('.')
        end
    end
end
solve(part2, real_input) do f
    f(test_large)
end
