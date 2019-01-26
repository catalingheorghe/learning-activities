#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

if (@ARGV != 1) {
    warn "Program expects a file name.";
    exit -1;
}

open FILE, "<", $ARGV[0] or die "Could not open $ARGV[0] ($!)";

# print the first line
print scalar <FILE>;

# this will also force scalar context, to get the first line only
#print my $line = <FILE>;

# or simply read a single line, by storing in a scalar
# print will print the default value $_
#$_ = <FILE>;
#print;

