#include <stdio.h>

int main(int argc, char *argv[])
{
	int nl = (argc > 1) ? 1 : 0;

	while (--argc)
		printf((argc > 1) ? "%s " : "%s", *++argv);

	if (nl)
		printf("\n");

	return 0;
}

