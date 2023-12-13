               unit module
               Day13;sub d
               (@a,@b) { (
            @a.join.comb Z @b.join.comb).grep({$^a[0] ne $a[1]}).elems };sub solve(Str $i, Int $d){
            [+] $i.split("\n\n").map(sub ($ma) {my @ma = $ma.lines».comb».Array; for 1..@ma.elems-1
            -> $v { my $len = min($v, @ma.elems-$v); return 100 * $v if d(@ma[$v - $len .. $v - 1],
               @ma[$v ..$v
               + $len - 1]
               .reverse)==

                  $d;};
                    ;
                    ;
                    ;
for 1..@ma[0].elems-1 -> $h { my $len = min($h, @ma[0].elems-$h); return $h if d(@ma[^*]»[$h
- $len .. $h -1]#`( ; ), @ma[^*]»[$h .. $h + $len - 1]».reverse».list) == $d; } }); }; our
sub part1(Str $i) { ; solve($i, 0) }; our sub part2(Str $i) { solve($i, 1) }
                    ;
                    ;
