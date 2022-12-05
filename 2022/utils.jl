ints(str::AbstractString) = Base.parse.(Int, eachmatch(r"(\d+)", str) .|> only)

current_part::Int8 = 0

function solve(tests::Function, solution::Function, real_input)
    global current_part += 1
    println()
    print("Part $current_part... ")
    tests(solution)
    println("ok")
    @time result = solution(real_input)
    println("Result: $result")
end
