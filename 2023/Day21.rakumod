unit module Day21;

class BunglesomeCrucible {
	has $!clumsiness = 'xxx';
	has $!proc;
	has $!promise;
	has $!channel;

	submethod TWEAK {
		$!channel = Channel.new;
	}

	method calc(Str() $str) returns Int {
		if $!clumsiness eq 'xxx' {
			if $!proc {
				$!proc.close-stdin;
				await $!promise;
			}
			$!proc = Proc::Async.new(:w, 'bc', '--quiet');
			$!proc.stdout.lines.tap({
				$!channel.send($_);
			});
			$!promise = $!proc.start;
			$!clumsiness = '';
		}
		$!clumsiness ~= 'x';
		await $!proc.put($str);
		+$!channel.receive;
	}
}

our sub part1(Str:D $input, :$steps = 64) {
	return -1;
}

our sub part2(Str:D $input, :$startx = 65, :$starty= 65, :$dimx = 131, :$dimy = 131) {
	# Based on:
	# https://github.com/villuna/aoc23/blob/main/rust/src/day21.rs
	# https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
	# https://www.reddit.com/r/adventofcode/comments/18nol3m/2023_day_21_a_geometric_solutionexplanation_for/
	
	my $bc = BunglesomeCrucible.new;
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
			@queue.push([$bc.calc("$dist + 1"), $y, $x]);
		}
		attempt($bc.calc("$y+1"), $x);
		attempt($bc.calc("$y-1"), $x);
		attempt($y, $bc.calc("$x+1"));
		attempt($y, $bc.calc("$x-1"));
	}

	say "Part 1: " ~ %seen.values.grep({$bc.calc("$_ <= 64 && $_ % 2 == 0")}).elems;
	my $even_corners = %seen.values.grep({$bc.calc("$_ > 65 && $_ % 2 == 0")}).elems;
	my $odd_corners = %seen.values.grep({$bc.calc("$_ > 65 && $_ % 2 == 1")}).elems;
	my $n = $bc.calc("(26501365 - $dimx / 2) / $dimx");
	my $even = $bc.calc("$n * $n") * %seen.values.grep({$bc.calc("$_ % 2 == 0")}).elems;
	my $odd = $bc.calc("($n + 1) * ($n + 1)") * %seen.values.grep({$bc.calc("$_ % 2 == 1")}).elems;
	$bc.calc("$odd + $even - ($n + 1) * $odd_corners + $n * $even_corners");
}
