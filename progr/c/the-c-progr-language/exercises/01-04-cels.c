#include <stdio.h>

int main()
{
	float fahr, celsius;
	int lower, upper, step;

	lower = -40;  /* lower limit of temp table (celsius) */
	upper = 100;  /* upper limit                         */
	step = 10;    /* step size                           */

	printf("Temperature conversion table\n");
	printf("  Celsius to Fahrenheit\n");
	printf("  F = (9/5 * C) + 32\n");
	printf("\n");

	printf("%6s  %6s\n", "C", "F");
	printf("--------------\n");

	for (celsius = lower; celsius <= upper; celsius += step) {
		fahr =  ((9.0/5.0) * celsius) + 32.0;
		printf("%6.0f  %6.1f\n", celsius, fahr);
	}
}
