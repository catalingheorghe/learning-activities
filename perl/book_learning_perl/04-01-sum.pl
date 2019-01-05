#!/usr/bin/env perl

use strict;
use warnings;
use v5.10;

sub total {
    my ($sum, @numbers) = (undef, @_); # or 0 for sum to not return undef
                                       # for an empty param list
    foreach (@numbers) {
        $sum += $_;
    }
    $sum;
}

my @fred = qw{ 1 3 5 7 9 };
my $fred_total = total(@fred);
print "The total of \@fred is $fred_total.\n";

print "Enter some numbers on separate lines: ";
my $user_total = total(<STDIN>);
print "The total of those numbers is $user_total.\n";

print "Sum of numbers 1 to 1,000: ", &total(1..1000), ".\n"

