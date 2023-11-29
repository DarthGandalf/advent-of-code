unit module Day09;

our sub part1(Str $input is copy) {
	my Int $outlen = 0;
	loop {
		my $m = $input ~~ /^(.*?) '(' (\d+) 'x' (\d+) ')' (.*)$/;
		if $m {
			$outlen += $m[0].chars;
			$outlen += $m[1] * $m[2];
			$input = $m[3].substr($m[1]);
		} else {
			$outlen += $input.chars;
			last
		}
	}
	$outlen
}

sub decomp(Str() $input is copy --> Int) {
	my Int $outlen = 0;
	loop {
		my $m = $input ~~ /^(.*?) '(' (\d+) 'x' (\d+) ')' (.*)$/;
		if $m {
			$outlen += $m[0].chars;
			$outlen += decomp($m[3].substr(0, $m[1])) * $m[2];
			$input = $m[3].substr($m[1]);
		} else {
			$outlen += $input.chars;
			last
		}
	}
	$outlen
}

our sub part2(Str $input) {
	decomp($input)
}
