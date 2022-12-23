using Test
using Pkg
Pkg.activate(".")
using Chain
include("utils.jl")

real_input = readchomp("input/2022/day22.txt")
test_input = raw"""
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"""

@enum Space empty wall void

function parse_input(input)
    karta_lines, instruction = split(input, "\n\n")
    lines = split(karta_lines, '\n')
    total_width = maximum(length, lines)
    karta = fill(void, (length(lines), total_width))
    for (row, line) in enumerate(lines)
        for (col, char) in enumerate(line)
            if char == '#'
                karta[row, col] = wall
            elseif char == '.'
                karta[row, col] = empty
            end
        end
    end
    last_num = 0
    instructions = []
    for ch in instruction
        if isdigit(ch)
            last_num = last_num * 10 + parse(Int, ch)
        else
            push!(instructions, last_num)
            last_num = 0
            push!(instructions, ch)
        end
    end
    if last_num > 0
        push!(instructions, last_num)
    end
    karta, instructions
end

function part1(input)
    karta, instructions = parse_input(input)
    posrow = 1
    poscol = 0
    for i in 1:size(karta, 2)
        if karta[1, i] == empty
            poscol = i
            break
        end
    end
    drow = 0
    dcol = 1
    for inst in instructions
        if inst == 'L'
            drow, dcol = -dcol, drow
        elseif inst == 'R'
            drow, dcol = dcol, -drow
        else
            for i in 1:inst
                next_pos = (mod1(posrow + drow, size(karta, 1)), mod1(poscol + dcol, size(karta, 2)))
                while karta[next_pos...] == void
                    next_pos = (mod1(next_pos[1] + drow, size(karta, 1)), mod1(next_pos[2] + dcol, size(karta, 2)))
                end
                if karta[next_pos...] == empty
                    posrow, poscol = next_pos
                end
            end
        end
    end
    result = 1000 * posrow + 4 * poscol
    if drow == 1
        result += 1
    elseif drow == -1
        result += 3
    elseif dcol == -1
        result += 2
    end
    result
end

solve(part1, real_input) do f
    @test f(test_input) == 6032
end
