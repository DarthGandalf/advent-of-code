using Test
using Pkg
#Pkg.activate(".")
include("utils.jl")

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

solve(part1, real_input) do f
    @test f(test_input) == "152"
end
