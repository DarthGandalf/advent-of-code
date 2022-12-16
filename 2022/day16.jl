using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using DataStructures
using ResumableFunctions

real_input = readchomp("input/2022/day16.txt")
test_input = raw"""
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""

struct Valve
    name::AbstractString
    rate::Int
    tunnels::Vector{AbstractString}
    tunneli::Vector{Int8}
end

function parse_input(input)
    function parse_line(line)
        m = match(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)", line)
        name, rate, tuns = m
        Valve(name, parse(Int, rate), split(tuns, ", "), [])
    end
    [parse_line(a) for a in eachsplit(input, '\n')]
end

struct MyVertex
    me::Int8
    day::Int8
    valves::BitArray{1}
end

struct MyGraph
    valves::Vector{Valve}
    total_rate::Int
end

function MyGraph(valves)
    total_rate = sum(v->v.rate, valves)
    indices = Dict()
    for (i, v) in enumerate(valves)
        indices[v.name] = i
    end
    for v in valves
        for t in v.tunnels
            push!(v.tunneli, indices[t])
        end
    end
    MyGraph(valves, total_rate)
end

@resumable function neigh(g::MyGraph, v::MyVertex)
    if g.valves[v.me].rate > 0 && !v.valves[v.me]
        va = copy(v.valves)
        va[v.me] = true
        @yield MyVertex(v.me, v.day+1, va)
    end
    for s in g.valves[v.me].tunneli
        @yield MyVertex(s, v.day+1, v.valves)
    end
end

function distfrom(g::MyGraph, v::MyVertex)
    g.total_rate - sum(v->v.rate, g.valves[v.valves]; init=0)
end

const big = typemax(Int)÷2
function dijkstra(g::MyGraph, start::MyVertex, limit)
    Q = PriorityQueue{MyVertex, Int}()
    Q[start] = 0
    dist = Dict(start => 0)
    ps = 0
    k = 0

    while !isempty(Q)
        current = dequeue!(Q)
        d = get(dist, current, big) + distfrom(g, current)
        k += 1
        if k == 100000
            k = 0
            @show current, distfrom(g, current), length(Q)
        end
        if current.day >= limit
            return dist[current]
        end
        for towards in neigh(g, current)
            if d < get(dist, towards, big)
                dist[towards] = d
                Q[towards] = d
                if ps < towards.day
                    ps = towards.day
                    @show ps
                end
            end
        end
    end
end

function part1(input)
    valves = parse_input(input)
    sort!(valves, by=v->v.name)
    graph = MyGraph(valves)
    start = MyVertex(1, 0, falses(length(valves)))
    z = dijkstra(graph, start, 30)
    graph.total_rate * 30 - z
end
solve(part1, real_input) do f
    @test f(test_input) == 1651
end

function part2(input)
    valves = parse_input(input)
    sort!(valves, by=v->v.name)
    graph = MyGraph(valves)
    start = MyVertex(1, 0, falses(length(valves)))
    z = dijkstra(graph, start, 26, neigh2)
    graph.total_rate * 26 - z
end
solve(part2, real_input) do f
    @test f(test_input) == 1707
end
