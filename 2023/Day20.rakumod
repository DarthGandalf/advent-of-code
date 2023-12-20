unit module Day20;

grammar G {
	token TOP { <line>*%"\n" }
	rule line { <type>? <name> '->' <dest>*%"," }
	token type { '&' | '%' }
	token name { \w+ }
	token dest { \w+ }
}

class Pulse {
	has Str:D $.from is required;
	has Str:D $.to is required;
	has Bool:D $.value is required;
}

class Module {
	has Str:D $.name is required;
	has Str:D @.outputs;

	method send(Bool:D $pulse) {
		@.outputs.map({
			Pulse.new(from => $.name, to => $_, value => $pulse);
		})
	}

	method process(Pulse:D $pulse) {
		self.send($pulse.value)
	}

	method add_input(Str:D $in) {
	}
}

class FlipFlop is Module {
	has Bool:D $.state is rw = False;

	method process(Pulse:D $pulse) {
		{
			True=>{[]},
			False=>{
				$.state = !$.state;
				self.send($.state);
			},
		}{$pulse.value}();
	}

	method add_input(Str:D $in) {
	}
}

class Conjunction is Module {
	has Bool:D %.state;

	method process(Pulse:D $pulse) {
		%.state{$pulse.from} = $pulse.value;
		self.send(!%.state.values.all);
	}

	method add_input(Str:D $in) {
		%.state{$in} = False;
	}
}

sub parse(Str:D $input) {
	my $g = G.parse($input);
	my Module:D %modules;
	for $g<line> {
		%modules{$_<name>} = {
			'' => Module,
			'&' => Conjunction,
			'%' => FlipFlop,
		}{$_<type> // ''}.new(name => ~$_<name>, outputs => [~«$_<dest>]);
	}
	for %modules.kv -> $name, $m {
		for $m.outputs -> $out {
			%modules{$out}.add_input($name);
		}
	}
	%modules;
}

our sub part1(Str:D $input) {
	my Module:D %modules = parse($input);
	my Int:D @count = 0, 0;
	for ^1000 {
		my Pulse:D @mq;
		@mq.push(Pulse.new(
			from => 'button',
			to => 'broadcaster',
			:!value
		));
		LOOP: while @mq {
			my $pulse = @mq.shift;
			@count[+$pulse.value]++;
			while %modules{$pulse.to}:!exists {
				next LOOP;
			}
			@mq.append(%modules{$pulse.to}.process($pulse));
		}
	}
	[*] @count
}

sub inputs(Module:D %modules, Str:D $name) {
	%modules.grep({
		$name (elem) $_.value.outputs
	})».key
}

our sub part2(Str:D $input) {
	my Module:D %modules = parse($input);
	my $hb = inputs(%modules, 'rx')[0];
	my Int:D %k = inputs(%modules, $hb) »=>» 0;
	for 1..^Inf -> $press {
		my Pulse:D @mq;
		@mq.push(Pulse.new(
			from => 'button',
			to => 'broadcaster',
			:!value
		));
		LOOP: while @mq {
			my $pulse = @mq.shift;
			while $pulse.value and $pulse.to eq $hb and %k{$pulse.from}:exists and %k{$pulse.from} == 0 {
				%k{$pulse.from} = $press;
				while !%k.values.grep(* == 0) {
					return [lcm] %k.values;
				}
				last;
			}
			while %modules{$pulse.to}:!exists {
				next LOOP;
			}
			@mq.append(%modules{$pulse.to}.process($pulse));
		}
	}
}
