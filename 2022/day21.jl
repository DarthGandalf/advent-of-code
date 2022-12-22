using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using Z3

real_input = readchomp("input/2022/day21.txt")
test_input = raw"""
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""

function part1(input)
    program = replace(input, ":"=>"=", "/"=>"`div`")
    open("day21.hs", "w") do io
        println(io, program)
        println(io, "main = putStrLn (show root)")
    end
    run(`ghc day21.hs`)
    readchomp(`./day21`)
end

#solve(part1, real_input) do f
#    @test f(test_input) == "152"
#end

function part2broken(input)
    program = replace(replace(input,
                              ":" => "(H, X) :-",
                              r"(\d+)" => s"X is \1.",
                              r"(\w+) (.) (\w+)$"m => s"\1(H, A), \3(H, B), X #= A \2 B."),
                      r"^root.*:-(.*,.*),.*$"m => s"root(H) :-\1, A #= B.",
                      r"^humn.*"m => "humn(H, H).")
    open("day21.pl", "w") do io
        println(io, program)
    end
    output = readchomp(`gprolog --consult-file day21.pl --query-goal 'root(ANSWER)'`)
    println(output)
    match(r"^ANSWER\s*=\s*(\d+)$"m, output).captures[1]
end

function part2(input)
    ctx = Context()
    vars = Dict([split(line, ':')[1] => int_const(ctx, string(split(line, ':')[1]))
                 for line in eachsplit(input, '\n')])
    s = Solver(ctx)
    for line in eachsplit(input, '\n')
        name, expr = split(line, ": ")
        name == "humn" && continue
        parts = split(expr, ' ')
        if length(parts) == 1
            add(s, vars[name] == parse(Int, expr))
        else
            l, op, r = parts
            if name == "root"
                add(s, vars[l] == vars[r])
            elseif op == "+"
                add(s, vars[name] == vars[l] + vars[r])
            elseif op == "-"
                add(s, vars[name] == vars[l] - vars[r])
            elseif op == "*"
                add(s, vars[name] == vars[l] * vars[r])
            elseif op == "/"
                add(s, vars[name] * vars[r] == vars[l])
            else
                error("unknown op $op")
            end
        end
    end
    res = check(s)
    @assert res == Z3.sat
    m = get_model(s)
    for (k, v) in consts(m)
        io = IOBuffer()
        print(io, k)
        if String(take!(io)) == "humn"
            print(io, v)
            return String(take!(io))
        end
    end
end

# 3759569926193 too high
solve(part2, real_input) do f
    @test f(test_input) == "301"
end
