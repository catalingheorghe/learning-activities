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

On \*nix systems, shebang is `#!/usr/bin/perl` or the more portable `#!/usr/bin/env perl`.

A Perl statement is an expression followed by semicolon (;). The semicolons are used to separate statements, not terminate them.

Perl programs are not compiled by the user. The perl interpreter compiles the source code into internal *bytecodes*. It then runs that bytecode. The entire source is compiled before the bytecode is run.

perldoc contains thousands of pages of documentation

```bash
$ perldoc perl
$ perldoc perlfunc
$ perldoc perlvar
$ perldoc -f print
$ perldoc -v '$_'
$ perldoc -q open # search the faq
(others ...)
```

Running perl with -w (see perldiag) shows you warnings `perl -w ...`.

## Scalar data

A scalar is a single thing.

All **numbers** in Perl are internally double-precision floating point numbers. There are no subclasses (integer, float, double).  

You can write long integer literals with underscores `61_298_232_392_765` (*not with commas*).

```perl
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

```perl
10 / 3  # 3.33333 ..., not 3 - all floating point division
```

Operators: 

```perl
+ - * / % **
```

**Strings** are a sequence of characters. Shortest one is the *empty string*. Longest string fills all your memory. Strings can contain binary data, like an image.

Built-in support for Unicode, but not enabled by default. To interpret the source code its-self, including string literals in it, as UTF-8, add this pragma `use utf8;` - recommended.

String literals - single-quoted or double-quoted (backslash - control characters and others).
Also, double-quoted strings are variable interpolated - variable current values are replaced when the string is used.

```perl
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

```perl
"hello" . "world" # . concatenates strings
"fred" x 3        # repetition
5 x 4.8           # left operand of x is converted to string;
                  # right operand is truncated to an int
```

Perl will automatically convert between numbers and strings, depending on what the operators expect.

**Warning and diagnostics** can be easily enabled. Perl interpreter can be ran with -w, or -w can be added to the shebang. Per file, the pragma `use warnings;` can be used. For extra details `use diagnostics` can be added (this slows down the start of the program). At runtime it can be enabled with `perl -Mdiagnostics ...`

Non-decimal notations to numbers:

```perl
hex('DEADBEEF')     # 3_735_928_559 decimal
hex('OxDEADBEEF')   # 3_735_928_559 decimal

oct('0377')         # 255 decimal
oct('377')          # 255 decimal
oct('0xDEADBEEF')   # 3_735_928_559 decimal, saw leading 0x
oct('0b1101')       # 13 decimal, saw leading 0b
oct("0b$bits")      # convert $bits from binary
```

A **scalar variable** holds one value. Starts with *sigil*, $, and then alphanumerics and underscore (can't start with a digit). Names are case sensitive. The sigil is used to distinguish from any special word in Perl. Also, the $ denotes that this variable stores a single item. (all caps names indicates a special variable - avoid to not clash with a perl variable)

Character by code point is created with `chr()` function. The other way, using the `ord` function.

Precedence of operators: perlop.

Numerical comparators - C like.  
String comparators: lt, le, eq, ge, gt, ne.

Number 0, empty string, string '0', variable without value - false. The rest - true.

The ! operator normalizes to 1 or 0 (the opposite logical value).

**User input**: simplest way is the line-input operator `<STDIN>`. It will return the line entered at stdinput by the user, including the newline. To remove the newline - `chomp()`. 

```perl
chomp($text = <STDIN>);   # read the text, assign to variable, use the chomp on the variable

$text = <STDIN>;
chomp($text);             # same, but in two steps
                          # return value: no of chars removed - 1 or 0.
                          #  it removes only one newline

chomp $text;              # paranthesis can be omitted if they don't change the meaning
```

Uninitialized variables have the *undef* value. undef is interpreted as 0 or as the empty string, so variables can be used directly. To check for undef, the function `defined` is used. Ex: <STDIN> return undef on no input, like end-of-file, so defined can be used to stop. 

## Lists and arrays

List = ordered collection of scalars.  
Array = a variable that contains a list.  

A list is the value, an array is a variable. An array variable always contains a list, but you can have a list value that isn't stored in an array.

 - indexing starting with 0 (like C)
 - can store a mix of scalar values 
 - can have any number of elements (from none to filling up all memory)
 - access via subscript 0 1 2 3 ...

Array name is from a different namespace than scalar variables. You can have an array and a scalar with the same name.

Index outside bonds of array - undef (just like an uninitialized scalar variable).  
If you store beyond the end, perl will extend the array with undef values until that point.  
Negative indexes can be used to count from the end of the array. # is used for last index of array.

```perl
$end = $#rocks;                  # 99, which is the last element's index
$number_of_rocks = $end + 1;     # OK, but you'll see a better way later
$rocks[ $#rocks ] = 'hard rock'; # the last rock

$rocks[ -1 ]   = 'hard rock';   # easier way to do that last example
$dead_rock     = $rocks[-100];  # gets 'bedrock'
$rocks[ -200 ] = 'crystal';     # fatal error!
```

A list literal is a comma separated values inside parentheses.  
The `..` can be used for consecutive ranges (it only counts up, not down).  
If the element is a variable, it will be evaluated every time the literal is used.

```perl
(1, 2, 3)
(1, 2..29)
( )
(1, "fred", $m)
(0..$#rocks)
```

`qw` *(quoted words)* can be used to generate a literal of strings with less typing. Treated like single-quoated strings with the whitespaces being insignificant. Any char, not only `(`, can be used as a delimiter.

```perl
qw( fred barney dino wilma )
qw( fred  barney
	dino )
qw/ fred barney /
qw{ 
	/usr/bin
	/usr/local/bin
}
```

**List assignment**

```perl
($fred, $barney, $dino) = ("flint", "rubble", undef);

($fred, $barney) = ($barney, $fred)          # swap
($rock[0], $rock[1]) = ($rock[1], $rock[0])
```

perl ignores extra values on the right, or puts undef for extra variables on the left.

So, an entire array could be build up with list assigning all the elements, but perl offer @ for refering to the entire array.

```perl
($rocks[0], $rocks[1], $rocks[2]) = qw[ talc mica quartz ];

@rocks = qw<talc mica quartz>;
@tiny   = ( );                       # the empty list
@giant  = 1..1e5;                    # a list with 100,000 elements
@stuff  = (@giant, undef, @giant);   # a list with 200,001 elements
$dino   = "granite";
@quarry = (@rocks, "crushed rock", @tiny, $dino); # @tiny does not add an undef element to quarry

@copy = @quarry  # still a list assignment; copy a list from one array to another
```

> Perl data structures cookbook: perldsc - arrays of arrays etc

**Pop and push operators** for treating the array like a stack.

```perl
@array = 5..9;

$fred = pop(@array);
$barney = pop @array;
pop @array;   # void-context, discard poped value

push(@array, 0);
push @array, 8..10;
push @array, @others;
```

**Shift and unshift** do the same thing at the "left-side" of the array.

```perl
unshift @array, 5;
$five = shift(@array);
```

The **splice** operator lets you manipulate elements in the middle of the array. Two mandatory args, two more optional.

```perl
@array = qw( pebbles dino fred barney betty );
@removed = splice @array, 2; # remove fred and everything after
                             # @removed is qw(fred barney betty)
                             # @array is qw(pebbles dino)

@array = qw( pebbles dino fred barney betty );
@removed = splice @array, 1, 2; # remove dino, fred
                                # @removed is qw(dino fred)
                                # @array is qw(pebbles barney betty)

@array = qw( pebbles dino fred barney betty );
@removed = splice @array, 1, 2, qw(wilma); # remove dino, fred
                                # @removed is qw(dino fred)
                                # @array is qw(pebbles wilma
                                #                barney betty)
```

If you specify 0 elements to remove, you can just insert the replacement list into that position.

Arrays are **interpolated into double-quoted strings**, with spaces between elements. Take care when trying to put an email address into a "" string.

To go through each element of a list or array, perl has **foreach**. The control variable is not a copy of the list element, it is the actual element, so it can modify the list values. Also, the control variable will have the same value as before the `foreach` construct, when finished.

```perl
$rock = 'shale';
foreach $rock (@rocks) {
	$rock .= "\n";   # add newline to each element
    ... # yada yada yada operator - compiles, but fatal error if encountered when running
}
 # $rock is still shale
```

If the control variable is omitted, TODO

**test**

# 2. Review



