unit module Day04;

grammar G {
	regex TOP { <name> '-' <id> '[' <checksum> ']' }
	regex name { <[\w] + [-]>+ }
	token id { \d+ }
	token checksum { \w ** 5 }
}

sub chk(Str $name is copy) {
	s:g/'-'// given $name;
	$name.comb.Bag.pairs.sort({[-$^a.value, $a.key]})[0..4].map(*.key).join;
}

our sub part1(Str $input) {
	sum race for $input.lines {
		# This var seems useless but actually it's not:
		# https://github.com/rakudo/rakudo/commit/8542ad2116
		# It can be deleted in raku 6.e when it arrives
		my $/;
		my $room = G.parse($_);
		if chk(~$room<name>) eq $room<checksum> { $room<id> } else { 0 }
	};
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
