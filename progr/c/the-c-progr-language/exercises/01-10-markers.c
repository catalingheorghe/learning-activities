#include <stdio.h>

//#define PRINT_SUMMARY 0
#define PRINT_SUMMARY 1

int main()
{
	int c;
	long nc, nl;

	nc = 0;   /* number of characters */
	nl = 0;   /* number of lines (newline chars) */

	while ((c = getchar()) != EOF) {
		++nc;
		switch (c) {
			case '\t':
				putchar('\\');
				putchar('t');
				break;
			case '\b':
				putchar('\\');
				putchar('b');
				break;
			case '\\':
				putchar('\\');
				putchar('\\');
				break;
			case '\n':
				++nl;
				/* FALL-THROUGH */
			default:
				putchar(c);
		}
	}

	if (PRINT_SUMMARY) {
		printf("---\nchars: %ld\nlines: %ld\n", nc, nl);
	}

	return 0;
}

