# Learning Perl

*Learning Perl, 7th edition, by Randal Schwartz, brian foy, and Tom Phoenix (O’Reilly). Copyright 2017 Randal Schwartz, brian foy, and Tom Phoenix, 978-1-491-95432-4.*

sada

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

Running perl with -w (see perldiag) shows you warnings ```perl -w ...```.

## Scalar data

A scalar is a single thing.

All **numbers** in Perl are internally double-precision floating point numbers. There are no subclasses (integer, float, double).  

You can write long integer literals with underscores ```61_298_232_392_765``` (*not with commas*).

```
255
0377       # octal
0xff       # hex, still 255
0b11111111 # binary
255.000
1.25
7.2e45  # 7.2 x 10 to the 45th
-12E-24 # negative 12 x 10 to the -24th
0x1.8p1 # hexadecimal floating point literals (v5.22)
	    # 1.8 hex X 2 to the 1st power = 3 dec
```

Numbers are in floating point:

```
10 / 3  # 3.33333 ..., not 3 - all floating point division
```

Operators: 

```
+ - * / % **
```

**Strings** are a sequence of characters. Shortest one is the *empty string*. Longest string fills all your memory. Strings can contain binary data, like an image.

Built-in support for Unicode, but not enabled by default. To interpret strings as UTF-8, add this pragma ```use utf8;``` - recommended.

String literals - single-quoted or double-quoated (backslash - control characters and others).
Also, double-quoted strings are variable interpolated - variable current values are replaced when the string is used.

```
'fred'
'don\'t'
'newline
here'
'no newline\nhere'

"hello world\n"   # newline
"tab\there"
"\x{2668}"        # unicode HOT SPRINGS character code point
"N{SNOWMAN}"      # unicode SNOWMAN by name

```

Operators:

```
"hello" . "world" # . concatenates strings
"fred" x 3        # repetition
5 x 4.8           # left operand of x is converted to string;
                  # right operand is truncated to an int
```

Perl will automatically convert between numbers and strings, depending on what the operators expect.

**Warning and diagnostics** can be easily enabled. Perl interpreter can be ran with -w, or -w can be added to the shebang. Per file, the pragma ```use warnings;``` can be used. For extra details ```use diagnostics``` can be added (this slows down the start of the program). At runtime it can be enabled with ```perl -Mdiagnostics ...```

Nondecimal notations to numbers:

```
hex('DEADBEEF')     # 3_735_928_559 decimal
hex('OxDEADBEEF')   # 3_735_928_559 decimal

oct('0377')         # 255 decimal
oct('377')          # 255 decimal
oct('0xDEADBEEF')   # 3_735_928_559 decimal, saw leading 0x
oct('0b1101')       # 13 decimal, saw leading 0b
oct("0b$bits")      # convert $bits from binary
```

A **scalar variable** holds one value. Starts with *sigil*, $, and then alphanumerics and underscore (can't start with a digit). Names are case sensitive. The sigil is used to distinguish from any special word in Perl. Also, the $ denotes that this variable stores a single item. (all caps names indicates a special variable - avoid to not clash with a perl variable)

Character by code point is created with ```chr()``` function. The other way, using the ```ord``` function.

Precedence of operators: perlop.

Numerical comparators - C like.  
String comparators: lt, le, eq, ge, gt, ne.

Number 0, empty string, string '0', variable without value - false. The rest - true.

The ! operator normalizez to 1 or 0 (the oposite logical value).

**User input**: simplest way is the line-input operator ```<STDIN>```. It will return the line entered at stdinput by the user, including the newline. To remove the newline - ```chomp()```. 

```
chomp($text = <STDIN>);   # read the text, assign to variable, use the chomp on the variable

$text = <STDIN>;
chomp($text);             # same, but in two steps
                          # return value: no of chars removed - 1 or 0.
                          #  it removes only one newline

chomp $text;              # paranthesis can be omitted if they don't change the meaning
```

Unitialized variables have the *undef* value. undef is interpreted as 0 or as the empty string, so variables can be used directly. To check for undef, the function ```defined``` is used. Ex: <STDIN> return undef on no input, like end-of-file, so defined can be used to stop. 


# 2. Review



