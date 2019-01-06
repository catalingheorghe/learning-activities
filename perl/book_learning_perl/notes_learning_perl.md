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

If the control variable is omitted, perl uses the special variable `$_` in the loop. (`$_` is perl's most common default variable, but not the only one). Ex: `print;` will also print the value of `$_`.

The **reverse** operator takes a list of values and returns a reversed list. Can be used in combination with the range operator, for example, to count down.  
It does not affect its arguments, it returns a different value.

The **sort** operator takes a list of values and sorts them in internal character order, by default, which is unicode code point order. It also outputs a new value.  
Note that numbers are also sorted in char order, not numerically (by default).

The **each** operator can be used to go through the index and the value of the elements in an array. It applies to arrays since v5.12. It was originally applied to hashes.

```perl
@wilma = reverse 5..10;

@fred = sort(qw/ bedrock slate rubble/);

while ( ($index, $value) = each @fred ) {
    print "$index: $value\n");
}
```

**Scalar and list context**

Depending on the context of the expression, the same thing can be interpreted differently by perl. One example of this is the numerical vs string operators. An array name can give a list or a single scalar value

```perl
@people = qw( fred barney betty );
@sorted = sort @people;  # list context: barney, betty, fred
$number = 42 + @people;  # scalar context: 42 + 3 gives 45

@list = @people; # a list of three people
$n = @people;    # the number 3
```

List producing expressions can return various things in scalar context (where perl expects a scalar). You can't make any assumption and you must check the doc.

```perl
@backwards = reverse qw/ yabba dabba doo /;
    # gives doo, dabba, yabba
$backwards = reverse qw/ yabba dabba doo /;
    # gives oodabbadabbay
```

Examples of list vs scalar contexts:

```perl
$fred = something;            # scalar context
@pebbles = something;         # list context
($wilma, $betty) = something; # list context
($dino) = something;          # still list context!

# scalar
$fred = something;
$fred[3] = something;
123 + something
something + 654
if (something) { ... }
while (something) { ... }
$fred[something] = something;

# list
@fred = something;
($fred, $barney) = something;
($fred) = something;
push @fred, something;
foreach $fred (something) { ... }
sort something
reverse something
print something
```

A scalar producing expression is automatically promoted to a one element list in a list context. Note that `undef` is also a scalar, so to empty an array you assign the empty list `( )`.

To force a scalar context, you use the fake function `scalar`, which tells perl to provide a scalar context.

```perl
@rocks = qw( talc quartz jade obsidian );
print "How many rocks do you have?\n";
print "I have ", @rocks, " rocks!\n";        # WRONG, prints names of rocks
print "I have ", scalar @rocks, " rocks!\n"; # Correct, gives a number
```

**<STDIN> in list context** returns all the remaining input until EOF, one line in each list element.  
To discard newlines from all elements, just use `chomp` on the array. 

## Subroutines

Subroutine names are from a different namespace, so they can't clash with scalar names for example. Usually the name has in front an `&`, but it some cases it is not allowed.

Subroutines can be defined anywhere in the program.  
No forward declaration needed.  
The last definition of same subroutine overwrites the previous one.
Subroutines can access any global variable.

```perl
sub marine {
  $n += 1;  # Global variable $n
  print "Hello, sailor number $n!\n";
}
```

To invoke the subroutine: `&marine`.

All Perl subroutines have a return value. Not necessarily a useful one. Whatever calculation is last performed in a subroutine is also the return value.

Be careful with extra statements. Below, the return value will be 1, the one from `print`.

```perl
sub sum_of_fred_and_barney {
  print "Hey, you called the sum_of_fred_and_barney subroutine!\n";
  $fred + $barney;  # That's not really the return value!
  print "Hey, I'm returning a value now!\n";      # Oops!
}
```

To pass arguments to a subroutine, you give it a list of arguments. Perl makes the parameter list available to the subroutine, by copying it, in the special array variable `@_`. First argument is in `$_[0]`, but it has nothing to do with the `$_` special variable.

Perl doesn't care if insufficient arguments are passed. Beyond `@_` there are undefs. Also, it doesn't care if too many parameters are passed, they are just ignored if the subroutine does not access them.

The `@_` is private to the subroutine. The outside scope value is preserved. This why a subroutine can pass arguments to another subroutine.

Private variables for a subroutine, called **lexical variables**, can be created at any time with the `my` operator.

```perl
sub max {
  if (@_ != 2) {
    print "WARNING! &max should get exactly two arguments!\n";
  }  
  my($m, $n) = @_   # new, private variables for this block
                    #  which take the parameters provided
  if ($m > $n) { $m } else { $n }
}
```

Usually in Perl, subroutine take variable list of parameters and are written so that they can adapt to that. Example of max:

```perl
$maximum = &max(3, 5, 10, 4, 6);

sub max {
  my($max_so_far) = shift @_;  # the first one is the largest yet seen
  foreach (@_) {               # look at the remaining arguments
    if ($_ > $max_so_far) {    # could this one be bigger yet?
      $max_so_far = $_;
    }
  }
  $max_so_far;
}
```

*Note that there is no special connection between `@_` and `$_`.*

Take note also that a subroutine should also handle and empty parameter list. In the example above, it returns `undef`, which is ok.

**Note:** lexical (my) variables can be used in any block, they are not special to subroutines. If there is no enclosing block, it will be private to the source file.

**Note:** `my` does not change the context.

```perl
my($num) = @_;  # list context, same as ($num) = @_; - gets first param
my $num  = @_;  # scalar context, same as $num = @_; - get num of params
```

The **`use strict`** pragma can be used to tell the compiler to enforce strict variable rules in the containing block or file. Starting with 5.12, this is implicit when you declare a minimum Perl version. Note that $a and $b are not checked, since they are used by sort. The recommendation is to use it if the program is longer that a screen.

To return early from a subroutine, use the **return** operator. An empty return gives an undef or an empty list, depending if the subroutine is called in a scalar or list context.

If Perl can tell that it is a subroutine call, the ampersand can be omitted. However, if the subroutine has the same name a Perl build-in, you must use the ampersand.

A subroutine can **return nonscalar values**. If a subroutine is called in a list context, it can return a list of values. The function `wantarray` tells if a subroutine is being evaluated in a scalar or list context.

**Persistent private variables** can be created with `state`, instead of `my`. The values will be kept between calls. Any variable type can be made 'state', not only scalars.

```perl
use v5.10;

sub marine {
  state $n = 0;  # private, persistent variable $n
  $n += 1;
  print "Hello, sailor number $n!\n";
}
```

Perl v5.20 added **subroutine signatures** as an experimental feature. See perlsub.

## Input and Output

**Input from Standard input**

`<STDIN>` - line-input operator around a file-handle STDIN. It will return undef when reaching end-of-file.

```perl
# scalar context
while (defined($line = <STDIN>)) {
  print "I saw $line";
}

# shortcut with the use of perl's favourite default value $_
#  only works if there is nothing but the line-input operator in the
#  conditional of a while loop; no other connection between <> and $_
while (<STDIN>) {
  print "I saw $_";
}

# list context
#  here the entire input is stored in a list first, before the loop starts!
foreach (<STDIN>) {
  print "I saw $_";
}
```

**Input from the Diamond operator <>**

For a perl program that can be used like utilities cat, sed, awk, sort, grep, etc. Standard unit tools process files given to them as command line arguments one after another. If nothing is given, or if `-` is used instead of a file name, it reads from standard input. This is what the `<>` does: it reads from the files given as arguments, one line after another, in one big input.

```perl
while (defined($line = <>)) {
  chomp($line);
  print "It was $line that I saw!\n";
}
# name of file is stored in $ARGV special variable;

# or while (<>) shortcut
```

For v5.22 or higher use the double command `<<>>` to avoid interpreting a | in a file name (open a pipe input to an external program).

*The invocation arguments* - @ARGV array with all the arguments - normal array. This is what the diamond operator uses. You can tinker with the @ARGV array before calling the diamond operator.

**Output to Standard Output**

`print` takes a list of values and sends each item, as strings, to standard output, no spaces between.

```perl
print <>;          # implementation of /bin/cat
print sort <>;     # implementation of /bin/sort

 # Perl Power Tools project - goal is to implement all Unix standard utilities
```

if there are not parentheses, print is a list operator. If there are parentheses, print is a function call that will print what is found inside the parentheses.

Formatted output, C-style, can be done with `printf fmt_string list_of_things;`. For doc: <sprintf> in perlfunc.  
The format is an expression that can be computed. Ex: `printf "The items are:\n".("%10s\n" x @items), @items;`.

**Filehandles**

*Filehandle* = name of an I/O connection between your Perl process and the outside. Initially, they were barewords. v5.6 added the ability to store them into scalar variables. Recommendation to use uppercase barewords, to not clash with reserved words.

Special names: STDIN, STDOUT, STDERR, DATA, ARGV, ARGVOUT.

Opening a file handle: `open`.

```perl
open CONFIG, 'dino';
open CONFIG, '<dino';   # open for input, default
open BEDROCK, '>fred';
open LOG, '>>logfile';

my $selected_output = 'my_output';
open LOG, "> $selected_output";
 # the space avoids an append if the variable is ">something", for ex
```

Starting with 5.6, the way of opening is separated into a separate argument. This is recommended. It also allows specified the encoding.

```perl
open CONFIG, '<', 'dino';
open BEDROCK, '>', $file_name;
open LOG, '>>', &logfile_name();

open CONFIG, '<:encoding(UTF-8)', 'dino';
open BEDROCK, '>:encoding(UTF-8)', $file_name;
open LOG, '>>:encoding(UTF-8)', &logfile_name();
 # :utf-8 can be used as a shortcut, but it is not exactly the same thing; it does not
$ ensure that the data is in utf-8 encoding, it just treats it as such
```

Get all the encodings with this oneliner: `perl -MEncode -le "print for Encode->encodings(':all')"`

Other layer that handles transformation of input/output: ensure that at the end of each line there is a CR-LF - `:>crlf` - newlines (LF) will be transformed to CR-LF; or the other way around `:<crlf` to read a DOS line-endings file - CR-LF will be translated by Perl to LF when reading the file.

You can modify layers at anytime by **binmoding** a filehandle.

```perl
binmode STDOUT; # don't translate line endings
binmode STDERR; # don't translate line endings
binmode STDOUT, ':encoding(UTF-8)'; # STDOUT has to know how to handle utf-8
binmode STDIN, ':encoding(UTF-8)';
```

Reading from a **bad filehandle** (file not opened properly or closed network connection), gives an end-of-file. Writing will just silently discard the data.  
`open` returns true for success, false for failure.

To **close** a filehandle: `close BEDROCK`. It will flush output buffers and release any file locks.  
Perl automatically closes a filehandle if it is reused in another open (or if the program terminates, of course).

----

Fatal errors with **die function** : the `die` function prints the message to STDERR and terminates the program with a nonzeor exit status. The errno associated string from a system or library call (like `perror) is available in `$!`. If the message has a newline, the file name and line number won't be reported.

```perl
if ( ! open LOG, '>>', 'logfile' ) {
  die "Cannot create logfile: $!";
}

if (@ARGV < 2) {
  die "Not enough arguments\n";
}
```

Messages like **warnings** can be produced with `warn` function. Similar to `die`, except for terminating the program.

To automatically die on failed system calls, since v5.10, you can used the **autodie** pragma.

----

To **use a filehandle** is similar to the using STDIN, for example.

```perl
if ( ! open PASSWD, "/etc/passwd") {
  die "How did you get logged in? ($!)";
}

while (<PASSWD>) {
  chomp;
  others(...)
}
```

If a filehandle is open for writing or appending, the handle can be used in print or printf, before the argument list.

```perl
print LOG "Captain's log, stardate 3.14159\n";  # output goes to LOG
printf STDERR "%d percent complete.\n", $done/$total * 100;
```
No comma between the file handle and the items to be printed.

The `select` operator can change the default output file handle (`select BEDROCK;`). By default, the output of each filehandle is buffered. To change the behaviour of the currently selected filehandle, set `$|` to 1. The setting on that filehandle remains even after the selected one is changed.

```perl
select LOG;
$| = 1;  # don't keep LOG entries sitting in the buffer
select STDOUT;
 # ... time passes, babies learn to walk, tectonic plates shift, and then...
print LOG "This gets written to the LOG at once!\n";
```

To standard messages to file, the standard filehandles can be re-opened. Perl will not close the standard ones until it makes sure that it can open the file.

```perl
 # Send errors to my private error log
if ( ! open STDERR, ">>/home/barney/.error_log") {
  die "Can't open error log for append: $!";
}
```

**Filehandles in a Scalar** (since v5.6)

If you use a scalar variable without a value instead of a bareword in open, the filehandle is put in that variable. Some prefer to put a *_fh* at the of the names of these variables. 

```perl
my $rocks_fh;
open $rocks_fh, '<', 'rocks.txt'
  or die "Could not open rocks.txt: $!";

open my $rocks_fh, '<', 'rocks.txt'
  or die "Could not open rocks.txt: $!";

while( <$rocks_fh> ) {
  chomp;
  stuff;
}
```

Note that for print it is important to not put a comma after the filehandle, bareword or scalar variable; otherwise, perl will interpret the scalar variable as part of the items to print and it will print a stringification of that; for bareword, perl knows that it is a filehandle (if you just give print a bareword, it will print the content of `$_`).

```perl
print STDOUT;
print $rocks_fh;  # WRONG, probably
```

# 2. Review



