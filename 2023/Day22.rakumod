unit module Day22;

our sub part1(Str $input) {
	my @bricks = $input.lines.map({
		my @a = +<<m:g/\d+/;
		@a
	});
	my $maxx = @bricks>>[0, 3].flat.max + 1;
	my $maxy = @bricks>>[1, 4].flat.max + 1;
	my @map = [0 xx $maxy] xx $maxx;
	my @br = @bricks.sort({ min($_[2], $_[5]) });
	my %which;
	my @whereD;
	my @whereU;
	for @br.kv -> $i, @b {
		my $z;
		my $height = abs(@b[2] - @b[5]) + 1;
		for @b[0, 3].min .. @b[0, 3].max -> $x {
			for @b[1, 4].min .. @b[1, 4].max -> $y {
				$z max= @map[$x][$y];
			}
		}
		for @b[0, 3].min .. @b[0, 3].max -> $x {
			for @b[1, 4].min .. @b[1, 4].max -> $y {
				@map[$x][$y] = $z + $height;
				for $z .. $z+$height-1 -> $z is copy {
					$z++;
					%which{"$x,$y,$z"} = $i;
				}
			}
		}
		@whereD.push($z);
		@whereU.push($z + $height);
	}
	my $answer = 0;
	BRICK: for @br.kv -> $i, @b {
		my $above = Set.new;
		my $z = @whereU[$i] + 1;
		for @b[0, 3].min .. @b[0, 3].max -> $x {
			for @b[1, 4].min .. @b[1, 4].max -> $y {
				$above ∪= Set.new(%which{"$x,$y,$z"}) if %which{"$x,$y,$z"}:exists;
			}
		}
		for $above.keys -> $j {
			my $z = @whereD[$j];
			my $falls = 1;
			for @br[$j][0, 3].min .. @br[$j][0, 3].max -> $x {
				for @br[$j][1, 4].min .. @br[$j][1, 4].max -> $y {
					$falls = 0 if %which{"$x,$y,$z"}:exists and %which{"$x,$y,$z"} != $i;
				}
			}
			next BRICK if $falls;
		}
		$answer++;
	}
	$answer
}

our sub part2(Str $input) {
}

 # 521 is too low
