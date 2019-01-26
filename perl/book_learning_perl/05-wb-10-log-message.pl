#!/usr/bin/env perl

use warnings;
use strict;
use v5.10;

open LOG, ">>", "messages-05-wb-10.log"
    or die "Could not open log file ($!)"; 

say "Provide message:";
chomp(my $message = <STDIN>);

# localtime function returns different things in scalar vs list 
# context; in scalar returns a string like ctime(3)
my $timestamp = localtime;
say LOG "[$timestamp] $message";

