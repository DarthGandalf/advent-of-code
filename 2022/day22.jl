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

function part(warper, karta, instructions)
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
                prev_pos = (posrow, poscol)
                next_pos = (mod1(posrow + drow, size(karta, 1)), mod1(poscol + dcol, size(karta, 2)))
                next_drow, next_dcol = drow, dcol
                if karta[next_pos...] == void
                    next_pos, next_drow, next_dcol = warper(prev_pos, drow, dcol)
                end
                if karta[next_pos...] == void
                    error("warped into void")
                end
                if karta[next_pos...] == empty
                    posrow, poscol = next_pos
                    drow, dcol = next_drow, next_dcol
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

function part1(input)
    karta, instructions = parse_input(input)
    part(karta, instructions) do prev_pos, drow, dcol
        next_pos = (mod1(prev_pos[1] + drow, size(karta, 1)), mod1(prev_pos[2] + dcol, size(karta, 2)))
        while karta[next_pos...] == void
            next_pos = (mod1(next_pos[1] + drow, size(karta, 1)), mod1(next_pos[2] + dcol, size(karta, 2)))
        end
        next_pos, drow, dcol
    end
end

solve(part1, real_input) do f
    @test f(test_input) == 6032
end

#=

       +------+------+
       |  g   |   e  |
       |f     |     d|
       |      |  b   |
       +------+------+
       |      |
       |a    b|
       |      |
+------+------+
|   a  |      |
|f     |     d|
|      |   c  |
+------+------+
|      |
|g    c|
|   e  |
+------+

=#

function wrap2(side, (row, col), drow, dcol)
    if drow == 1
        edge = row ÷ side
        if edge == 1
            return (col-side, side*2), 0, -1 # b
        elseif edge == 3
            return (col+2*side, side), 0, -1 # c
        elseif edge == 4
            return (1, col+side*2), 1, 0 # e
        else
            error("drow==1 but edge==$edge")
        end
    elseif drow == -1
        edge = (row-1) ÷ side
        if edge == 0
            if col > side*2
                return (4*side, col-side*2), -1, 0 # e
            else
                return (col+side*2, 1), 0, 1 # g
            end
        elseif edge == 2
            return (col+side, side+1), 0, 1 # a
        else
            error("drow==-1 but edge==$edge")
        end
    elseif dcol == 1
        edge = col ÷ side
        if edge == 1
            return (side*3, row-side*2), -1, 0 # c
        elseif edge == 2
            if row > side*2
                return (side*3-row+1, side*3), 0, -1 # d
            else
                return (side, row+side), -1, 0 # b
            end
        elseif edge == 3
            return (side*3 - row+1, side*2), 0, -1 # d
        else
            error("dcol==1 but edge==$edge")
        end
    elseif dcol == -1
        edge = (col-1) ÷ side
        if edge == 0
            if row > side*3
                return (1, row -2*side), 1, 0 # g
            else
                return (side*3 - row+1, side+1), 0, 1 # f
            end
        elseif edge == 1
            if row > side
                return (side*2+1, row - side), 1, 0 # a
            else
                return (side*3 - row+1, 1), 0, 1 # f
            end
        else
            error("dcol==-1 but edge==$edge")
        end
    else
        error("wrong direction")
    end
end

function part2(input)
    karta, instructions = parse_input(input)
    side::Int = √((count(!=(void), karta))÷6)
    part(karta, instructions) do prev_pos, drow, dcol
        wrap2(side, prev_pos, drow, dcol)
    end
end

function test2(name, (pos, drow, dcol), (pos2, drow2, dcol2))
    try
        @test wrap2(10, pos, drow, dcol) == (pos2, drow2, dcol2)
        @test wrap2(10, pos2, -drow2, -dcol2) == (pos, -drow, -dcol)
    catch
        println("  While testing warp `$name`")
        rethrow()
    end
end

solve(part2, real_input) do f
    test2("a", ((13, 11), 0, -1), ((21, 3), 1, 0))
    test2("b", ((13, 20), 0, 1), ((10, 23), -1, 0))
    test2("c", ((33, 10), 0, 1), ((30, 13), -1, 0))
    test2("d", ((3, 30), 0, 1), ((28, 20), 0, -1))
    test2("e", ((1, 23), -1, 0), ((40, 3), -1, 0))
    test2("f", ((3, 11), 0, -1), ((28, 1), 0, 1))
    test2("g", ((1, 13), -1, 0), ((33, 1), 0, 1))
end
