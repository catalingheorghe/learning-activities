# The C Programming Language (second edition)

## Introduction

Great short overview of C, which points out the fact that it is a small language. It does not offer input/output facilities or multithreaded mechanisms. It deals only with fundamental datatypes (and structures, unions), it does not have built-in operators for lists, strings. All of these are provided by the C library, which is also part of the standard.

Best wording about the use of pointers: offer an architecture independent way to do address arithmetics.

## Chapter 1 - a tutorial introduction

Some simple **style notes** in the following excerpt:

> we will always indent the statements controlled by the while by one tab stop
> (which we have shown as four spaces) so you can see at a glance which statements 
> are inside the loop. The indentation emphasizes the logical structure of the program.
> Although C compilers do not care about how a program looks, proper indentation 
> and spacing are critical in making programs easy for people to read. We recommend 
> writing only one statement per line, and using blanks around operators to clarify 
> grouping. The position of braces is less important, although people hold passionate 
> beliefs. We have chosen one of several popular styles. Pick a style that suits you, 
> then use it consistently

**"Magic numbers"**

It is bad practice to leave "magic numbers" like 300, 20, buried in the code. To give meaning to the reader, use a *symbolic name* or *symbolic constant* with the `#define name replacement-text` directive.

**Character vs string constants**

A *character constant* is a character written between single quotes; it is just another way to write an integer, a more meaningful way, of course, for characters. It is different than the same character written between double quotes, which is a *string constant* that contains only one character.

**Expressions and values**

Expressions can be evaluated to a value. An assignment expression evaluates to the left hand side value after assignment. That is why you can use the assignment expression in test conditions.

    `while ((c = getchar()) != EOF) { ...}`

Note that the parantheses are required, as `!=` has higher precedence than `=`, so without it, `c` would get assigned the truth evaluation (0 or 1) of the comparison. The loop would still stop at an EOF, but it would not give you the actual character from the input stream.

Assignments associate from left to right, so this is why you can write something like the below line to assign the same value to multiple variables.

    `a = b = c = 0`

**Arguments - call by value**

All function arguments are passed "by value", meaning that the called function receives a copy of its arguments, not the originals. To really modify the variable in the calling function, the address to that variable must be passed as an arguments (pointer).





## Future work

 1. Write unit tests for `01-wordcount.c` based on the comments in the file.





