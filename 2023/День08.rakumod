unit module День08;

grammar Парсер {
	token TOP { <откуда> ' = (' <лево> ', ' <право> ')' }
	token место { \w ** 3 }
	token откуда { <.место> }
	token лево { <.место> }
	token право { <.место> }
}

our sub часть1(Str $текст) {
	my ($первая, $остальное) = $текст.split("\n\n");
	my $куда = lazy gather {
		loop {
			for $первая.comb {
				take $_;
			}
		}
	};
	my %карта = $остальное.lines.map({
		my $п = Парсер.parse($_);
		$п<откуда> => {L => ~$п<лево>, R => ~$п<право>}
	});
	my $где = 'AAA';
	for $куда.kv -> $шаг, $куда {
		return $шаг if $где eq 'ZZZ';
		$где = %карта{$где}{$куда};
	}
}

our sub часть2(Str $текст) {
	my ($первая, $остальное) = $текст.split("\n\n");
	my %карта = $остальное.lines.map({
		my $п = Парсер.parse($_);
		$п<откуда> => {L => ~$п<лево>, R => ~$п<право>}
	});
	[lcm] %карта.keys.grep(*.ends-with('A')).map(sub ($где is copy) {
		my $куда = lazy gather {
			loop {
				for $первая.comb {
					take $_;
				}
			}
		};
		for $куда.kv -> $шаг, $куда {
			return $шаг if $где.ends-with('Z');
			$где = %карта{$где}{$куда};
		}
	})
}
