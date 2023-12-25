unit module Day25;
use Inline::Python;

our sub part1(Str $input) {
	my $py = Inline::Python.new();
	$py.run(q:to/END/);
		import networkx
		import random

		def solve(input):
			g = networkx.Graph()
			for line in input.split('\n'):
				start, rest = line.split(': ')
				for end in rest.split(' '):
					g.add_edge(start, end, capacity=1)
			while True:
				a, b = random.sample([n for n in g], 2)
				cut, parts = networkx.minimum_cut(g, a, b)
				if cut == 3:
					return len(parts[0]) * len(parts[1])
		END
	$py.call('__main__', 'solve', $input);
}

our sub part2(Str $input) {
}
