unit module Day23;

our sub part1(Str $input, :$part2 = False) {
	my @map = $input.lines».comb».Array;
	@map.splice(0, 0, [['#' xx @map[0].elems], ['#' xx @map[0].elems]]);
	@map.splice(*, 0, [['#' xx @map[0].elems], ['#' xx @map[0].elems]]);
	@map[1][1] = '.';
	@map[2][1] = 'v';
	@map[*-3][*-2] = 'v';
	@map[*-2][*-2] = '.';
	my @Vs = @map».map({-1;})».Array;
	my @V;
	@Vs[1][1] = 0;
	@Vs[@map.end - 1][@map[0].end - 1] = 1;
	@V.push([1, 1]);
	@V.push([@map.end-1, @map[0].end - 1]);
	for 1..@map.end-1 -> $y {
		for 1..@map[0].end-1 -> $x {
			next if @map[$y][$x] ne '.';
			if [[1, 0], [0, 1], [-1, 0], [0, -1]].map(-> [$dy, $dx] { [$y+$dy, $x+$dx] }).grep(-> [$y, $x] { @map[$y][$x] (elem) ['v', '>', '<', '^'] }).elems >= 2 {
				@V.push([$y, $x]);
				@Vs[$y][$x] = @V.end;
			}
		}
	}
	my @E = [[] xx @V.elems];
	for @V.kv -> $i, $start {
		sub find_end($pos) {
			my %seen;
			%seen{$pos} = 1;
			my @queue;
			@queue.push($pos);
			while @queue {
				my ($y, $x) = @queue.shift;
				for [[1, 0], [0, 1], [-1, 0], [0, -1]] -> [$dy, $dx] {
					my $nx = $dx + $x;
					my $ny = $dy + $y;
					next if %seen{$[$ny, $nx]}:exists;
					given @map[$ny][$nx] {
						when '.' {
							@queue.push([$ny, $nx]);
							%seen{$[$ny, $nx]} = 1;
							next;
						}
						when '#' { next; }
					}
					$nx += $dx;
					$ny += $dy;
					my $v = @Vs[$ny][$nx];
					die if $v < 0;
					@E[$i].push({to=>$v, len=>%seen.elems+3}) if $v != $i;
					@E[$v].push({to=>$i, len=>%seen.elems+3}) if $v != $i and $part2;
				}
			}
		}
		if @map[$start[0]][$start[1] + 1] eq '>' {
			find_end([$start[0], $start[1] + 2]);
		}
		if @map[$start[0]][$start[1] - 1] eq '<' {
			find_end([$start[0], $start[1] - 2]);
		}
		if @map[$start[0] + 1][$start[1]] eq 'v' {
			find_end([$start[0] + 2, $start[1]]);
		}
		if @map[$start[0] - 1][$start[1]] eq '^' {
			find_end([$start[0] - 2, $start[1]]);
		}
	}
	my $max = 0;
	my %seen;
	%seen<0> = 1;
	sub dfs($v, $already) {
		if $v == 1 {
			$max max= $already;
			return;
		}
		for @(@E[$v]) -> $e {
			my $to = $e<to>;
			next if %seen{$to}:exists;
			%seen{$to} = 1;
			dfs($e<to>, $already + $e<len>);
			%seen{$to}:delete;
		}
	}
	dfs(0, -2);
	$max
}

our sub part2(Str $input) {
	part1($input, :part2);
}
