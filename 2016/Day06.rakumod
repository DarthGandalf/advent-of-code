unit module Day06;

our sub part1(Str $input) {
	my @letters = $input.lines.map(*.comb.list);
	@letters[0].keys.map(-> $i {
		@letters[*;$i].Bag.antipairs.max.value
	}).join
}

our sub part2(Str $input) {
	my @letters = $input.lines.map(*.comb.list);
	@letters[0].keys.map(-> $i {
		@letters[*;$i].Bag.antipairs.min.value
	}).join
}
