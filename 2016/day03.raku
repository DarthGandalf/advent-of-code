#!/usr/bin/env raku

my $input = slurp 'input/2016/day3.txt';

sub triangle(@nums) {
	my $long = @nums.max;
	my $other = @nums.sum - $long;
	$long < $other
}

sub part1(Str $input) {
	my $count = 0;
	for $input.lines {
		my Int @nums = split(' ', $_, :skip-empty).map(*.Numeric);
		++$count if triangle(@nums);
	}
	return $count
}

sub part2(Str $input) {
	my $count = 0;
	for $input.lines -> $l1, $l2, $l3 {
		my Int @all = split(' ', "$l1 $l2 $l3", :skip-empty).map(*.Numeric);
		++$count if triangle(@all[0,3,6]);
		++$count if triangle(@all[1,4,7]);
		++$count if triangle(@all[2,5,8]);
	}
	$count
}

say 'Part 1: ', part1 $input;
say 'Part 2: ', part2 $input;

say „done after { now - INIT now }s“;
