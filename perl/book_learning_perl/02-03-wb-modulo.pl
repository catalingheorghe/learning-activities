#!/usr/bin/env perl

use warnings;

print "Integer:\n";
chomp($num = <STDIN>);
#$num = <STDIN>;
# if we take out the chomp, the program would still work, and it will
# not give out a warning. perl will convert the string num to a number
# when doing the modulo operation and would not complain. num will still
# be a string with a newline at the end, however

if ($num % 2 == 0) {
	print "Even\n";
} else {
	print "Odd\n";
}

