# Learning Perl

*Learning Perl, 7th edition, by Randal Schwartz, brian foy, and Tom Phoenix (O’Reilly). Copyright 2017 Randal Schwartz, brian foy, and Tom Phoenix, 978-1-491-95432-4.*

# Table of Contents

0. [About](#0-about)
1. [Notes](#1-notes)
2. [Review](#2-review)

# 0. About

 - Safari [link](https://learning.oreilly.com/library/view/learning-perl-7th/9781491954317/)
 - Companion website [link](https://www.learning-perl.com/)
 - Companion workbook with exercises [link](https://learning.oreilly.com/library/view/learning-perl-student/9781449328047/)

# 1. Notes

## General/intro

No main routine. The "main" program consists of the Perl statements (not including anything in subroutines).  
There is no special section for variable declaration (it doesn't mean that you can't declare them).

On \*nix systems, sh-bang is ```#!/usr/bin/perl``` or the more portable ```#!/usr/bin/env perl```.

A Perl statement is an expression followed by semicolon (;). The semicolons are used to separate statements, not terminate them.

Perl programs are not compiled by the user. The perl interpreter compiles the source code into internal *bytecodes*. It then runs that bytecode. THe entire source is compiled before the bytecode is run.

perldoc contains thousands of pages of documentation

```
 $ perldoc perl
 $ perldoc perlfunc
 $ perldoc perlvar
 $ perldoc -f print
 $ perldoc -v '$_'
 $ perldoc -q open # search the faq
 ...
```

Running perl with -w (see perldiag) shows you warnings ```perl -w ...```..

## Scalar data

A scalar is a single thing.

All **numbers** in Perl are internally double-precision floating point numbers. There are no subclasses (integer, float, double).  

You can write long integer literals with underscores ```61_298_232_392_765```. **Not commas**.  

```
255
0377 # octal
0xff # hex, still 255
0b11111111 # binary
```






# 2. Review



