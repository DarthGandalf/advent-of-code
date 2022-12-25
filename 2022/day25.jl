using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using Chain

real_input = readchomp("input/2022/day25.txt")
test_input = raw"""
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"""


struct SNAFU
    digits::Vector{Int8}
end
const dir1 = Dict("2"=>2, "1"=>1, "0"=>0, "-"=>-1, "="=>-2)
function SNAFU(text::AbstractString)
    @chain text begin
        split(_, "")
        @. (x->dir1[x])(_)
        reverse
        SNAFU
    end
end

function Base.:+(a::SNAFU, b::SNAFU)
    r = Int8[]
    overflow = 0
    for i in 1:max(length(a.digits), length(b.digits))
        x = get(a.digits, i, 0)
        y = get(b.digits, i, 0)
        z = x + y + overflow
        overflow = 0
        if z > 2
            z -= 5
            overflow = 1
        elseif z < -2
            z += 5
            overflow = -1
        end
        push!(r, z)
    end
    push!(r, overflow)
    while length(r) > 1 && r[end] == 0
        pop!(r)
    end
    SNAFU(r)
end

const dir2 = Dict(2=>'2', 1=>'1', 0=>'0', -1=>'-', -2=>'=')
function Base.show(io::IO, a::SNAFU)
    for x in reverse(a.digits)
        print(io, dir2[x])
    end
end

function part(input)
    @chain input begin
        eachsplit(_, '\n')
        SNAFU.()
        sum(_; init=SNAFU("0"))
        repr
    end
end
solve(part, real_input) do f
    @test f(test_input) == "2=-1=0"
end
