#!/usr/bin/env perl

use warnings;
use strict;

print "Enter list of strings (CTRL-D to stop):\n";
chomp(my @lines = <STDIN>);

#foreach (1..75) {
#   print $_ % 10;
#}
#print "\n";

print "1234567890" x 7, "12345\n";

foreach (@lines) {
    printf "%20s\n", $_;
}

