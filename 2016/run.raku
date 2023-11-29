#!/usr/bin/env raku

use lib '.';
use Day03;
use Day04;
use Day05;
use Day06;
use Day07;
subset Day of Int where 3 <= * <= 7;

unit sub MAIN(Day :$day!);

my $input = chomp slurp "input/2016/day$day.txt";

my $module = $day < 10 ?? "Day0$day" !! "Day$day";

sub part($part) {
	my $before = ENTER now;
	say "Part $part";
	say '======';
	say "Answer: ", &::($module)::("part$part")($input);
	say "took  : { now - $before }s";
	say '';
}
part 1;
part 2;

say "total : { now - INIT now }s";
