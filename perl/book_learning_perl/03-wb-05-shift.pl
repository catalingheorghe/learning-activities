#!/usr/bin/env perl

use warnings;
use utf8;
use v5.10;

@numbers = 1..10;

while (@numbers) {
	$num = shift @numbers;
	say "$num: " . $num**2 . " " . $num**3;
}

