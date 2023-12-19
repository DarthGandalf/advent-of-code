unit module Day19;

class Rule {
	has $.dest;
	has &.cond;
	has $.letter;

	method match(%part) {
		(&.cond)(%part{$.letter})
	}
}

class Workflow {
	has @.rules;

	method process(%part) {
		for @.rules -> $r {
			return $r.dest if $r.match(%part);
		}
	}

	method fork(%parter) {
		my @next;
		%parter = %parter.clone;
		for @.rules -> $r {
			my %newparter = %parter.clone;
			# This should intersect ranges to get a range instead of grepping through array, but this is fast enough as is, so whatever
			%newparter{$r.letter} = %newparter{$r.letter}.grep($r.cond).Array;
			@next.push({
				at => $r.dest,
				parter => %newparter;
			});
			%parter{$r.letter} = %parter{$r.letter}.grep({ !($r.cond)($_) }).Array;
		}
		@next;
	}
}

sub workflows(Str $workflows) {
	$workflows.split("\n").map: {
		my ($name, $rest) = .chop.split('{');
		my @rules = $rest.split(',').map({
			my @r = .split(':');
			my &cond;
			my $letter = 'x';
			my $way = '>';
			my $rhs = -1;
			if @r.end {
				$letter = @r[0].substr(0, 1);
				$way = @r[0].substr(1, 1);
				$rhs = @r[0].substr(2);
			}
			if $way eq '>' {
				&cond = * > $rhs;
			} else {
				&cond = * < $rhs;
			}
			Rule.new(cond => &cond, dest => @r[*-1], letter => $letter);
		});
		$name => Workflow.new(rules => @rules)
	};
}

our sub part1(Str $input) {
	my ($workflows, $parts) = $input.split("\n\n");
	my %workflows = workflows($workflows);
	[+] $parts.split("\n").map(sub ($part) {
		my @ints = +«m:g/\d+/ given $part;
		my %part = <x m a s> Z=> @ints;
		my $w = 'in';
		loop {
			return [+] %part.values if $w eq 'A';
			return 0 if $w eq 'R';
			$w = %workflows{$w}.process(%part);
		}
	});
}

our sub part2(Str $input) {
	my ($workflows, $parts) = $input.split("\n\n");
	my %workflows = workflows($workflows);

	my @queue;
	@queue.push({
		seen => ['in', 'R'].Set,
		at => 'in',
		parter => {
			x => 1..4000,
			m => 1..4000,
			a => 1..4000,
			s => 1..4000,
		},
	});
	my $answer = 0;
	while @queue {
		my %state = @queue.pop;
		my $w = %workflows{%state<at>};
		for $w.fork(%state<parter>) -> %newstate {
			if %newstate<at> eq 'A' {
				$answer += [*] %newstate<parter>.values».elems;
				next;
			}
			next if %newstate<at> (elem) %state<seen>;
			next if 0 == [*] %newstate<parter>.values».elems;
			%newstate<seen> = %state<seen> ∪ [%newstate<at>].Set;
			@queue.push(%newstate);
		}
	}
	$answer
}
