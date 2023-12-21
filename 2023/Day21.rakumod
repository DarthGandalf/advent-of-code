unit module Day21;

our sub part1(Str:D $input, :$steps = 64) {
	my @map = $input.lines».comb».Array;
	my @queue;
	my %seen;
	for @map.kv -> $y, @line {
		for @line.kv -> $x, $c {
			if $c eq 'S' {
				@queue.push([$y, $x]);
			}
		}
	}
	for ^$steps {
		my @Q;
		my %seen;
		sub attempt($y, $x) {
			return if $y < 0;
			return if $x < 0;
			return if $y > @map.end;
			return if $x > @map[0].end;
			return if @map[$y][$x] eq '#';
			return if %seen{"$y,$x"}:exists;
			%seen{"$y,$x"} = 1;
			@Q.push([$y, $x]);
		}
		for @queue -> [$y, $x] {
			attempt($y+1, $x);
			attempt($y-1, $x);
			attempt($y, $x+1);
			attempt($y, $x-1);
		}
		@queue = @Q;
	}
	@queue.elems
}

our sub part2(Str:D $input, :$startx = 65, :$starty= 65, :$dimx = 131, :$dimy = 131) {
	# Based on:
	# https://github.com/villuna/aoc23/blob/main/rust/src/day21.rs
	# https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
	# https://www.reddit.com/r/adventofcode/comments/18nol3m/2023_day_21_a_geometric_solutionexplanation_for/
	
	my @map = $input.lines».comb».Array;
	my @queue;
	@queue.push([0, $starty, $startx]);
	my %seen;
	while @queue {
		my ($dist, $y, $x) = @queue.shift;
		next if %seen{"$y,$x"}:exists;
		%seen{"$y,$x"} = $dist;
		sub attempt($y, $x) {
			return if $y < 0;
			return if $x < 0;
			return if $y > @map.end;
			return if $x > @map[0].end;
			return if @map[$y][$x] eq '#';
			return if %seen{"$y,$x"}:exists;
			@queue.push([$dist + 1, $y, $x]);
		}
		attempt($y+1, $x);
		attempt($y-1, $x);
		attempt($y, $x+1);
		attempt($y, $x-1);
	}

	#part1: %seen.values.grep(* <= 64).grep(* % 2 == 0).elems
	my $even_corners = %seen.values.grep(* > 65).grep(* % 2 == 0).elems;
	my $odd_corners = %seen.values.grep(* > 65).grep(* % 2 == 1).elems;
	my $n = round((26501365 - $dimx / 2) / $dimx);
	my $even = $n²;
	my $odd = ($n + 1)²;
	$odd * %seen.values.grep(* % 2 == 1).elems +
	$even * %seen.values.grep(* % 2 == 0).elems -
	($n + 1) * $odd_corners +
	$n * $even_corners;
}
