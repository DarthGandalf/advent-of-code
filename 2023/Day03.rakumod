unit module Day03;

our sub part1(Str $input) {
	my @lines = $input.lines.map('.' ~ * ~ '.');
	@lines.unshift('.' x @lines[0].chars);
	@lines.push(@lines[0]);
	[+] do for @lines.kv -> $y, $line {
		my @m = m:g/\d+/ given $line;
		[+] do for @m {
			[@lines[$y - 1, $y, $y + 1]]».substr(.from - 1, .to - .from + 2).join.comb.grep(/<-[\d .]>/) and +$_
		};
	}
}

our sub part2(Str $input) {
	my @lines = $input.lines.map('.' ~ * ~ '.');
	@lines.unshift('.' x @lines[0].chars);
	@lines.push(@lines[0]);
	[+] do for @lines.kv -> $y, $line {
		my @m = m:g/'*'/ given $line;
		[+] do for @m -> $m {
			my @nums;
			loop (my $yy = $y-1; $yy <= $y + 1; ++$yy) {
				if @lines[$yy].substr($m.from, 1) ~~ /\d/ {
					my $left = $m.from;
					--$left while @lines[$yy].substr($left - 1, 1) ~~ /\d/;
					my $right = $m.from;
					++$right while @lines[$yy].substr($right + 1, 1) ~~ /\d/;
					@nums.push(@lines[$yy].substr($left, $right - $left + 1));
					next;
				}
				if @lines[$yy].substr($m.from - 1, 1) ~~ /\d/ {
					my $left = $m.from - 1;
					--$left while @lines[$yy].substr($left - 1, 1) ~~ /\d/;
					@nums.push(@lines[$yy].substr($left, $m.from - $left));
				}
				if @lines[$yy].substr($m.from + 1, 1) ~~ /\d/ {
					my $right = $m.from + 1;
					++$right while @lines[$yy].substr($right + 1, 1) ~~ /\d/;
					@nums.push(@lines[$yy].substr($m.from + 1, $right - $m.from));
				}
			}
			[*] @nums if @nums.elems == 2;
		}
	}
}
