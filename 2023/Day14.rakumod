our sub day14part1(Str $input) {
	my @map = $input.split("\n")».comb».Array;
	[+] do loop (my $i = 0; $i < Int(@map[0]); ++$i) {
		my $num=0;
		my $sum = 0;
		my $start = 0;
		for @map[^*]»[$i].kv -> $k, $_ {
			if $_ ~~ '#' {
				$sum += $num * ($num + 1) / 2 + $num * (Int(@map) - $num - $start);
				$num = 0;
				$start = $k+1;
			}
			if $_ ~~ 'O' { $num++ }
		}
		$sum += $num * ($num + 1) / 2 + $num * (Int(@map) - $num - $start);
		$sum
	}
}

sub infix:<:=:>($a is rw, $b is rw) {
	($a, $b) = ($b, $a)
}

sub u(@map) {
	loop (my $x = 0; $x < Int(@map[0]); ++$x) {
		my $w = -1;
		loop (my $y = 0; $y < Int(@map); ++$y) {
			my $z = @map[$y][$x];
			if $z ~~ '#' {
				$w = -1;
			}
			if $z ~~ '.' {
				$w = $y if $w == -1;
			}
			if $z ~~ 'O' && $w > -1 {
				@map[$y][$x] :=: @map[$w++][$x];
			}
		}
	}
}

sub l(@map) {
	loop (my $y = 0; $y < Int(@map); ++$y) {
		my $w = -1;
		loop (my $x = 0; $x < Int(@map[0]); ++$x) {
			my $z = @map[$y][$x];
			if $z ~~ '#' {
				$w = -1;
			}
			if $z ~~ '.' {
				$w = $x if $w == -1;
			}
			if $z ~~ 'O' && $w > -1 {
				@map[$y][$x] :=: @map[$y][$w++];
			}
		}
	}
}

sub d(@map) {
	loop (my $x = 0; $x < Int(@map[0]); ++$x) {
		my $w = -1;
		loop (my $y = Int(@map) - 1; $y >= 0; --$y) {
			my $z = @map[$y][$x];
			if $z ~~ '#' {
				$w = -1;
			}
			if $z ~~ '.' {
				$w = $y if $w == -1;
			}
			if $z ~~ 'O' && $w > -1 {
				@map[$y][$x] :=: @map[$w--][$x];
			}
		}
	}
}

sub r(@map) {
	loop (my $y = 0; $y < Int(@map); ++$y) {
		my $w = -1;
		loop (my $x = Int(@map[0]) - 1; $x >= 0; --$x) {
			my $z = @map[$y][$x];
			if $z ~~ '#' {
				$w = -1;
			}
			if $z ~~ '.' {
				$w = $x if $w == -1;
			}
			if $z ~~ 'O' && $w > -1 {
				@map[$y][$x] :=: @map[$y][$w--];
			}
		}
	}
}

sub w(@map) {
	my $w = 0;
	loop (my $y = 0; $y < Int(@map); ++$y) {
		loop (my $x = 0; $x < Int(@map[0]); ++$x) {
			if @map[$y][$x] ~~ 'O' {
				$w += Int(@map) - $y
			}
		}
	}
	$w;
}

sub c(@map) {
	u(@map);
	l(@map);
	d(@map);
	r(@map);
}

sub short(@map) {
	$_ = @map.join("|");
	s:g/' '//;
	$_
}

our sub day14part2(Str $input) {
	my @map = $input.split("\n")».comb».Array;
	my %saw;
	%saw{short(@map)} = 0;
	my $k;
	my $p;
	loop ($k = 1; $k <= 1000000000; ++$k) {
		c(@map);
		my $s = short(@map);
		if %saw (cont) $s {
			$p = %saw{$s};
			last;
		}
		%saw{$s} = $k;
	}
	$k = Int((1000000000 - $p) / ($k - $p)) * ($k - $p) + $p;
	loop (; $k < 1000000000; ++$k) {
		c(@map);
	}
	w(@map)
}
