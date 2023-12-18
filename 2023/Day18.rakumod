unit module Day18;

grammar G {
	rule TOP { <dir> <len> '(#' <color> ')' }
	token dir { 'R'|'U'|'L'|'D' }
	token len { \d+ }
	token color { <len2> <dir2> }
	token len2 { \w ** 5 }
	token dir2 { \w }
}

our sub part1(Str $input) {
	my %map;
	my $x = 0;
	my $y = 0;
	%map<0,0> = 1;
	my $minx = 0;
	my $maxx = 0;
	my $miny = 0;
	my $maxy = 0;
	for $input.lines -> $line {
		my $g = G.parse($line);
		my $dx;
		my $dy;
		given $g<dir> {
			when 'R' { $dx=1; $dy=0;  }
			when 'L' { $dx=-1;$dy=0;  }
			when 'U' { $dx=0; $dy=-1; }
			when 'D' { $dx=0; $dy=1;  }
		}
		loop (my $i = 0; $i < $g<len>; ++$i) {
			$x += $dx;
			$y += $dy;
			%map{"$x,$y"} = 1;
			$minx min= $x;
			$maxx max= $x;
			$miny min= $y;
			$maxy max= $y;
		}
	}
	$minx--;
	$miny--;
	$maxx++;
	$maxy++;
	my @queue = "$minx,$miny";
	my %seen;
	%seen{"$minx,$miny"} = 1;
	while @queue {
		($x, $y) = @queue.pop.split(',');
		for $x - 1 .. $x + 1 -> $nx {
			if ($minx <= $nx <= $maxx) {
				for $y - 1 .. $y + 1 -> $ny {
					if $miny <= $ny <= $maxy and %seen{"$nx,$ny"}:!exists and %map{"$nx,$ny"}:!exists {
						%seen{"$nx,$ny"} = 1;
						@queue.push("$nx,$ny");
					}
				}
			}
		}
	}
	for $miny..$maxy -> $y {
		my $line = '';
		for $minx..$maxx -> $x {
			if "$x,$y" (elem) %seen {
				$line ~= 's';
			} else {
				$line ~= '.';
			}
		}
		say $line;
	}
	($maxx - $minx + 1) * ($maxy - $miny + 1) - %seen.elems
}

class Space {
	has @!map;
	has @!x;
	has @!y;

	submethod TWEAK() {
		@!x.push(-Inf);
		@!x.push(0);
		@!x.push(1);
		@!x.push(Inf);
		@!y.push(-Inf);
		@!y.push(0);
		@!y.push(1);
		@!y.push(Inf);
		@!map.push([0, 0, 0]);
		@!map.push([0, 1, 0]);
		@!map.push([0, 0, 0]);
	}

	method ensureSubdivisionX($x) {
		my $index = @!x.pairs.first(*.value >= $x).key;
		return if @!x[$index] == $x;
		@!x.splice($index, 0, $x);
		for @!map -> @row {
			@row.splice($index - 1, 0, @row[$index - 1]);
		}
	}

	method ensureSubdivisionY($y) {
		my $index = @!y.pairs.first(*.value >= $y).key;
		return if @!y[$index] == $y;
		@!y.splice($index, 0, $y);
		# https://github.com/rakudo/rakudo/issues/5500
		my @newrow = @!map[$index - 1].list;
		@!map.splice($index - 1, 0, 0);
		@!map[$index - 1] = @!map[$index].Array;
	}

	method fillVert($x, $ya, $yb) {
		self.ensureSubdivisionX($x);
		self.ensureSubdivisionX($x+1);
		self.ensureSubdivisionY($ya);
		self.ensureSubdivisionY($yb+1);
		my $index = @!x.pairs.first(*.value >= $x).key;
		for @!y.kv -> $i, $y {
			@!map[$i][$index] = 1 if $ya <= $y <= $yb;
		}
	}

	method fillHor($xa, $xb, $y) {
		self.ensureSubdivisionX($xa);
		self.ensureSubdivisionX($xb+1);
		self.ensureSubdivisionY($y);
		self.ensureSubdivisionY($y+1);
		my $index = @!y.pairs.first(*.value >= $y).key;
		for @!x.kv -> $i, $x {
			@!map[$index][$i] = 1 if $xa <= $x <= $xb;
		}
	}

	method debug() {
		say @!x;
		for @!map Z @!y {
			.say
		}
	}

	method flood() {
		my @queue;
		@queue.push([0, 0]);
		my @seen = @!map».map({0;})».Array;
		@seen[0][0] = 1;
		while @queue {
			my ($x, $y) = @queue.pop;
			for $x - 1 .. $x + 1 -> $nx {
				if (0 <= $nx <= @!map[0].end) {
					for $y - 1 .. $y + 1 -> $ny {
						if 0 <= $ny <= @!map.end and @seen[$ny][$nx] == 0 and @!map[$ny][$nx] == 0 {
							@seen[$ny][$nx] = 1;
							@queue.push([$nx, $ny]);
						}
					}
				}
			}
		}
		(@!x[*-2] - @!x[1]) * (@!y[*-2] - @!y[1]) - [+] @seen.kv.map(sub ($y, $row) {
			return 0 if $y == 0;
			return 0 if $y == @!map.elems-1;
			[+] $row.kv.map(sub ($x, $value) {
				return 0 if $value == 0;
				return 0 if $x == 0;
				return 0 if $x == @!map[0].elems-1;
				my $xd = @!x[$x+1] - @!x[$x];
				my $yd = @!y[$y+1] - @!y[$y];
				$xd * $yd
			})
		})
	}
}

our sub part2(Str $input) {
	my $space = Space.new;
	my $x = 0;
	my $y = 0;
	for $input.lines -> $line {
		my $g = G.parse($line);
		my $len = Int('0x' ~ $g<color><len2>);
		given $g<color><dir2> {
			when '0' {
				$space.fillHor($x, $x+$len, $y);
				$x+=$len;
			}
			when '1' {
				$space.fillVert($x, $y, $y+$len);
				$y+=$len;
			}
			when '2' {
				$x-=$len;
				$space.fillHor($x, $x+$len, $y);
			}
			when '3' {
				$y-=$len;
				$space.fillVert($x, $y, $y+$len);
			}
		}
	}
	$space.flood;
}
