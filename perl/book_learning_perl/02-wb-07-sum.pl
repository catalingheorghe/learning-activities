#!/usr/bin/env perl

use warnings;
use v5.10;

say "Provide the numbers that will be added:";
while ($line = <STDIN>) {
	$sum += $line;
}

say "Sum: $sum"

