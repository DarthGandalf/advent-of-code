using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using YAML
using OrderedCollections

real_input = readchomp("input/2022/day11.txt")
test_input = raw"""
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"""

mutable struct Monkey
    items::Vector{Int}
    op::Function
    test::Int
    yes::Int8
    no::Int8
    inspections::Int
end

function parse_input(input)
    input = replace(input, "Monkey "=>"m", "Starting "=>"", "Operation: new ="=>"op:", "  If t"=>"", "  If f"=>"")
    data = YAML.load(input; dicttype=OrderedDict{String,Any})
    function create_monkey(d)
        op = Meta.parse("old -> " * d["op"]) |> eval
        Monkey(d["items"]|>ints, old->Base.invokelatest(op, old), d["Test"]|>ints|>only, (d["rue"]|>ints|>only) + 1, (d["alse"]|>ints|>only) + 1, 0)
    end
    Monkey[create_monkey(v) for (_, v) in data]
end

function work(monkeys, relief, limit)
    for round in 1:limit
        for monkey in monkeys
            for item in monkey.items
                item = monkey.op(item) |> relief
                push!(monkeys[item % monkey.test == 0 ? monkey.yes : monkey.no].items, item)
            end
            monkey.inspections += length(monkey.items)
            empty!(monkey.items)
        end
    end
    partialsort!(monkeys .|> m->m.inspections, 1:2, rev=true) |> prod
end

function part1(input)
    monkeys = parse_input(input)
    work(monkeys, item -> item ÷ 3, 20)
end
function part2(input)
    monkeys = parse_input(input)
    mod = monkeys .|> (m -> m.test) |> prod
    work(monkeys, item -> item % mod, 10000)
end

solve(part1, real_input) do f
    @test f(test_input) == 10605
end
solve(part2, real_input) do f
    @test f(test_input) == 2713310158
end
