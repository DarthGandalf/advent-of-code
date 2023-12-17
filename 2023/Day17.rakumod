unit module Day17;
use Astar;

my @map;
my Int $min;
my Int $max;

sub set_globals($mi, $ma) {
	$min = $mi;
	$max = $ma;
}

sub parse(Str $input) {
	$input.lines».comb».Int».Array;
}

sub dist(@v, @w) {
	if @v[0].im == @map.end && @v[0].re == @map[0].end {
		return 0;
	}
	heat_loss_at(@w)
}

sub filterX(@succ) {
	@succ.grep({
		my $p = $_[0];
		0 <= $p.re <= @map[0].end
	})
}

sub filterY(@succ) {
	@succ.grep({
		my $p = $_[0];
		0 <= $p.im <= @map.end
	})
}

sub filter(@succ) {
	@succ ==> filterX() ==> filterY()
}

sub add_straight(@succ, $pos, $dir, $len) {
	if $len < $max {
		@succ.push([$pos + $dir, $dir, $len + 1]);
	}
}

sub add_turns(@succ, $pos, $dir is copy, $len) {
	return if $len < $min;
	$dir *= 1i;
	@succ.push([$pos + $dir, $dir, 1]);
	$dir *= -1;
	@succ.push([$pos + $dir, $dir, 1]);
}

sub add_neigh(@succ, $pos, $dir, $len) {
	add_straight(@succ, $pos, $dir, $len);
	add_turns(@succ, $pos, $dir, $len);
}

sub succ([$pos, $dir, $len]) {
	if $len == 0 {
		return [
			[1+0i, 1+0i, 1],
			[0+1i, 0+1i, 1],
		];
	}
	succ2([$pos, $dir, $len]);
}

sub succ25($pos) {
	my @succ;
	@succ.push([$pos, 0+0i, 0]);
	return @succ;
}

sub succ2([$pos, $dir, $len]) {
	if $pos.im == @map.end && $pos.re == @map[0].end && $len >= $min {
		return succ25($pos);
	}
	succ3([$pos, $dir, $len]);
}

sub succ3([$pos, $dir, $len]) {
	my @succ;
	add_neigh(@succ, $pos, $dir, $len);
	return filter(@succ)
}

sub heur(@v, @w) {
	@map.end - @v[0].im + @map[0].end - @v[0].re
}

sub id(@v) {
	@v.join(',')
}

sub build_graph {
	Astar.new(
		distance => &dist,
		successors => &succ,
		heuristic => &heur,
		identifier => &id,
	)
}

sub goal {
	[Complex.new(@map[0].end, @map.end), 0+0i, 0]
}

sub find_path($graph) {
	$graph.best-path([0+0i, 0+0i, 0], goal());
}

sub heat_loss_at(@v) {
	@map[@v[0].im][@v[0].re]
}

sub process_path(@path) {
	@path[1..*-2].map(&heat_loss_at).sum
}

sub solve(Str $input) {
	@map = parse($input);
	my $graph = build_graph();
	my @path = find_path($graph);
	process_path(@path);
}

our sub part1(Str $input) {
	set_globals(0, 3);
	solve($input);
}

our sub part2(Str $input) {
	set_globals(4, 10);
	solve($input);
}
