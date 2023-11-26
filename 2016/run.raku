#!/usr/bin/env raku

use lib '.';
use Day03;
use Day04;

unit sub MAIN(Int :$day);

my $input = slurp "input/2016/day$day.txt";

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
