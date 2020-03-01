#include <stdio.h>

/* copy input to output; 1st version */

int main()
{
	/* NOTE: we need 'c' as an int because getchar() can return EOF, which
	 * is outside the char range, so that it cannot be confused with valid
	 * data. See also man getchar */
	int c;

	//c = getchar();
	//while (c != EOF) {
	while ((c = getchar()) != EOF) {
		putchar(c);
		//c = getchar();
	}
	/* NOTE: c = getchar() is an expression, and has the value of the left
	 * hand side after assignemnt. This value can be used in a test, like
	 * for the while loop test 
	 *
	 * The parantheses are mandtory as != has a higher precedence than = */

	/* The exact value of EOF must not be relied upon, but you can print it */
	//printf("EOF: %d\n", EOF);
}

