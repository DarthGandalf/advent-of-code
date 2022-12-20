using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using DataStructures

real_input = readchomp("input/2022/day19.txt")
test_input = raw"""
Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian."""

function parse_input(input)
    Iterators.partition(input |> ints, 7) |> collect
end

function part1(input)
    result = 0
    for (id, ore_cost, clay_cost, obs_cost_ore, obs_cost_clay, geo_cost_ore, geo_cost_obs) in parse_input(input)
        space::Set{Tuple{Int8, Int8, Int8, Int8, Int16, Int16, Int8, Int8}} = Set([(1, 0, 0, 0, 0, 0, 0, 0)])
        for day in 1:23
            @show id, day, length(space)
            new_space = Set()
            for (robot_ore, robot_clay, robot_obs, robot_geo, ore, clay, obs, geo) in space
                if ore >= ore_cost
                    push!(new_space, (robot_ore + 1, robot_clay, robot_obs, robot_geo, ore + robot_ore - ore_cost, clay + robot_clay, obs + robot_obs, geo + robot_geo))
                end
                if ore >= clay_cost
                    push!(new_space, (robot_ore, robot_clay + 1, robot_obs, robot_geo, ore + robot_ore - clay_cost, clay + robot_clay, obs + robot_obs, geo + robot_geo))
                end
                if ore >= obs_cost_ore && clay >= obs_cost_clay
                    push!(new_space, (robot_ore, robot_clay, robot_obs + 1, robot_geo, ore + robot_ore - obs_cost_ore, clay + robot_clay - obs_cost_clay, obs + robot_obs, geo + robot_geo))
                end
                if ore >= geo_cost_ore && obs >= geo_cost_obs
                    push!(new_space, (robot_ore, robot_clay, robot_obs, robot_geo + 1, ore + robot_ore - geo_cost_ore, clay + robot_clay, obs + robot_obs - geo_cost_obs, geo + robot_geo))
                end
                if ore >= ore_cost && ore >= clay_cost && robot_clay == 0
                elseif ore >= ore_cost && ore >= clay_cost && ore >= obs_cost_ore && clay >= obs_cost_clay && robot_obs == 0
                elseif ore >= ore_cost && ore >= clay_cost && ore >= obs_cost_ore && clay >= obs_cost_clay && ore >= geo_cost_ore && obs >= geo_cost_obs
                else
                    push!(new_space, (robot_ore, robot_clay, robot_obs, robot_geo, ore + robot_ore, clay + robot_clay, obs + robot_obs, geo + robot_geo))
                end
            end
            space = new_space
            z = maximum(x->x[8] + x[4], space)
            @show z
        end
        result += id * maximum(x->x[8] + x[4], space)
    end
    result
end
solve(part1, real_input) do f
    @test f(test_input) == 33
end
