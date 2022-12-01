using Test
using AutoHashEquals

real_input = readlines("input/2016/day2.txt")
test_input = ["ULL", "RRDDD", "LURDL", "UUUUD"]

@auto_hash_equals mutable struct Pos
    x::Int8
    y::Int8
end

function part1(input)
    pos = Pos(2, 2)
    result = []
    for line in input
        for char in line
            if char == 'D'
                pos.y = min(3, pos.y+1)
            elseif char == 'U'
                pos.y = max(1, pos.y-1)
            elseif char == 'L'
                pos.x = max(1, pos.x-1)
            elseif char == 'R'
                pos.x = min(3, pos.x+1)
            else
                error("Bad direction " * char)
            end
        end
        push!(result, pos.x + (pos.y-1)*3)
    end
    join(result)
end

print("Part 1... ")
@test part1(test_input) == "1985"
println("ok")
println("Result: ", part1(real_input))


function part2(input)
    map = Dict([
        (Pos(3, 1), '1'),
        (Pos(2, 2), '2'),
        (Pos(3, 2), '3'),
        (Pos(4, 2), '4'),
        (Pos(1, 3), '5'),
        (Pos(2, 3), '6'),
        (Pos(3, 3), '7'),
        (Pos(4, 3), '8'),
        (Pos(5, 3), '9'),
        (Pos(2, 4), 'A'),
        (Pos(3, 4), 'B'),
        (Pos(4, 4), 'C'),
        (Pos(3, 5), 'D'),
    ])
    pos = Pos(1, 3)
    result = []
    for line in input
        for char in line
            newpos = deepcopy(pos)
            if char == 'D'
                newpos.y += 1
            elseif char == 'U'
                newpos.y -= 1
            elseif char == 'L'
                newpos.x -= 1
            elseif char == 'R'
                newpos.x += 1
            else
                error("Bad direction " * char)
            end
            if newpos in keys(map)
                pos = newpos
            end
        end
        push!(result, get(map, pos, '#'))
    end
    join(result)
end

print("Part 2... ")
@test part2(test_input) == "5DB3"
println("ok")
println("Result: ", part2(real_input))

