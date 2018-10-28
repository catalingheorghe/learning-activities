/*!
 * \file    001_02_alarm_sync.c
 * \brief   Synchronous version of an "alarm clock"
 * \details Only one alarm request can be active at a time
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

int main(int argc, char *argv[])
{
	int seconds;
	char line[128];
	char message[64];

	while (1) {
		printf("Alarm> ");
		if (fgets(line, sizeof(line), stdin) == NULL)
			exit(EXIT_SUCCESS);
		if (strlen(line) <= 1)
			continue;

		/* message of form: seconds message */
		if (sscanf(line, "%d %64[^\n]", &seconds, message) < 2) {
			fprintf(stderr, "Bad command\n");
			continue;
		}

		/* sleep and then "alarm" */
		sleep(seconds);
		printf("(%d) %s\n", seconds, message);
	}
}

