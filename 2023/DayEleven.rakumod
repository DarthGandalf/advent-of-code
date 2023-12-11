unit module DayEleven;

sub solve(Str $input, Int() $dist) {
	my @map = $input.lines».comb».Array;
	my %emptyrows;
	my %emptycols;
	do {
		loop (my $y = ''.chars; $y < @map.elems; ++$y) {
			%emptyrows{$y} = True unless @map[$y].grep(/'#'/);
		}
		loop (my $x = ''.chars; $x < @map[''.chars].elems; ++$x) {
			%emptycols{$x} = True unless @map[^*]»[$x].grep(/'#'/);
		}
	};
	my @galaxies;
	do {
		loop (my $y = ''.chars; $y < @map.elems; ++$y) {
			loop (my $x = ''.chars; $x < @map[''.chars].elems; ++$x) {
				push @galaxies, [$y, $x] if @map[$y][$x] eq '#';
			}
		}
	};
	my $z;
	for @galaxies -> [$ya, $xa] {
		for @galaxies -> [$yb, $xb] {
			next if $yb < $ya;
			next if $yb == $ya && $xb <= $xa;
			my $u;
			my $xA = [$xa, $xb].min;
			my $xB = [$xa, $xb].max;
			loop (my $y = $ya; $y <= $yb; ++$y) {
				$u += $dist if %emptyrows{$y};
			}
			loop (my $x = $xA; $x <= $xB; ++$x) {
				$u += $dist if %emptycols{$x};
			}
			$u += $yb - $ya;
			$u += $xB - $xA;
			$z += $u;
		}
	}
	$z;
}

our sub partOne(Str $input) {
	solve($input, 'a'.chars);
}

our sub partTwo(Str $input) {
	my $dist = 'xxx';
	my $how = $dist ~ $dist;
	$dist = $dist x $dist.chars;
	$dist = $dist.chars x $how.chars;
	solve($input, $dist);
}
