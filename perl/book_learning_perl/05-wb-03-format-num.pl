#!/usr/bin/perl
use strict;
use warnings;

print format_number( 9999 ), "\n";

sub format_number {
    local $_ = shift;

    1 while s/^([-+]?\d+)(\d{3})/$1,$2/;

    #print $_;
    # the above will print the number an then return 1 from the subroutine

    $_;
}
