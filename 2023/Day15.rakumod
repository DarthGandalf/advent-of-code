unit module Day15;

sub hashfunc(Str $s) {
	my $x = 0;
	for $s.comb {
		$x = (($x + .ord) * 17) % 256;
	}
	$x
}

our sub part1(Str $input) {
	[+] $input.split(',').map(&hashfunc)
}

our sub part2(Str $input) {
	my @boxes = [] xx 256;
	for $input.split(',') {
		if .ends-with('-') {
			my $name = .chop;
			my $hash = hashfunc($name);
			@boxes[$hash] = @boxes[$hash].grep(*.key ne $name).Array;
		} else {
			my $pair = m/^(.*) '=' (.*)$/;
			my ($name, $num) = @$pair;
			$name = ~$name;
			$num = +$num;
			my $done = False;
			my $hash = hashfunc($name);
			for @boxes[$hash].flat {
				if .key eq $name {
					$done = True;
					$_.value = $num;
				}
			};
			unless $done {
				@boxes[$hash].push($name => $num);
			}
		}
	}
	[+] @boxes.kv.map({
		[+] $^b.kv.map(($^a + 1) * (* + 1) * *.value)
	})
}
