#!/usr/bin/env perl

use warnings;
use strict;

sub print_ruler {
    my $len = shift @_;
    my $str = "";

    foreach (1..$len) {
        $str .= $_ % 10;
    }
    print "$str\n";
}

print "Enter len of ruler:\n";
chomp(my $len = <STDIN>);

print "Enter list of strings (CTRL-D to stop):\n";
chomp(my @lines = <STDIN>);

&print_ruler($len);

foreach (@lines) {
    my $fmt = "%${len}s\n";
    printf $fmt, $_;
    # printf "$*s\n", $len, $_;
}

