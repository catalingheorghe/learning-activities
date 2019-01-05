#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

sub greet {
    state @previous;
    my $name = shift @_;
    my $str = "";

    if (! defined($name)) {
        $name = "No Name";
    }

    $str .= "Hi $name!";

    if (@previous) {
        $str .= " I've seen @previous.";
    } else {
        $str .= " You are the first one here!";
    }
    push @previous, $name;

    say $str;
}

greet("Fred");
greet("Barney");
greet;
&greet();
greet "Wilma";
&greet("Dino");

