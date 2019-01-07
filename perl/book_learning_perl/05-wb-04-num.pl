#!/usr/bin/env perl

use warnings;
use strict;

print "Enter a whole number:\n";
chomp (my $num = <STDIN>);

# get the width specifier - maximum will be for binary representation
my $width = int(log($num) / log(2)) +1;

#printf "Number: %g, 0b%0.16b, 0%0.6o, 0x%0.4x\n", $num, $num, $num, $num;
# replicator (x) operator is not just for strings
printf "Number: %g
0b%0.${width}b
 0%0.${width}o
0x%0.${width}x\n", ($num) x 4;

