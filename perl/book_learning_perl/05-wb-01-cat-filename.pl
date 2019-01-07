#!/usr/bin/env perl

use warnings;
use strict;

while (<<>>) {
    print "$ARGV: $_";
}

# without diamond operator
#foreach my $file ( @ARGV ) {
#    open my( $fh ), '<', $file;
#
#    while( <$fh> ) { print "$file: $_" }
#
#    close $fh;
#}
#
print "Enter some lines (CTRL-D) to stop.\n";
while (<STDIN>) {
    print "stdin: $_";
}

# or just manipulate @ARGV before <<>>, but you have to use <> to work
#push @ARGV, '-'


