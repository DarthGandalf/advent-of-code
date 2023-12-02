unit module Day12;

sub run(Int %reg is copy, Str $input) {
	my @program = $input.lines».words».Array;
	my Int $pc = 0;
	sub value(Str $what) {
		given $what {
			when /\d+/ { +$_ }
			default { %reg{$_} };
		};
	}
	while 0 <= $pc < @program.elems {
		#say "pc=$pc: {@program[$pc]}; {%reg.raku}";
		my $w = @program[$pc++];
		given $w[0] {
			when 'cpy' { %reg{$w[2]} = value($w[1]) }
			when 'inc' {
				if ($pc + 1 < @program.elems) and (@program[$pc][0] eq 'dec') and (@program[$pc][1] eq @program[$pc+1][1]) and (@program[$pc+1][0] eq 'jnz') and (@program[$pc+1][2] == -2) {
					%reg{$w[1]} += %reg{@program[$pc][1]};
					%reg{@program[$pc][1]} = 1;
				} else {
					++%reg{$w[1]}
				}
			}
			when 'dec' { --%reg{$w[1]} }
			when 'jnz' {
				$pc += value($w[2]) - 1 if value($w[1]);
			}
		}
	}
	%reg<a>
}

our sub part1(Str $input) {
	my Int %reg = (:0a, :0b, :0c, :0d);
	run(%reg, $input);
}

our sub part2(Str $input) {
	my Int %reg = (:0a, :0b, :1c, :0d);
	run(%reg, $input);
}
