unit module Day08;

sub draw(Str $input) {
	my @screen = ['  ' xx 50] xx 6;
	for $input.lines -> $line {
		my @nums = $line ~~ m:g/\d+/;
		given $line.substr(1, 1) {
			when 'e' {
				@screen[^@nums[1];^@nums[0]] »=» '##';
			}
			when 'o' {
				given $line.substr(7, 1) {
					when 'r' {
						@screen[@nums[0];*] = @screen[@nums[0];*].rotate(-@nums[1]);
					}
					when 'c' {
						@screen[*;@nums[0]] = @screen[*;@nums[0]].rotate(-@nums[1]);
					}
				}
			}
		}
	}
	@screen
}

our sub part1(Str $input) {
	draw($input).map({@$_}).flat.grep('##').elems
}

our sub part2(Str $input) {
	draw($input).map("\n" ~ *.join).join
}
