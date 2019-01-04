#!/usr/bin/env perl

use warnings;
use v5.10;

say "Give the list of strings:";
chomp(@lines = <STDIN>);

@lines = reverse @lines;

say "\nReversed list:";
$" = ",";    # use , as separator
print "@lines\n";

#print @lines;
# this will print one element after another, no spacing because it prints a list

# more verbose
#foreach (@lines) {
#	say;
#}

# the simplest form:
# 
#print reverse <STDIN>;
#
