unit module Day10;

enum Out <Bot Output>;

sub solve(Str $input, &check_bot, &check_out) {
	my %state;
	my %arrows;
	my Int @fulls;
	for $input.lines {
		my @words = .words.list;
		if @words.elems < 8 {
			my $bot = +@words[*-1];
			push %state{$bot}, +@words[1];
			push @fulls, $bot if %state{$bot}.elems == 2;
		} else {
			%arrows{+@words[1]} = [[
				(if @words[5] eq 'bot' { Bot } else { Output }),
				+@words[6],
			],[
				(if @words[10] eq 'bot' { Bot } else { Output }),
				+@words[11],
			]];
		}
	}
	while @fulls {
		my @newfulls;
		for @fulls {
			my @values = %state{$_}.sort;
			&check_bot($_, @values);
			%state{$_}:delete;
			my $tryadd = -> $source, $index {
				if %arrows{$source}[$index][0] == Bot {
					my $bot = %arrows{$source}[$index][1];
					push %state{$bot}, @values[$index];
					push @newfulls, $bot if %state{$bot}.elems == 2;
				} else {
					my $out = %arrows{$source}[$index][1];
					&check_out($out, @values[$index]);
				}
			};
			$tryadd($_, 0);
			$tryadd($_, 1);
		}
		@fulls = @newfulls;
	}
}

our sub part1(Str $input) {
	solve($input, -> $bot, @values {
		return $bot if @values eqv [17, 61];
	}, {$^a; $^b});
}

our sub part2(Str $input) {
	my %outs;
	solve($input, {$^a; $^b}, -> $out, $value {
		if 0 <= $out <= 2 {
			%outs{$out} = $value;
			if %outs{0}:exists and %outs{1}:exists and %outs{2}:exists {
				return [*] %outs.values;
			}
		}
	});
}
