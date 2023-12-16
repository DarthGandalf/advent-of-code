unit module Day16;

sub bfs($start, @map) {
	#my @vis = $input.lines».comb».Array;
	my %energized;
	my %seen;
	my @queue; # "xpos,ypos,xvec,yvec"
	@queue.push($start);
	sub tryadd($xpos, $ypos, $xvec, $yvec) {
		my $str = "$xpos,$ypos,$xvec,$yvec";
		return if %seen{$str};
		%seen{$str} = 1;
		@queue.push($str);
	}
	while @queue {
		my ($xpos, $ypos, $xvec, $yvec) = @queue.pop.split(',');
		#say ($xpos, $ypos, $xvec, $yvec, @queue);
		$xpos += $xvec;
		$ypos += $yvec;
		next if $ypos < 0;
		next if $xpos < 0;
		next if $ypos >= @map.elems;
		next if $xpos >= @map[0].elems;
		%energized{"$xpos,$ypos"} = 1;
		#@vis[$ypos][$xpos] = '#';
		#say @vis».join('').join("\n");
		#say "";
		given @map[$ypos][$xpos] {
			when '.' {
				tryadd($xpos, $ypos, $xvec, $yvec);
			}
			when '/' {
				tryadd($xpos, $ypos, -$yvec, -$xvec);
			}
			when '\\' {
				tryadd($xpos, $ypos, $yvec, $xvec);
			}
			when '-' {
				if $yvec == 0 {
					tryadd($xpos, $ypos, $xvec, $yvec);
				} else {
					tryadd($xpos, $ypos, 1, 0);
					tryadd($xpos, $ypos, -1, 0);
				}
			}
			when '|' {
				if $xvec == 0 {
					tryadd($xpos, $ypos, $xvec, $yvec);
				} else {
					tryadd($xpos, $ypos, 0, 1);
					tryadd($xpos, $ypos, 0, -1);
				}
			}
		}
	}
	%energized.elems
}

our sub part1(Str $input) {
	my @map = $input.lines».comb».Array;
	bfs("-1,0,1,0", @map);
}

our sub part2(Str $input) {
	my @map = $input.lines».comb».Array;
	my $xmax = @map.elems;
	my $ymax = @map[0].elems;
	my @answers;
	@answers.append(@map.keys.race.map({ bfs("-1,$_,1,0", @map) }));
	say "A";
	@answers.append(@map.keys.race.map({ bfs("$xmax,$_,-1,0", @map) }));
	say "A";
	@answers.append(@map[0].keys.race.map({ bfs("$_,-1,0,1", @map) }));
	say "A";
	@answers.append(@map[0].keys.race.map({ bfs("$_,$ymax,0,-1", @map) }));
	@answers.max
}
