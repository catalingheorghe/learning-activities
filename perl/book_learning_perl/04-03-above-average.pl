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

sub average {
    my ($avg, @numbers) = (undef, @_);

    if ($#numbers > 0) {
        $avg = &total(@numbers) / ($#numbers + 1);
    }
    $avg;
}

sub above_average {
    my $avg = &average(@_);
    my @above_average;

    if (! defined($avg)) {
        return ();
    }

    foreach (@_) {
        if ($_ > $avg) {
            push @above_average, $_;
        }
    }

    @above_average;
}

my @fred = above_average(1..10);
print "\@fred is @fred\n";
print "(Should be 6 7 8 9 10)\n";

my @barney = above_average(100, 1..10);
print "\@barney is @barney\n";
print "(Should be just 100)\n";

