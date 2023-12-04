unit module Day04;

our sub part1(Str $i) {
	[+](2 «**«($i.lines».split(':')»[1]».split('|')».map(*.split(' ',:skip-empty)).map({[∩] $_})».elems.grep(*>0)))/2
}

our sub part2(Str $i) {
	my int @a;for $i.lines.reverse {my ($c,$r)=.split(':');s/Card\s*// given $c;@a[$c]=1+[+] @a[$c+1..$c+([∩] $r.split('|')».split(' ',:skip-empty)).elems];};[+] @a
}
