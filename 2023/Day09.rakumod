unit module Day09;

sub continueseq(@seq) {
	if @seq.grep(* != 0) {
		my @newseq = (@seq[0..*-2] Z @seq[1..*-1]).map({ $_[1] - $_[0] });
		my $next = continueseq(@newseq);
		@seq[*-1] + $next;
	} else {
		return 0;
	}
}

sub continueseq2(@seq, $indent = '') {
	if @seq.grep(* != 0) {
		say "$indent@seq[]";
		my @newseq = (@seq[0..*-2] Z @seq[1..*-1]).map({ $_[1] - $_[0] });
		my $next = continueseq2(@newseq, $indent ~ '  ');
		say "$indent$next - @seq[]";
		@seq[0] - $next;
	} else {
		return 0;
	}
}

our sub part1(Str $input) {
	[+] $input.lines.map(sub ($line) {
		my @seq = $line.words;
		continueseq(@seq);
	})
}

our sub part2(Str $input) {
	[+] $input.lines.map(sub ($line) {
		my @seq = $line.words;
		continueseq2(@seq);
	})
}
