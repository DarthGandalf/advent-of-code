die if $] >= 4;
$_ = <>;
@times = split;
shift @times;
$_ = <>;
@distances = split;
shift @distances;
@times = join('', @times);
@distances = join('', @distances);
$answer = 1;
for ($i = 0; $i <= $#times; ++$i) {
	$start = -1;
	$end = -1;
	for ($j = 0; $j <= $times[$i]; ++$j) {
		$d = ($times[$i] - $j) * $j;
		if ($d > $distances[$i]) {
			$start = $j if $start < 0;
			$end = $j;
		}
	}
	$answer *= $end - $start + 1;
}
print "answer: $answer\n"
