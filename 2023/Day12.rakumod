unit module Day12;
use Memoize;

sub solve(@pattern is copy, @counts) {
	unless @pattern {
		return @counts ?? 0 !! 1;
	}
	unless @counts {
		return @pattern.grep(* eq '#') ?? 0 !! 1;
	}
	given @pattern[0] {
		when '.' { solve(@pattern[1..*], @counts) }
		when '#' {
			return 0 if @pattern.elems < @counts[0];
			return 0 if @pattern.elems >= @counts[0] + 1 && @pattern[@counts[0]] eq '#';
			return 0 if @pattern[0..@counts[0]-1].grep(* eq '.');
			solve(@pattern[@counts[0]+1..*], @counts[1..*]);
		}
		when '?' {
			@pattern[0] = '.';
			my $x = solve(@pattern, @counts);
			@pattern[0] = '#';
			my $y = solve(@pattern, @counts);
			$x + $y
		}
	}
}

memoize('solve');

our sub part1(Str $input) {
	[+] $input.lines.map({
		my ($pattern, $counts) = .split(' ');
		my @pattern = $pattern.comb;
		my @counts = $counts.split(',');
		solve(@pattern, @counts);
	})
}

our sub part2(Str $input) {
	[+] $input.lines.map({
		my ($pattern, $counts) = .split(' ');
		$pattern ~= '?';
		$pattern x= 5;
		my @pattern = $pattern.chop.comb;
		my @counts = $counts.split(',');
		@counts = flat(@counts xx 5);
		solve(@pattern, @counts);
	})
}
