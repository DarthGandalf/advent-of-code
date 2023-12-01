#!/usr/bin/env raku

use lib '.';
use Day01;
subset Day of Int where 1 <= * <= 1;

unit sub MAIN(Day :$day!);

my $input = chomp slurp "input/2023/day$day.txt";

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
