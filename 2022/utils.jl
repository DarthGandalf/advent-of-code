ints(str) = parse.(Int, eachmatch(r"(\d+)", str) .|> only)

current_part = 0

function solve(tests, solution, real_input)
    global current_part += 1
    print("Part $current_part... ")
    tests(solution)
    println("ok")
    @time result = solution(real_input)
    println("Result: $result")
end
