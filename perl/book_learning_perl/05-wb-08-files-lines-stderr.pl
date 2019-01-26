#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

if (@ARGV == 0) {
    warn "Program expects a list of file names.";
    exit -1;
}

# reopen STDERR to a local file
# use ">>" for append
if (! open STDERR, ">", "stderr-05-wb-08.log") {
    die "Can't open local stderr log: $!;"
}

foreach my $file (@ARGV) {
    my $lines = &count_lines($file);
    if (defined $lines) {
        print "$file: $lines lines\n";
    }
}

sub count_lines {
    # implicit "@_"
    my $file = shift;
    my $num_lines = 0;

    if (open my $file_fh, "<", $file) {
        while (<$file_fh>) { $num_lines += 1 }
        # close is optional as it will be closed on program exit
        close $file_fh;
        return $num_lines;
    } else {
        # this goes to stderr
        # $! contains errno string
        warn "Could not open $file ($!)";
        # this returns undefined or empty listL
        return;
    }
}

