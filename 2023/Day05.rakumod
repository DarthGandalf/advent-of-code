# The module to solve day 5. Since the rest of the file belongs to this module,
# I specified 'unit' instead of wrapping the rest with {}.
unit module Day05;

# Function to solve part 1. 'sub' is how functions are declared, 'our' allows
# calling it from run.raku which is outside this module (Day05), because by
# default subs are declared as 'my' which limits access to them to be only from
# within current scope (in this case, module). $input is a parameter, and it's
# specified to have type Str, which stands for String.
our sub part1(Str $input) {
	# First, split the input for seed and different maps. Empty line is
	# essentially newline followed by newline, which is why this works.
	# It uses @ instead of $ like for $input because this is array. In Raku
	# arrays use @ while scalars use $, this symbol is called 'sigil'.
	my @blocks = $input.split("\n\n");
	# First block is 'seeds:', but we take [0] instead of [1] because in Raku
	# indexing starts with zero. m// is a regex (regular expression) search in
	# a string, and :g makes it search all the non-overlapping instances of
	# whatever is being searched. It searches in variable named $_, which is
	# why we use 'given' to make the value of $_ to be the same as @blocks[0].
	# () is a capture group, since we want to capture the numbers. Speaking of
	# numbers, \d is a digit, and appending + makes it find "one or more"
	# digit. But the result of this operation is list of strings, not list of
	# numbers. That's why we call .Int on each element, this is done by using
	# ». operator, which calls the specified method on every element in the
	# sequence, potentially in multiple threads. This is similar to .map,
	# except that it may run in parallel, and unlike .map, it will recurse into
	# elements if they are lists/hashes themselves. That makes it behave more
	# similar to .deepmap, but here the elements are just strings, so it works.
	my @current = m:g/(\d+)/».Int given @blocks[0];
	# We already parsed the list of seeds, no need to hold it anymore. .shift
	# method removes first element of the array.
	@blocks.shift;
	# Now do loop over all mappings. At each iteration @current holds the
	# current list of numbers. Initially, seeds, in the end it'll be locations.
	for @blocks -> $block {
		# $block is the text of the current mapping we're processing. Its first
		# line just states its name, and since they are in order, it's actually
		# irrelevant. .lines method splits the text by newlines, and .skip(1)
		# causes to ignore the first element in the list. The method .map is
		# called for every line, and it accepts a code block, which is very
		# similar to a function/subroutine.
		my @ranges = $block.lines.skip(1).map({
			# It's possible to specify a signature of a block, just like
			# subroutines have signatures, but here we skipped it. So by
			# default .map passes the parameter in the variable $_. There is
			# nothing on the left of .words, this is a shortcut for calling the
			# method on $_. Therefore, here the line is being split to words,
			# and then, for every word, just like before, its string
			# representation is converted to a number. Since this is the last
			# statement in the block, its value is what the block returns. As
			# result, @ranges will hold one element per line, where every
			# element is an array of 3 numbers, because that's how many numbers
			# were on every line in the mappings in the input.
			.words».Int;
		});
		# Unfortunately, '@current = @current.map()' doesn't work, therefore I
		# have to use a separate variable for it. Here I decided to use a sub
		# instead of a block, because I'm using 'return' in it. This is the
		# difference between sub and block - 'return' statement finds the
		# nearest sub, and returns from it, ignoring all the blocks which are
		# in that sub. Because we don't want to return from sub part1 yet,
		# here's the sub it'll return from. It's possible to avoid 'return' and
		# just use the last statement's value, but that makes the code more
		# complicated.
		my @next = @current.map(sub ($num) {
			# I didn't mention it for the earlier 'for', but actually it
			# accepts a block. And -> is start of the block's signature. Unlike
			# the block above, which accepts $block as parameter (I probably
			# should have chosen less confusing naming scheme to avoid the
			# overloaded term), this signature requires the argument passed to
			# it to be a list of 3 elements, and it automatically assigns the
			# specified names to these elements. This is called destructuring,
			# or structered bindings, depending on the language. A sub's
			# signature can do such thing too.
			for @ranges -> [$dst, $src, $len] {
				# Checking if our seed is within the range. In Raku it's
				# possible to use such chained comparison, and it won't try to
				# compare a bool with a number, unlike in some other languages.
				if $src <= $num < $src + $len {
					# If it was in fact within the range, do the
					# transformation: shift the number by $dst - $src.
					return $num - $src + $dst;
				}
			}
			# If there were no ranges in the mapping which covered our seed, it
			# should be left as is, therefore return it from the sub. As this
			# is the last statement in the sub anyway, the 'return' keyword may
			# be skipped. ';' also can be skipped for last statement.
			return $num;
		});
		# The current mapping is done, move the numbers back to @current to
		# prepare for the next mapping.
		@current = @next;
	}
	# All mappings are done, @current now holds the numbers of the locations,
	# just take the minimum of these numbers, and return it from part1. This is
	# the last statement in the sub, so 'return' can be skipped. Here I skipped
	# ';' too.
	@current.min
}

# Function to solve part 2 of the task. Many concepts should be familiar
# already after reading comments for part1.
our sub part2(Str $input) {
	my @blocks = $input.split("\n\n");
	my @numbers = m:g/(\d+)/».Int given @blocks[0];
	@blocks.shift;
	#  For now the code was the same as in part1, here the difference starts.
	#  Now @current will hold not just numbers, but pairs which will represent
	#  ranges. Raku has a dedicated Range type, but I found it simpler for this
	#  case to just use Pair where key (first element) is the start of the
	#  interval (including), and value (second element) is the end of the
	#  interval (excluding - so, it's the first element which is after the
	#  interval).
	#  Here you can see that the signature of the for block accepts 2
	#  parameters. This makes the for loop to group elements by 2, without
	#  overlap, and call the block with such pairs. Inside the block there is
	#  an expression which creates a Pair, using =>. The expression to the left
	#  of => is called the key, and to the right is called the value. Usually
	#  Pairs are used in mappings, e.g. in hashes, or when passing named
	#  arguments to functions. Here we use the Pair as just a pair numbers.
	#  'do' keyword turns for loop into an expression. This for loop will
	#  return the list of values - the list of pairs.
	my @current = do for @numbers -> $a, $b { $a => $a + $b };
	for @blocks -> $block {
		# This is done just like before.
		my @ranges = $block.lines.skip(1).map({
			.words».Int;
		});
		my @next;
		# While there are some ranges to proccess for the current mapping, process one of them.
		# Not using for loop or .map here, because we'll need to put values back to @current, depending on how ranges intersect.
		# Moreover, this loop has a label. This is used inside the loop with
		# the 'next' keyword which loop's next iteration to execute. More on
		# that later.
		LOOP: while @current.elems > 0 {
			# .pop takes the last element of the array. We can take any element
			# really, but the last one makes @current to be treated as stack,
			# if we push to the last element, and take from the last element,
			# which potentially allows less reallocation, and less movements of
			# the elements, making the program perform faster, at least in
			# theory.
			my $range = @current.pop;
			for @ranges -> [$dst, $src, $len] {
				# There are several cases, depending on how the range we're
				# handing overlaps with the range in the mapping. First, check
				# whether the start of current range is within.
				if $src <= $range.key < $src + $len {
					# If end of the range is also within, the whole range is
					# mapped by the range in the mapping, so shift it to @next
					# as a whole. Note <= here, because we're comparing the
					# ends of two open intervals.
					if $range.value <= $src + $len {
						@next.push($range.key - $src + $dst => $range.value - $src + $dst);
					} else {
						# However, if the end of the range was not within the
						# range of mapping, we need to split our range into
						# two: one part is shifted and goes to the next
						# mapping, but the remainder will need to search for
						# some other mapping to be shifted, we push it back to
						# the @current stack, and next iteration of the while
						# loop will handle it.
						@next.push($range.key - $src + $dst => $dst + $len);
						@current.push($src + $len => $range.value);
					}
					# 'next' keyword searches for the nearest loop, finishes
					# current iteration of it, and starts the next iteration if
					# there are still any. Here we need to provide the label
					# LOOP, because the nearest loop is 'for @ranges', but we
					# need to continue to next element of @current instead.
					next LOOP;
				}
				# Now check if the mapping range contains the end of our range,
				# and if so, do a similar split, except that the end of the
				# range is shifted, while the beginning is pushed back to
				# @current.
				if $src < $range.value <= $src + $len {
					@next.push($dst => $dst + $range.value - $src);
					@current.push($range.key => $src);
					next LOOP;
				}
				# This will happen if the mapping range cuts in middle of our
				# range, this will cut it in 3 parts. The middle gets shifted
				# to next mapping, but the start and the end go back to
				# @current, and will be handled in next iterations of the loop.
				if $range.key <= $src <= $src + $len <= $range.value {
					@next.push($dst => $dst + $len);
					@current.push($range.key => $src);
					@current.push($src + $len => $range.value);
					next LOOP;
				}
			}
			# None of the ranges in the mapping intersect our range at all, so
			# just put it to @next as is.
			@next.push($range);
		}
		# Now @next contains the list of ranges for the next mapping, and we
		# can do a similar assignment to one in part1. But we did so any splits
		# of the ranges, that their number could grow a lot. And in fact,
		# depending on how these mappings were defined, now some of the ranges
		# can overlap. So, here we'll simplify this list by merging the
		# overlapping ranges, reducing their number.
		# The .sort method returns a sorted copy of the array. Here the for
		# block doesn't have a signature, which means that the value is passed
		# via $_.
		for @next.sort {
			# Try to merge the current range $_ with the last range in @current
			# if any. Because we're doing it in the ascending order, this
			# gurantees that the last range in @current is the highest one.
			# $_.key is the same as .key
			if (@current.elems > 0) and (@current[*-1].value >= $_.key) {
				# '*' is called 'Whatever' in Raku, and its meaning highly
				# depends on the context how it's used. '*-1' is actually an
				# expression which returns a function which takes a number,
				# substracts 1 from it, and returns the result. Then the
				# question is how is it possible to index an array using a
				# function. The answer is that the indexing operator does
				# different things depending on the type: if it's just a
				# number, it returns the Nth element; but if it's a callable (a
				# sub or a block), it calls the function with the length of the
				# array as the parameter. Therefore, [*-1] will return the
				# element with index "size - 1", and since Raku indexes arrays
				# from zero, this will be the last element of the array.
				my $updated = @current[*-1];
				$updated = $updated.key => max($updated.value, $_.value);
				@current[*-1] = $updated;
			} else {
				# The range doesn't overlap with the last one, so just push it
				# to @current.
				@current.push($_);
			}
		}
	}
	# Because the last operation in the loop was sort, @current is now still
	# sorted by key (first element in the Pair), therefore first element of it
	# is the smallest, which is what was required.
	@current[0].key
}
