#include <stdio.h>

#define MAXLINE 1000

int mygetline(char s[], int lim);
void reverse(char s[]);

int main()
{
	char line[MAXLINE];

	while (mygetline(line, MAXLINE) > 0) {
		reverse(line);
		printf("%s", line);
	}

	return 0;
}

int mygetline(char s[], int lim)
{
	char c;
	int i, j;

	j = 0;
	for (i = 0; (c = getchar()) != EOF && c != '\n'; i++) {
		if (i < lim-2) {
			s[j] = c;
			j++;
		}
	}
	if (c == '\n') {
		s[j] = c;
		j++;
		i++;
	}
	s[j] = '\0';
	return i;
}

void reverse(char s[])
{
	int i, j;
	char c;

	for (i = 0; s[i] != '\n' && s[i] != '\0'; i++)
		;

	j = 0;
	for (j = 0, i--; j < i; j++, i--) {
		c = s[j];
		s[j] = s[i];
		s[i] = c;
	}
}


