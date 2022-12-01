using Test

real_input = readchomp("input/2022/day1.txt")
test_input = """
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000"""

function part1(input)
    result = 0
    for elf = eachsplit(input, "\n\n")
        this = sum(parse.(Int, eachsplit(elf)))
        result = max(this, result)
    end
    result
end

print("Part 1... ")
@test part1(test_input) == 24000
println("ok")
println("Result: ", part1(real_input))

function part2(input)
    return sum(partialsort!(eachsplit(test_input, "\n\n")
                            .|> (elf -> parse.(Int, eachsplit(elf)) |> sum),
                            1:3, rev=true))
    # previous solution, also works but doesn't use fancy julia features
    elves = []
    for elf = eachsplit(input, "\n\n")
        this = parse.(Int, eachsplit(elf)) |> sum
        push!(elves, this)
    end
    sum(partialsort!(elves, 1:3, rev=true))
end

print("Part 2... ")
@test part2(test_input) == 45000
println("ok")
println("Result: ", part2(real_input))
