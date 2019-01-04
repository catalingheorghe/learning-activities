#!/usr/bin/env perl

use warnings;
use v5.10;

# second to last line, without using an array

say "Input lines, please. End with <CTLR-D>.";
while ($line = <STDIN>) {
	($second_last, $last_line) = ($last_line, $line);
}

if (defined $second_last) {
	say "Second to last line: $second_last"
} else {
	say "No second to last line"
}

