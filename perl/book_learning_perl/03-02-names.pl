#!/usr/bin/env perl

use warnings;
use v5.10;

@names = qw/ 
            fred
			betty
			barney
			dino
			wilma
			pebbles
			bamm-bamm
			/;

print "List of names = @names\n";
say "Give me a number:";
while ($num = <STDIN>) {
	say "name: $names[$num]";
	say "Give me another number:"
}

