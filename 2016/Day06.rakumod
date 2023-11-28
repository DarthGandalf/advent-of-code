unit module Day06;

our sub part1(Str $input) {
	my @letters = $input.lines.map(*.comb.list);
	@letters[0].kv.map(-> $i, $l {
		@letters[*;$i].Bag.antipairs.sort[*-1].value
	}).join
}

our sub part2(Str $input) {
	my @letters = $input.lines.map(*.comb.list);
	@letters[0].kv.map(-> $i, $l {
		@letters[*;$i].Bag.antipairs.sort[0].value
	}).join
}
