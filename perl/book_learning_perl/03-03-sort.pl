#!/usr/bin/env perl

use warnings;
use utf8;
use v5.10;

say "Give me the list of strings, one per line. End with <CTRL-D>.";
chomp(@lines = <STDIN>);

@sorted_lines = sort @lines;
print "@sorted_lines\n";

# or
#print sort <STDIN>; # output one per line
