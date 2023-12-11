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

unit sub MAIN(:$day!);

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
