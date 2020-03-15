## String Handling

Understand the difference between the following definitions

```C
    char *pname = "test name";
    char aname[] = "test name";
```

---------------------------------------------------------------

Always match your declarations with definitions.

```C
   /* file1 - definition */
   char s[10];

   /* file2, or header - declaration */
   extern char *s;    /* not the same thing */
```

---------------------------------------------------------------

Don't confuse a char constant `'x'` with a string constant `"x"`.

---------------------------------------------------------------

Use safer alternatives for parsing numbers into strings. Instead of `atoi()` 
and `atof()`, `strtol()` and `strtod()` allow error checking.

---------------------------------------------------------------

Remember that *n* string functions do not guarantee that the destination will
be a NUL terminated string. If *n* is the size of the destination string and
there isn't a NUL char in the first *n* positions of the source string, the 
result will be a full, not NUL terminated character array.

---------------------------------------------------------------

Only memmove can handle overlapping buffers.

---------------------------------------------------------------

Use `asprintf()` to deal with memory allocation for string of various formats.

Note that it is not a C standard library function but it is part of GNU C library
and BSD, so it is widely available. If missing can be easily implemented; as a
tip, you can use `vsnprintf()` with a one byte char array to calculate the length
required for the string, without going into the complexity of handling printf
format.

Remember to free the returned memory location.

---------------------------------------------------------------

Use `strtok_r()` or `strtok_s()` instead of `strtok()` to make sure your code
is and remains thread-safe.

---------------------------------------------------------------

`strlen()` also has a *n* alternative - `strnlen()`, to handle strings that may
not be NUL-terminated

---------------------------------------------------------------

General: check that strings from untrusted sources are of sane length and format
before passing them on to other functions.

General: text encoding in C is UTF-8. When dealing with different encoding, you
need to get the locale of the host and based on that, transform it to UTF-8. When
wirting files, you can do the reverse transform the UTF-8 text into the host locale.

POSIX includes `iconv` function for character set transformations. GLib offers
utility functions on top of that for easy transformation.

Wide char datatype has been in C before Unicode (`wchar_t`), so it does not map
specifically to any UTF encoding (8, 16 or 32). Its width is not fixed by the C
standard, so it is not very useful. C11 provides `char16_t` and `char32_t`, but not
much code is written using them.

