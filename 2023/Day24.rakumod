unit module Day24;

our sub part1(Str $input, :$range=[200000000000000, 400000000000000]) {
	my @lines = +«$input.lines.map({
		my ($pos, $vec) = .split(' @ ');
		my ($px, $py, $pz) = $pos.split(', ');
		my ($vx, $vy, $vz) = $vec.split(', ');
		{
			px=>$px,
			py=>$py,
			vx=>$vx,
			vy=>$vy,
		}
	});
	[+] @lines.combinations(2).race.map(sub (($a, $b)) {
		my $d = $a<vx> * $b<vy> - $b<vx> * $a<vy>;
		return 0 if $d == 0;
		my $au = $a<px> * ($a<py> + $a<vy>) - $a<py> * ($a<px> + $a<vx>);
		my $bu = $b<px> * ($b<py> + $b<vy>) - $b<py> * ($b<px> + $b<vx>);
		my $x = ($a<vx> * $bu - $au * $b<vx>) / $d;
		my $y = ($a<vy> * $bu - $au * $b<vy>) / $d;
		return 0 if $x < $range[0];
		return 0 if $x > $range[1];
		return 0 if $y < $range[0];
		return 0 if $y > $range[1];
		return 0 if ($a<px> - $x) * $a<vx> + ($a<py> - $y) * $a<vy> > 0;
		return 0 if ($b<px> - $x) * $b<vx> + ($b<py> - $y) * $b<vy> > 0;
		return 1;
	})
}

our sub part2(Str $input) {
	my $p = run('python', 'day24p2.py', :in, :out);
	$p.in.print($input);
	$p.in.close;
	my $out = $p.out.slurp(:close);
	chomp $out
}
