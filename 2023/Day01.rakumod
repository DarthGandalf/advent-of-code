unit module Day01;

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
	sum $input.lines.map({
		my regex digit { \d || one || two || three || four || five || six || seven || eight || nine }
		my regex antidigit { \d || eno || owt || eerht || ruof || evif || xis || neves || thgie || enin }
		my $a = translate(~m/ <&digit> /);
		my $b = translate((~m/ <&antidigit> /).flip) given .flip;
		"$a$b"
	})
}
