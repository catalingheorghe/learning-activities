#!/usr/bin/env perl

use warnings;
use v5.10;

say "Provide the two numbers that will be multiplied:";
$count = 0;
while ($count < 2) {
	$count += 1;
    if ($count == 1) {
		chomp($num1 = <STDIN>);
	} else {
		chomp($num2 = <STDIN>);
	}
}

$prod = $num1 * $num2;

say "Product: $prod"

