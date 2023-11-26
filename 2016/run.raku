#!/usr/bin/env raku

use lib '.';
use Day03;

unit sub MAIN(Int :$day);

my $input = slurp "input/2016/day$day.txt";

my $module = $day < 10 ?? "Day0$day" !! "Day$day";
say 'Part 1: ', &::($module)::part1($input);
say 'Part 2: ', &::($module)::part2($input);

say „done after { now - INIT now }s“;
