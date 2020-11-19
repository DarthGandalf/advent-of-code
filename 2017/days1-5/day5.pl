#!/usr/bin/perl
use 5.30.0;
use strict;
use warnings;
use diagnostics;
use utf8;
use Data::Dumper;

my @mem = map { chomp; $_ } <>;
#@mem = (0, 3, 0, 1, -3);
$, = ' ';
#print Dumper(\@mem);

my $pos = 0;
for (my $steps = 0;; $steps++) {
	#say @mem;
	if ($pos > $#mem) {
		say $steps;
		exit;
	}
	my $off = $mem[$pos];
	if ($off >= 3) {
		$mem[$pos]--;
	} else {
		$mem[$pos]++;
	}
	$pos += $off
}
