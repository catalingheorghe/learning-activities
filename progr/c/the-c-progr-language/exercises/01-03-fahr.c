#include <stdio.h>

#define LOWER  -40  /* lower limit of temp table (fahr) */
#define UPPER  220  /* upper limit                      */
#define STEP    20  /* step size                        */

int main()
{
	float fahr, celsius;

	printf("Temperature conversion table\n");
	printf("  Fahrenheit to Celsius\n");
	printf("  C = 5/9 * (F-32)\n");
	printf("\n");

	printf("%6s  %6s\n", "F", "C");
	printf("--------------\n");

	for (fahr = LOWER; fahr <= UPPER; fahr += STEP) {
		celsius = (5.0/9.0) * (fahr-32.0);
		printf("%6.0f  %6.1f\n", fahr, celsius);
	}

	return 0;
}

