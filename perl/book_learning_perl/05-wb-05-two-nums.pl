#!/usr/bin/env perl

use warnings;
use strict;

if (@ARGV != 2) {
    printf "provide two numbers\n";
    exit;
}

print "@ARGV\n";

my @headings = qw/ first second sum product % /;
my $width = 10;
printf "%${width}s %${width}s %${width}s %${width}s %${width}s\n", @headings;
printf "%${width}g %${width}g %${width}g %${width}g %${width}.2f\n", ($ARGV[0], $ARGV[1], 
        $ARGV[0] + $ARGV[1],
        $ARGV[0] * $ARGV[1],
        ($ARGV[0] * 100) / $ARGV[1]);

# Note: should treat 0 division case above
#

