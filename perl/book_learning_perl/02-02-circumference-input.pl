#!/usr/bin/env perl

use warnings;
use v5.10;

$pi = 3.141592654;

say "What is the radius?";
chomp($radius = <STDIN>);

print "Calculating the circumference of a circle with radius $radius...\n";

$circumference = 2 * $pi * $radius;
print "Circumference: $circumference\n";
