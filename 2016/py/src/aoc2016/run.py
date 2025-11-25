#!/usr/bin/python

from importlib.metadata import entry_points
import argparse
import time
import datetime

def run():
    parser = argparse.ArgumentParser()
    parser.add_argument("--day", type=int, required=True)
    parser.add_argument("--part", type=int)
    args = parser.parse_args()

    eps = entry_points(group="aoc2016.day")
    solver = eps[f"day{args.day}"].load()()

    with open(f'../input/2016/day{args.day}.txt', 'rt') as f:
        solver.parse(f)

    for part, func in {1: solver.part1, 2: solver.part2}.items():
        if args.part is None or args.part == part:
            print(f"Part {part}...")
            start = time.time_ns()
            result = func()
            end = time.time_ns()
            elapsed = datetime.timedelta(microseconds=(end - start) / 1000)
            print(f"Result: {result} ({elapsed})")
            print()
