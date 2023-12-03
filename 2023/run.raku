#!/usr/bin/env raku

use lib '.';
use Day01;
use Day01x;
use Meal02;
use Day03;

unit sub MAIN(:$day!);

my $input = chomp slurp "input/2023/day$day.txt";

my $module = do given $day {
	when '1x' { 'Day01x' }
	when 2 { 'Meal02' }
	when * < 10 { "Day0$day" }
	default { "Day$day" }
};
my $partfunc = $day == 2 ?? "entrée" !! "part";

sub part($part) {
	my $before = ENTER now;
	say "Part $part";
	say '======';
	say "Answer: ", &::($module)::("$partfunc$part")($input);
	say "took  : { now - $before }s";
	say '';
}
part 1;
part 2;

say "total : { now - INIT now }s";
