#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

if (@ARGV != 2) {
    warn "Program expects <input_file> <output_file>.";
    exit -1;
}

my $input = shift @ARGV;
my $output = shift @ARGV;

open my $input_fh, "<:encoding(UTF-8)", $input
    or die "Cannot open input file $input ($!)";

open my $output_fh, ">:encoding(UTF-16BE)", $output
    or die "Cannot open output file $output ($!)";

while (<$input_fh>) {
    print $output_fh $_;
}

