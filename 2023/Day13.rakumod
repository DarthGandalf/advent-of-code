unit module Day13;

sub dist(@a, @b) {
	(@a.join.comb Z @b.join.comb).grep({$^a[0] ne $a[1]}).elems
}

sub solve(Str $input, Int $dist) {
	[+] $input.split("\n\n").map(sub ($map) {
		my @map = $map.lines».comb».Array;
		for 1..@map.elems-1 -> $vert {
			my $len = min($vert, @map.elems-$vert);
			return 100 * $vert if dist(@map[$vert - $len .. $vert - 1], @map[$vert .. $vert + $len - 1].reverse.list) == $dist;
		}
		for 1..@map[0].elems-1 -> $hor {
			my $len = min($hor, @map[0].elems-$hor);
			return $hor if dist(@map[^*]»[$hor - $len .. $hor - 1], @map[^*]»[$hor .. $hor + $len - 1]».reverse».list) == $dist;
		}
	});
}

our sub part1(Str $input) {
	solve($input, 0)
}

our sub part2(Str $input) {
	solve($input, 1)
}
