#!/usr/bin/env raku

use lib '.';
use Day01;
use Day01x;

unit sub MAIN(:$day!);

my $input = chomp slurp "input/2023/day$day.txt";

my $module;
if $day eq '1x' {
	$module = 'Day01x';
} else {
	$module = $day < 10 ?? "Day0$day" !! "Day$day";
}

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
