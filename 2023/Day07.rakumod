unit module Day07;

our sub part1(Str $input) {
	[+] $input.lines.sort(-> $line {
		my ($h, $bid) = $line.words;
		my $type = do given $h.comb.sort.join {
			when /(.) $0 $0 $0 $0/ { 7000 }
			when /(.) $0 $0 $0/ { 6000 }
			when /(.) $0 $0 (.) $1/ { 5000 }
			when /(.) $0 (.) $1 $1/ { 5000 }
			when /(.) $0 $0/ { 4000 }
			when /(.) $0 .? (.) $1/ { 3000 }
			when /(.) $0/ { 2000 }
			default { 1000 }
		};
		my @cards = ($type, $h.comb.map({
			when 'T' { 10 }
			when 'J' { 11 }
			when 'Q' { 12 }
			when 'K' { 13 }
			when 'A' { 14 }
			default { +$_ }
		})).flat;
		@cards
	}).kv.map(-> $num, $line {
		my ($h, $bid) = $line.words;
		($num + 1) * $bid
	})
}

our sub part2(Str $input) {
	[+] $input.trans('J' => '1').lines.sort(-> $line {
		my ($h, $bid) = $line.words;
		my $type = do given $h.comb.sort.join {
			when /(.) $0 $0 $0 $0/ { 7000 }
			when /'1' (.) $0 $0 $0/ { 7000 }
			when /'11' (.) $0 $0/ { 7000 }
			when /'111' (.) $0/ { 7000 }
			when /'1111'/ { 7000 }
			when /(.) $0 $0 $0/ { 6000 }
			when /'1' .* (.) $0 $0/ { 6000 }
			when /'11' .* (.) $0/ { 6000 }
			when /'111'/ { 6000 }
			when /(.) $0 $0 (.) $1/ { 5000 }
			when /(.) $0 (.) $1 $1/ { 5000 }
			when /'1' (.) $0 (.) $1/ { 5000 }
			when /(.) $0 $0/ { 4000 }
			when /'1' .* (.) $0/ { 4000 }
			when /'11'/ { 4000 }
			when /(.) $0 .? (.) $1/ { 3000 }
			when /(.) $0/ { 2000 }
			when /'1'/ { 2000 }
			default { 1000 }
		};
		my @cards = ($type, $h.comb.map({
			when 'T' { 10 }
			when 'Q' { 12 }
			when 'K' { 13 }
			when 'A' { 14 }
			default { +$_ }
		})).flat;
		@cards
	}).kv.map(-> $num, $line {
		my ($h, $bid) = $line.words;
		($num + 1) * $bid
	})
}
