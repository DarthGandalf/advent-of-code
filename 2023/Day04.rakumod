unit module Day04;

our sub part1(Str $input) {
	[+](2 «**«($input.lines».split(':')»[1]».split('|')».map(*.split(' ',:skip-empty)).map({[∩] $_})».elems.grep(*>0) »-»1))
}

our sub part2(Str $input) {
}
