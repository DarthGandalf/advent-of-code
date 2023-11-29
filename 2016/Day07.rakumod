unit module Day07;

grammar G {
	regex TOP { <outside> }

}

our sub part1(Str $input) {
	my $re = /(.) (.) $1 $0 <?{ $0 ne $1 }>/;
	$input.lines.grep({
		my $outsides = S:g/'[' \w+ ']'/[()]/;
		my @insides = m:g/'[' \w+ ']'/;
		$outsides ~~ $re and @insides.none ~~ $re;
	}).elems
}

our sub part2(Str $input) {
	$input.lines.grep({
		my $outsides = S:g/'[' \w+ ']'/[()]/;
		my @insides = m:g/'[' \w+ ']'/;
		my @abas = m:ov/(.) (.) $0 <?{ $0 ne $1 }>/ given $outsides;
		@abas.any ~~ {
			@insides.any ~~ / "$^a[1]$a[0]$a[1]" /
		}
	}).elems
}
