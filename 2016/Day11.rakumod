unit module Day11;
use Astar;

sub parse(Str $input) {
	$input.lines.map({
		m:g/ ' a ' (\w+ '-'?) /.map(*[0].Str).sort.Array
	});
}

sub solve(@floors) {
	my @start = [0, @floors];
	my $graph = Astar.new(
		distance   => -> @v, @w { 1 },
		successors => -> @v {
			my $me = @v[0];
			my @succ;
			my @objectshere = @(@v[1][$me]);
			#say "neigh of: " ~ @v.raku;
			#say @objectshere.raku;
			sub tryadd(@z) {
				for @(@z[1]) -> @objects {
					my @chips = @objects.grep(*.ends-with('-'));
					my @rtg = @objects.grep(!*.ends-with('-'));
					if @rtg {
						for @chips -> $chip {
							my $rtg = $chip.substr(0, *-1);
							return if $rtg !(elem) @rtg;
						}
					}
				}
				@succ.push(@z);
			}
			for $me - 1, $me + 1 -> $newfloor {
				next if $newfloor > 3;
				next if $newfloor < 0;
				next if $newfloor < 1 and @v[1][0].elems == 0;
				next if $newfloor < 2 and (@v[1][0, 1].elems == 0).all;
				for @objectshere -> $a {
					my @z = @v.deepmap({$_});
					@z[0] = $newfloor;
					@z[1][$me] = @objectshere.grep(* ne $a).Array;
					@z[1][$newfloor].push($a);
					@z[1][$newfloor] = @z[1][$newfloor].sort.Array;
					tryadd @z;
				}
				loop (my $i = 0; $i + 1 < @objectshere.elems; ++$i) {
					my $a = @objectshere[$i];
					loop (my $j = $i + 1; $j < @objectshere.elems; ++$j) {
						my $b = @objectshere[$j];
						my @z = @v.deepmap({$_});
						@z[0] = $newfloor;
						@z[1][$me] = @objectshere.grep({$_ ne $a and $_ ne $b}).Array;
						@z[1][$newfloor].append($a, $b);
						@z[1][$newfloor] = @z[1][$newfloor].sort.Array;
						tryadd @z;
					}
				}
			}
			@succ
		},
		heuristic  => -> @v, @w {
			my @f = @(@v[1]);
			@f[0].elems * 6 + @f[1].elems * 4 + @f[2] * 2 - 9
		},
		identifier => -> @v { @v.raku },
	);
	my @allobjects = @floors.map({@$_}).flat.sort.Array;
	my @goal = 3, [[], [], [], @allobjects];
	my $path = $graph.best-path(@start, @goal);
	$path.elems - 1
	#@start.raku
	#@goal.raku
	#@floors.raku
}

our sub part1(Str $input) {
	my @floors = parse($input);
	solve(@floors);
}

our sub part2(Str $input) {
	my @floors = parse($input);
	@floors[0].append('elerium', 'elerium-', 'dilithium', 'dilithium-');
	@floors[0] = @floors[0].sort.Array;
	solve(@floors);
}
