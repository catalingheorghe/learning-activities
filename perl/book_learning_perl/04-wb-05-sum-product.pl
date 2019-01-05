#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

sub calculate {
    my $op = shift;

    if (! defined $op) { return; }

    if    ($op eq "+") { &add; }
    elsif ($op eq "*") { &multiply; }
    else  { return; }
}

sub add { 
    my $n = 0; 
    foreach (@_) {
        $n += $_;
    }
    $n;
}

sub multiply { 
    my $n = 1; 
    foreach (@_) {
        $n *= $_;
    }
    $n;
}

my @input = (1, 2, 3, 4);

print("Sum: ", calculate("+", @input), "\n");
print("Product: ", calculate("*", @input), "\n");

