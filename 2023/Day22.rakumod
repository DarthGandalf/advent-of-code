unit module Day22;

sub deps(Str $input) {
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
	my @heldby;
	my @holding;
	for @br.kv -> $i, @b {
		my $above = Set.new;
		my $below = Set.new;
		my $za = @whereU[$i] + 1;
		my $zb = @whereD[$i];
		for @b[0, 3].min .. @b[0, 3].max -> $x {
			for @b[1, 4].min .. @b[1, 4].max -> $y {
				$above ∪= Set.new(%which{"$x,$y,$za"}) if %which{"$x,$y,$za"}:exists;
				$below ∪= Set.new(%which{"$x,$y,$zb"}) if %which{"$x,$y,$zb"}:exists;
			}
		}
		$below ∪= Set.new(-1) if $zb == 0;
		@heldby.push($below);
		@holding.push($above);
	}
	return {
		heldby => @heldby,
		holding => @holding
	};
}

our sub part1(Str $input) {
	my %input = deps($input);
	my @heldby = @(%input<heldby>);
	my @holding = @(%input<holding>);
	my $answer = 0;
	BRICK: for @holding.keys -> $i {
		for @holding[$i].keys -> $j {
			next BRICK if @heldby[$j].elems == 1;
		}
		$answer++;
	}
	$answer;
}

our sub part2(Str $input) {
	my %input = deps($input);
	my @heldby = @(%input<heldby>);
	my @holding = @(%input<holding>);
	my $answer = 0;
	for @holding.keys -> $i {
		my $gone = Set.new($i);
		for @holding.keys -> $j {
			if @heldby[$j] ⊆ $gone {
				$gone ∪= Set.new($j);
			}
		}
		$answer += $gone.elems - 1;
	}
	$answer;
}
