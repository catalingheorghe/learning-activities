#!/usr/bin/env perl

use warnings;
#use utf8;
# this pragma can be left out; it is needed if the are wide chars in the
# source code its-self; nevertheless, it is recommeded to have it

binmode STDOUT, ':utf8';

print "Unicode code point (hex):\n";
chomp($code_point = <STDIN>);

$w_char = chr(hex($code_point));

print "Unicode char: $w_char\n"

