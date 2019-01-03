#!/usr/bin/env perl

use warnings;

print "String:\n";
$str = <STDIN>;

print "Number of reps:\n";
chomp($reps = <STDIN>);

$output = $str x $reps;

print "Output:\n----\n" . $output . "----\n";

