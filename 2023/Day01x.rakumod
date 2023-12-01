unit module Day01x;

our sub part1(Str $input) {
	sum $input.lines.map({
		my $a = ~m/^ \D* (\d) /[0];
		my $b = ~m/ (\d) \D* $/[0];
		"$a$b"
	})
}

sub translate(Str $x) {
	given $x {
		when 'one' {1}
		when 'two' {2}
		when 'three' {3}
		when 'four' {4}
		when 'five' {5}
		when 'six' {6}
		when 'seven' {7}
		when 'eight' {8}
		when 'nine' {9}
		default {$x}
	}
}

our sub part2(Str $input) {
	my $fname = m/^ (\S+) / given $?FILE;
	$_ = slurp $fname;
	if / 'V' 'A' 'R' / {
		my @nums = m:g/ 'wh' 'en' \s "'" (\w+) "'" /.map(~*[0]);
		s/'VA' 'R1'/{@nums.join('||')}/;
		s/'VA' 'R2'/{@nums.join('||').flip}/;
		spurt $fname, $_;
		EVALFILE $fname;
		return Day01x::Day01x::part2($input);
	}
	sum $input.lines.map({
		my regex digit { \d || VAR1 }
		my regex antidigit { \d || VAR2 }
		my $a = translate(~m/ <&digit> /);
		my $b = translate((~m/ <&antidigit> /).flip) given .flip;
		"$a$b"
	})
}
