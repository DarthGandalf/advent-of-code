unit module Day17;
use Astar;

sub solve(Str $input, Int $min, Int $max) {
	my @map = $input.lines».comb».Int».Array;
	my $graph = Astar.new(
		distance   => sub (@v, @w) {
			#say "Dist between " ~ @v ~ ', ' ~ @v;
			if @v[0].im == @map.end && @v[0].re == @map[0].end {
				return 0;
			}
			@map[@w[0].im][@w[0].re];
		},
		successors => sub (@v) {
			#say "Succ of " ~ @v;
			my ($pos, $dir, $len) = @v;
			if $len == 0 {
				return [
					[1+0i, 1+0i, 1],
					[0+1i, 0+1i, 1],
				];
			}
			my @succ;
			if $pos.im == @map.end && $pos.re == @map[0].end && $len >= $min {
				@succ.push([$pos, 0+0i, 0]);
				return @succ;
			}
			if $len < $max {
				@succ.push([$pos + $dir, $dir, $len + 1]);
			}
			if $len >= $min {
				$dir *= 1i;
				@succ.push([$pos + $dir, $dir, 1]);
				$dir *= -1;
				@succ.push([$pos + $dir, $dir, 1]);
			}
			return @succ.grep({ my $p = $_[0]; 0 <= $p.im <= @map.end && 0 <= $p.re <= @map[0].end })
		},
		heuristic  => -> @v, @w {
			#say "Heur for " ~ @v;
			@map.end - @v[0].im + @map[0].end - @v[0].re
		},
		identifier => -> @v #{ my $x=@v.join(',');say "Id: $x";$x },
			{ @v.join(',') }
	);
	my @path = $graph.best-path([0+0i, 0+0i, 0], [Complex.new(@map[0].end, @map.end), 0+0i, 0]);
	@path[1..*-2].map({@map[$_[0].im][$_[0].re]}).sum
}

our sub part1(Str $input) {
	solve($input, 0, 3);
}

our sub part2(Str $input) {
	solve($input, 4, 10);
}
