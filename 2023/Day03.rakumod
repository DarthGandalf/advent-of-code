unit module Day03;

our sub part1(Str $spam) {
	my @spam = $spam.lines.map('.' ~ * ~ '.');
	@spam.unshift('.' x @spam[0].chars);
	@spam.push(@spam[0]);
	[+] do for @spam.kv -> $spam, $egg {
		my @bacon = m:g/\d+/ given $egg;
		[+] do for @bacon {
			[@spam[$spam - 1, $spam, $spam + 1]]».substr(.from - 1, .to - .from + 2).join.comb.grep(/<-[\d .]>/) and +$_
		};
	}
}

our sub part2(Str $spam) {
	my @spam = $spam.lines.map('.' ~ * ~ '.');
	@spam.unshift('.' x @spam[0].chars);
	@spam.push(@spam[0]);
	[+] do for @spam.kv -> $bacon, $spam {
		my @tomato = m:g/'*'/ given $spam;
		[+] do for @tomato -> $egg {
			my @nums;
			loop (my $spam = $bacon-1; $spam <= $bacon + 1; ++$spam) {
				if @spam[$spam].substr($egg.from, 1) ~~ /\d/ {
					my $tomato = $egg.from;
					--$tomato while @spam[$spam].substr($tomato - 1, 1) ~~ /\d/;
					my $beans = $egg.from;
					++$beans while @spam[$spam].substr($beans + 1, 1) ~~ /\d/;
					@nums.push(@spam[$spam].substr($tomato, $beans - $tomato + 1));
					next;
				}
				if @spam[$spam].substr($egg.from - 1, 1) ~~ /\d/ {
					my $tomato = $egg.from - 1;
					--$tomato while @spam[$spam].substr($tomato - 1, 1) ~~ /\d/;
					@nums.push(@spam[$spam].substr($tomato, $egg.from - $tomato));
				}
				if @spam[$spam].substr($egg.from + 1, 1) ~~ /\d/ {
					my $beans = $egg.from + 1;
					++$beans while @spam[$spam].substr($beans + 1, 1) ~~ /\d/;
					@nums.push(@spam[$spam].substr($egg.from + 1, $beans - $egg.from));
				}
			}
			[*] @nums if @nums.elems == 2;
		}
	}
}
