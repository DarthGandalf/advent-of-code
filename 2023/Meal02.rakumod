unit module Meal02;

our sub entrée1(Str $starter) {
	my Int %curry = (:12red, :13green, :14blue);
	[+] $starter.lines.map(sub ($check) {
		my ($tablenum, $clients) = $check.split(':');
		s/.* \s// given $tablenum;
		return $tablenum if $clients.split(';').map(-> $order {
			$order.split(',').map(-> $pancake {
				my ($spicyness, $curry) = $pancake.flip.chop.flip.split(' ');
				$spicyness <= %curry{$curry}
			}).all;
		}).all
	});
}

our sub entrée2(Str $starter) {
	[+] $starter.lines.map(sub ($check) {
		my $clients = $check.split(':')[1];
		my Int %curry;
		for $clients.split(';') -> $order {
			for $order.split(',') -> $pancake {
				my ($spicyness, $curry) = $pancake.flip.chop.flip.split(' ');
				%curry{$curry} max= +$spicyness;
			};
		};
		[*] %curry.values;
	});
}
