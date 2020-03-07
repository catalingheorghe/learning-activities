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


## Chapter 5 - pointers and arrays

A pointer is a variable that contains the address of a variable.

The proper type for a generic pointer is `void *`. This was standardised in ANSI C.

The `*` and `&` operators bind more tightly than arithmetic operators. However, when used with unary operator `++`, they associate from right to left

```C
    ++*ip;    // increments what ip points at
	(*ip)++;  // paranthesis required, otherwise you increment ip
```

**Pointers and arrays**

Everything that can be done with array subscripting, can be done with pointer arithmetic as well. The basis of pointer arithmetic is that `pa+1` points to the next object.

The name of an array is a synomin for the location of the first element. So the following statements are equivalent:

```C
    pa = &a[0];
	pa = a;

	x = a[i];
	x = *(a+i);
	x = pa[i];
```

A pointer is a variable, while an array name is not (it cannot pe used in expression like `a=pa` or `a++`).

When an array is passed to a function, what is passed is the address of the first element. It is preferred to use `char *s` instead of `char s[]` in function declarations, even though they are equivalent, because it is more explicit that the parameter is a pointer.

**Some points about pointer arithmetics**

Note that you can also index backwards (`p[-1]`) if you are sure that the previous elements exist.

C guarantees that '0' is not a valid address for data, so to signal an invalid pointer, '0' can be used. This is the only integer that can be used interchangeable with a pointer. The symbolic constant `NULL` is usually used to signal that it is a special value for a pointer.

Pointer comparisons (`==,<,>,!=`) work just fine between two pointers that point to elements in the same array (or to the address of the first element past the end of the array). Pointers towards the same array can be subtracted to give the number of elements between them. Arithmetic or comparisons is undefined for pointers that do not point to members of the same array.

*Valid pointer operations*

 - assignment of pointers of the same type
 - adding or subtracting a pointer and an integer
 - subtracting or comparing two pointers to members of the same array
 - assigning or comparing to zero

**Character pointers and arrays**

The following two definitions are very different.

```C
    char amessage[] = "a message";
	char *pmessage = "a message";
```

The first is an array big enough to hold the *string constant* and the NUL ('\0') character ends it. Content of the array can be changed, but the array will always refer to the same storage. On the other hand, the second is a character pointer that points to a string constant; the pointer can be changed to point elsewhere, but modifying the initial string contents leads to undefined behaviour.

Shortest form of strcpy: `while (*s++ = *t++) { ; }`. It uses the fact that the postfix `++` will increment `s` and `t` after they are referenced, and that the test expression is evaluated to the value of the left-hand side after assignment. When '\0', which is '0' in octal character constant form, is copied, it will exit the loop.

If the prefix operator is used, `*--p`, the pointer is decremented before dereferencing it.

```C
    *p++ = val;  // push val onto stack
	val = *--p;  // pop top of stack into val
```

**Pointer arrays**

A data storage for referencing lines of text can be an array of pointers: `char *lineptr[MAXLINES]` - array of MAXLINES elements, each element being a character pointer, which can point to the beginning of a string

Multi-dimensional arrays are usually less used than array of pointers, but they
are possible in C. For example, a two-dimensional array is really a simple array
with the elements being also arrays.

```C
    /* table with days per month, non-leap and leap */
    char daytab[2][13] = {
        {0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31},
        {0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
    };

    /* parameter declarations */
    f(int daytab[2][13]) { ... }
	f(int daytab[][13]) { ... }
	f(int (*daytab)[13]) { ... }
```

When passed to a function, what is passed is a pointer to an array, this time an
array of rows. This is why the number of rows is irrelevant. In general, only
the first subscript of a multi-array is "free", the rest have to be specified.

Note that `[]` have higher precedence than `*` so `int *daytab[13]` means an
array of 13 pointers to int, not pointer to an array of 13 ints, like `int
(*daytab)[13]`.

**Pointers to functions**

In C, a function is not a variables, but you can define pointers to functions
and then use them like a variable (assignment, put them in an array, pass them
to other functions). Like for array names, the `&` operator is not necessary
when referring to "addresses of functions".

```C
    /* function pointer to generic comparison function */
    int (*comp)(void *, void *);

    /* usage */
    if ((*comp)(v[i], v[left]) < 0) { ... }
```

**Complicated declarations**

TODO


## Future work

 1. Write unit tests for `01-wordcount.c` based on the comments in the file.





