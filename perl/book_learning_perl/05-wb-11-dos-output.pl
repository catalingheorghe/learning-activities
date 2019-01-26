#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

if (@ARGV == 0) {
    warn "Program expects a list of strings.";
    exit -1;
}

binmode STDOUT, ':crlf';

foreach (@ARGV) {
    say $_;
}

# if you pipe the output to `hexdump -C`, you will notice the
# CR-LF (0d 0a) pair
