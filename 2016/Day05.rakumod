unit module Day05;
use NativeCall;

our sub part1(Str $input) {
	return '...';
	my $buf = CArray[uint8].new(('.' x 33).encode.list);
	my @z = lazy gather {
		for ^Inf {
			.say if $_ % 10000 == 0;
			my $hash = md5("$input$_", $buf);
			if $hash.starts-with('00000') {
				say $hash;
				take $hash.substr(5, 1);
			}
		}
	};
	@z[^8].join
}

our sub part2(Str $input) {
	my $buf = CArray[uint8].new(('.' x 33).encode.list);
	my @answer = '_' xx 8;
	for ^Inf {
		say $_ ~ ' ' ~ @answer.join if $_ % 10000 == 0;
		my $hash = md5("$input$_", $buf);
		 if $hash.starts-with('00000') {
			 say $hash;
			 my $pos = $hash.substr(5, 1);
			 if try { $pos < 8 } {
				 if @answer[$pos] eq '_' {
					 @answer[$pos] = $hash.substr(6, 1);
				 }
				 unless '_' (elem) @answer {
					 return @answer.join
				 }
			 }
		 }
	}
}

sub md5(Str $input, $out is rw) {
	MD5Data(CArray[uint8].new($input.encode.list, 0), $input.chars, $out);
}

#-From /usr/include/md5.h:44
#char	*MD5Data(const uint8_t *, size_t, char *);
sub MD5Data(CArray[uint8]                 # const Typedef<uint8_t>->«Typedef<__uint8_t>->«unsigned char»»*
           ,size_t                         # Typedef<size_t>->«long unsigned int»
           ,CArray[uint8]                            # char*
            ) is native('md') returns Str { * }

## Externs

