using Test

real_input = readchomp("input/2016/day1.txt")

function part1(input)
    pos = (0, 0)
    dir = (0, 1)
    for step in split(input, ',') .|> strip
        if step[1] == 'L'
            dir = (-dir[2], dir[1])
        else
            dir = (dir[2], -dir[1])
        end
        len = parse(Int, step[2:end])
        pos = pos .+ len .* dir
    end
    abs(pos[1]) + abs(pos[2])
end

print("Part 1... ")
@test part1("R2, L3") == 5
@test part1("R2, R2, R2") == 2
@test part1("R5, L5, R5, R3") == 12
println("ok")
println("Result: ", part1(real_input))

function part2(input)
    pos = (0, 0)
    dir = (0, 1)
    was = Set([pos])
    for step in split(input, ',') .|> strip
        if step[1] == 'L'
            dir = (-dir[2], dir[1])
        else
            dir = (dir[2], -dir[1])
        end
        len = parse(Int, step[2:end])
        for i in 1:len
            pos = pos .+ dir
            if pos ∈ was
                return abs(pos[1]) + abs(pos[2])
            end
            push!(was, pos)
        end
    end
end

print("Part 2... ")
@test part2("R8, R4, R4, R8") == 4
println("ok")
println("Result: ", part2(real_input))
