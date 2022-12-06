using Test
include("utils.jl")

real_input = readchomp("input/2022/day6.txt")

findstart(input, len) = (input[i:i+len-1] for i in 1:length(input)-len+1) |> (x->findfirst(x) do marker
                                                                                  length(Set(marker)) == len
                                                                              end + len - 1)

part1(input) = findstart(input, 4)
part2(input) = findstart(input, 14)

solve(part1, real_input) do f
    @test f("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7
    @test f("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5
    @test f("nppdvjthqldpwncqszvftbrmjlhg") == 6
    @test f("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10
    @test f("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11
end
solve(part2, real_input) do f
end
