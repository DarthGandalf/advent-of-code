using Test
using Pkg
Pkg.activate(".")
include("utils.jl")
using DataStructures

real_input = readchomp("input/2022/day20.txt")
test_input = raw"""
1
2
-3
3
-2
0
4"""

mutable struct Node
    num::Int
    prev::Union{Node, Nothing}
    next::Union{Node, Nothing}
end

function move(this, mod)
    this.num == 0 && return
    this.prev.next = this.next
    this.next.prev = this.prev
    if this.num > 0
        for i in 1:(this.num % mod)
            this.next = this.next.next
        end
        this.prev = this.next.prev
    else
        for i in 1:((-this.num) % mod)
            this.prev = this.prev.prev
        end
        this.next = this.prev.next
    end
    this.prev.next = this
    this.next.prev = this
end

function show(zero)
    current = zero
    print(current.num, ' ')
    current = current.next
    while current != zero
        print(current.num, ' ')
        current = current.next
    end
    println()
end

function part(input, key, repeats)
    nums = signed_ints(input) .* key
    zero = Node(0, nothing, nothing)
    start = Node(0, nothing, nothing)
    last = start
    pointers = Node[]
    for i in nums
        next = i == 0 ? zero : Node(i, nothing, nothing)
        next.prev = last
        last.next = next
        last = next
        push!(pointers, next)
    end
    last.next = start.next
    start.next.prev = last
    start = nothing
    for j in 1:repeats
        for i in 1:length(nums)
            move(pointers[i], length(nums) - 1)
        end
    end
    result = 0
    current = zero
    for j in 1:3
        for i in 1:1000
            current = current.next
        end
        result += current.num
    end
    result
end
part1(input) = part(input, 1, 1)
part2(input) = part(input, 811589153, 10)
solve(part1, real_input) do f
    @test f(test_input) == 3
end
solve(part2, real_input) do f
    @test f(test_input) == 1623178306
end
