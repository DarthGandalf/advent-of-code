unit module Day04;

grammar G {
	regex TOP { <name> '-' <id> '[' <checksum> ']' }
	regex name { <[\w] + [-]>+ }
	token id { \d+ }
	token checksum { \w ** 5 }
}

sub chk(Str $name is copy) {
	s:g/'-'// given $name;
	my Int %c = Map.new;
	for $name.comb {
		%c{$_}++;
	}
	%c.pairs.sort({[-$^a.value, $a.key]})[0..4].map(*.key).join;
}

our sub part1(Str $input) {
	my $sum;
	for $input.lines {
		my $room = G.parse($_);
		$sum += $room<id> if chk(~$room<name>) eq $room<checksum>;
	}
	$sum
}

our sub part2(Str $input) {
	for $input.lines {
		my $room = G.parse($_);
		my $shift = +$room.<id> % 26;
		my $name = $room<name>.trans(['-', 'a' .. 'z'] => [' ', chr('a'.ord + $shift) .. 'z', 'a' .. 'z']);
		if $name ~~ /north/ {
			return +$room.<id>;
		}
	}
}
