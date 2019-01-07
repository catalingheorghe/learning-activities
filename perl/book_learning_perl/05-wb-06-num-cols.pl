#!/usr/bin/env perl

use warnings;
use strict;

sub max {
    my $max = 0;
    foreach (@_) {
        if ($_ > $max) { $max = $_ }
    }
    # or compare as strings with length $_, no need for width
    if (! defined($max)) { $max = 0 }
    $max
}

my $max = &max(@ARGV);
my $width;

if (! $max) {
    exit 1;
}

$width = int(log($max) / log(10)) + 1;

print "$width\n";

foreach (@ARGV) {
    printf "%${width}g\n", $_;
}

