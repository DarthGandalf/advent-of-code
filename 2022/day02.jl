using Test

real_input = readchomp("input/2022/day2.txt")
test_input = """
A Y
B X
C Z"""

function part1(input)
    x = Dict(['A'=>1, 'B'=>2, 'C'=>3,
              'X'=>1, 'Y'=>2, 'Z'=>3])
    result = 0
    for line in eachsplit(input, '\n')
        their = x[line[1]]
        mine = x[line[end]]
        if their == 3 && mine == 1
            their = 0
        end
        if their == 1 && mine == 3
            their = 4
        end
        won = sign(mine - their)
        result += mine + 3 + 3won
    end
    result
end

print("Part 1... ")
@test part1(test_input) == 15
println("ok")
println("Result: ", part1(real_input))

function part2(input)
    x = Dict(['A'=>1, 'B'=>2, 'C'=>3,
              'X'=>0, 'Y'=>3, 'Z'=>6])
    result = 0
    for line in eachsplit(input, '\n')
        their = x[line[1]]
        score = x[line[end]]
        mine = -1
        if score == 0
            mine = their - 1
            if mine == 0
                mine = 3
            end
        elseif score == 6
            mine = their + 1
            if mine == 4
                mine = 1
            end
        else
            mine = their
        end
        println("$line: their=$their score=$score mine=$mine")
        # hm, looks like I should have used modulus 3
        result += mine + score
    end
    result
end

print("Part 2... ")
@test part2(test_input) == 12
println("ok")
println("Result: ", part2(real_input))

