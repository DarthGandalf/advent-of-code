unit module Day10;

sub parse(Str $input) {
	$input.lines.map(*.comb).deepmap({
		when '.' {{e=>1}}
		when 'S' {{S=>1}}
		when '7' {{xm=>1, yp=>1}}
		when 'L' {{xp=>1, ym=>1}}
		when 'J' {{xm=>1, ym=>1}}
		when 'F' {{xp=>1, yp=>1}}
		when '-' {{xp=>1, xm=>1}}
		when '|' {{yp=>1, ym=>1}}
	});
}

sub find_loop(@symbols) {
	my ($sx, $sy);
	for @symbols.kv -> $y, $line {
		for $line.kv -> $x, $c {
			if $c<S> {
				$sx = $x;
				$sy = $y;
			}
		}
	}
	my @loop;
	push @loop, [$sx, $sy];
	for -1..+1 -> $dy {
		for -1..+1 -> $dx {
			next if $dy != 0 && $dx != 0;
			next if $dy == 0 && $dx == 0;
			my $check;
			if $dx == 1 {
				$check = 'xm';
			} elsif $dx == -1 {
				$check = 'xp';
			} elsif $dy == 1 {
				$check = 'ym';
			} else {
				$check = 'yp';
			}
			my $wx = $sx+$dx;
			my $wy = $sy+$dy;
			my $px = $sx;
			my $py = $sy;
			if @symbols[$wy][$wx]{$check} {
				while $wx != $sx || $wy != $sy {
					push @loop, [$wx, $wy];
					#say ["w=$wx,$wy", "p=$px,$py", @symbols[$wy][$wx]];
					for <xm xp ym yp> -> $check {
						next unless @symbols[$wy][$wx]{$check};
						my $nx = $wx;
						my $ny = $wy;
						given $check {
							when 'xm' {--$nx}
							when 'xp' {++$nx}
							when 'ym' {--$ny}
							when 'yp' {++$ny}
						}
						if $nx != $px || $ny != $py {
							$px = $wx;
							$py = $wy;
							$wx = $nx;
							$wy = $ny;
							last;
						}
					}
				}
				return @loop;
			}
		}
	}
}

our sub part1(Str $input) {
	find_loop(parse($input)).elems / 2
}
our sub part2(Str $input) {
	my @symbols = parse($input);
	my @loop = find_loop(@symbols);
	@loop.push(@loop[0]);
	my @map;
	for ^(@symbols.elems*2+2) {
		@map.push(('.' xx (@symbols[0].elems*2+2)).Array);
	}
	for @loop.rotor(2 => -1) -> [[$x, $y], [$xx, $yy]] {
		@map[1+$y*2][1+$x*2] = '#';
		@map[1+$y+$yy][1+$x+$xx] = '#';
	}
	my @queue;
	push @queue, [0, 0];
	@map[0][0] = 'o';
	while @queue {
		my ($x, $y) = @queue.shift;
		for $x-1..$x+1 -> $x {
			for $y-1..$y+1 -> $y {
				if $y >= 0 && $y < @map.elems && $x >= 0 && $x < @map[0].elems && @map[$y][$x] eq '.' {
					push @queue, [$x, $y];
					@map[$y][$x] = 'o';
				}
			}
		}
	}
	my $answer = 0;
	for @symbols.kv -> $y, $line {
		for $line.kv -> $x, $c {
			if @map[1+$y*2][1+$x*2] eq '.' {
				@map[1+$y*2][1+$x*2] = 'X';
				$answer++;
			}
		}
	}
	for @map {
		say $_.join
	}
	$answer;
}
