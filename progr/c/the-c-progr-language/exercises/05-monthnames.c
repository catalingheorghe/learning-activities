#include <stdio.h>
#include <string.h>

int main()
{
	int len;
	long unsigned space;

	char *pnames[] = {
		"Illegal month",
		"Jan", "Feb", "Mar", "Apr", "May", "Jun",
		"Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
	};

	char anames[][15] = {
		"Illegal month",
		"Jan", "Feb", "Mar", "Apr", "May", "Jun",
		"Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
	};

	printf("Sizeof multi-array   : %4lu bytes\n", sizeof(anames));
	space = sizeof(anames);
	printf("  storage            : %4lu bytes\n", space);

	printf("Sizeof pointers array: %4lu bytes\n", sizeof(pnames));
	space = 0;
	len = sizeof(pnames)/sizeof(pnames[0]);
	for ( ; len > 0; --len) {
		space += strlen(pnames[len-1]);
	}
	space += sizeof(pnames);
	printf("  storage            : %4lu bytes\n", space);

	{
		/* other
		 * using a pointer to 15 character array to go through
		 * the multi-dimensional array */
		char (*name)[15] = &anames[0];
		/*              or anames, or even &anames */
		printf("\nfirst two elements of multi-array:\n");
		printf("  %s, ", (char *) name);
		name++;
		printf("%s\n", (char *) name);
	}

	return 0;
}
