#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

if (@ARGV == 0) {
    warn "Program expects a list of file names.";
    exit -1;
}

foreach my $file (@ARGV) {
    open my $file_fh, "<:crlf", $file
        or die "Cannot open file $file ($!)";
    while (my $line = <$file_fh>) {
        print $line;
    }
}

