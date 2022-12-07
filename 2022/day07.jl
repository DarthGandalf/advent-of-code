using Test
using Pkg
Pkg.activate(".")
using Chain
include("utils.jl")

real_input = readchomp("input/2022/day7.txt")
test_input = raw"""
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""

mutable struct FileOrDir
    dir::Bool
    size::Int
    contents::Dict{AbstractString, FileOrDir}
end

function Base.show(io::IO, f::FileOrDir, indent=1)
    if f.dir
        println(io, ": (", f.size, ")")
        for (name, sub) in f.contents
            print(io, "    " ^ indent, "- ", name)
            show(io, sub, indent + 1)
        end
    else
        println(io, ", ", f.size)
    end
end

function parse(input)
    root::FileOrDir = FileOrDir(true, -1, Dict())
    path::Vector{FileOrDir} = [root]
    cdcmd = r"^\$ cd (.+)"
    dirout = r"^dir (.+)"
    sizeout = r"^(\d+) (.+)"
    for line in eachsplit(input, '\n')
        mcd = match(cdcmd, line)
        mdir = match(dirout, line)
        msize = match(sizeout, line)
        if mcd != nothing
            subdir = mcd[1]
            if subdir == "/"
                path = [root]
            elseif subdir == ".."
                @assert length(path) > 1 "Can't cd .."
                pop!(path)
            else
                sub = path[end].contents[subdir]
                @assert sub.dir
                push!(path, sub)
            end
        elseif mdir != nothing
            name = mdir[1]
            @assert !haskey(path[end].contents, name)
            path[end].contents[name] = FileOrDir(true, -1, Dict())
        elseif msize != nothing
            name = msize[2]
            size = Base.parse(Int, msize[1])
            @assert !haskey(path[end].contents, name)
            path[end].contents[name] = FileOrDir(false, size, Dict())
        else
        end
    end
    root
end

function fillsizes(dir)
    if dir.size >= 0
        return
    end
    s = 0
    for (name, sub) in dir.contents
        fillsizes(sub)
        s += sub.size
    end
    dir.size = s
end

function alldirs(ch, dir)
    if dir.dir
        put!(ch, dir)
        for (_, sub) in dir.contents
            alldirs(ch, sub)
        end
    end
end

getsize(dir) = dir.size

function part1(input)
    root = parse(input)
    fillsizes(root)
    @chain Channel(ch->alldirs(ch, root)) begin
        @. getsize
        Iterators.filter(x -> x <= 100000, _)
        sum
    end
end

solve(part1, real_input) do f
    @test f(test_input) == 95437
end

function part2(input)
    root = parse(input)
    fillsizes(root)
    needed = root.size - 40000000
    @chain Channel(ch->alldirs(ch, root)) begin
        @. getsize
        Iterators.filter(x -> x >= needed, _)
        minimum
    end
end

solve(part2, real_input) do f
    @test f(test_input) == 24933642
end
