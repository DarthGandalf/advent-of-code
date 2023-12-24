#!/usr/bin/env raku

use lib '.';
use Day01;
use Day01x;
use Meal02;
use Day03;
use Day04;
use Day05;
use Day07;
use День08;
use Day09;
use Day10;
use DayEleven;
use Day12;
use Day13;
use Day14;
use Day15;
use Day16;
use Day17;
use Day18;
use Day19;
use Day20;
use Day21;
use Day22;
use Day23;
use Day24;

module Day14 {
	our sub part1($input) {
		day14part1($input);
	}
	our sub part2($input) {
		day14part2($input);
	}
}

sub MAIN(:$day!) {

my $input = chomp slurp "input/2023/day$day.txt";

my $module = do given $day {
	when '1x' { 'Day01x' }
	when 2 { 'Meal02' }
	when 8 { 'День08' }
	when * < 10 { "Day0$day" }
	when 11 { "DayEleven" }
	default { "Day$day" }
};
my $partfunc = do given $day {
	when 2 { "entrée" }
	when 8 { "часть" }
	default { "part" }
};

sub part($part) {
	my $before = ENTER now;
	say "Part $part";
	say '======';
	say "Answer: ", &::($module)::("$partfunc$part")($input);
	say "took  : { now - $before }s";
	say '';
}

if $day == 11 {
	part "One";
	part "Two";
} else {
	part 1;
	part 2;
}

say "total : { now - INIT now }s";
}
