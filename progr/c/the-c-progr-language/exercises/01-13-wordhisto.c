#include <stdio.h>

#define	IN	1            /* inside a word */
#define	OUT	0            /* outside a word */

#define MAXWORDLEN 20    /* maximum chars in a word to consider */

#define MAXHISTWIDTH 60  /* maximum width of a histogram line */

int main()
{
	int c, state, ncw, i, max, j, width;
	int nwords[MAXWORDLEN] = { 0 };

	state = OUT;  /* initially, "outside" of a word */
	ncw = 0;      /* number of chars in a word */

	while ((c = getchar()) != EOF) {
		if (c == ' ' || c == '\n' || c == '\t') {
			if (state == IN) {
				if (ncw > 0 && ncw < (MAXWORDLEN-1)) {
					++nwords[ncw-1];
				} else if (ncw >= MAXWORDLEN) {
					++nwords[MAXWORDLEN];
				}
			}
			state = OUT;
			ncw = 0;
		} else {
			state = IN;
			++ncw;
		}
	}

	/* print the number of words per length */
	for (i = 0; i < (MAXWORDLEN-1); i++) {
		printf(" %2d: %3d\n", (i+1), nwords[i]);
	}
	printf(">%2d: %3d\n", (i+1), nwords[i]);

	/* print an horizontal histogram */
	max = 0;

	for (i = 0; i < MAXWORDLEN; i++) {
		if (nwords[i] > max) {
			max = nwords[i];
		}
	}

	width = 0;    /* width of current histogram line, related to max */
	
	for (i = 0; i < MAXWORDLEN; i++) {
		width = (nwords[i] * MAXHISTWIDTH) / max;
		if (width == 0 && nwords[i] > 0) {
			width = 1;
		}
		printf(" %2d: ", (i+1));
		for (j = 0; j < width; j++) {
			printf("#");
		}
		printf("\n");
	}

	return 0;
}

