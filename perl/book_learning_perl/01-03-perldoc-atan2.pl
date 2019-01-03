#!/usr/bin/env perl

@lines = `perldoc -u -f atan2`;
foreach (@lines) {
    s/\w<(.+?)>/\U$1/g;
    print;
}

# line 3 runs an external command
#   the output is saved in the array variable @lines
# a loop for each line
#   changes markers < >
#   prints the line (changed)
