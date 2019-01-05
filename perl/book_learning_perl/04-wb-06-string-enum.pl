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

sub enumerate {
    my $str = ""; # better than undef, to avoid warning in 'say enumerate();'
    my $first = shift @_;
    my $last = pop @_;

    # defined or just $first - undef is interpreted as 0 in if
    if (defined $first) {
        $str .= $first;
    }

    foreach (@_) {
        $str .= ", $_";
    }

    if ($last) {
        $str .= " and $last";
    }

    $str;
}

# book solution, nice and elegant
sub separate {
    if(    @_ == 1 ) { return "@_" }
    elsif( @_ == 2 ) { return "$_[0] and $_[1]" }
    elsif( @_ >  2 ) {
        my $last = pop;
        local $" = ", ";
        return "@_, and $last";
    }
}

my @words = qw/ eggs bacon tomatos honey /;
say enumerate(@words);

say enumerate('one');
say enumerate('one', 'two');

say enumerate();
