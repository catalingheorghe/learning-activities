#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

sub show_args {
    $"="][";
    say "The arguments are [@_]";
    # Data::Dumper module handles formats even better
}

sub show_args_again {
    &show_args;
    # & passes the @_ to the subroutine called
    show_args;
    # this will call the subroutine with empty parameter list
    &show_args();
    # this calls with empty parameter list
}

sub show_args_reverse {
    show_args(reverse @_)
}

show_args('fred', 'barney', 'betty');
show_args_again(qw/ fred barney betty/);
show_args_reverse(qw/ fred barney betty/);

