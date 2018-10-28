/*!
 * \file    001_03_alarm_fork.c
 * \brief   Async multi-process version of an "alarm clock"
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>

int main(int argc, char *argv[])
{
	int seconds;
	char line[128];
	pid_t pid;
	char message[64];

	while (1) {
		printf("Alarm> ");
		if (fgets(line, sizeof(line), stdin) == NULL)
			exit(EXIT_SUCCESS);
		if (strlen(line) <= 1)
			continue;
		/* child reaping should be done in the above cases as well */

		/* message of form: seconds message */
		if (sscanf(line, "%d %64[^\n]", &seconds, message) < 2) {
			fprintf(stderr, "Bad command\n");
			continue;
		}

		/* fork */
		pid = fork();
		if (pid == (pid_t) -1) {
			fprintf(stderr, "Error creating child (%m)\n");
			continue;
		}
		if (pid == (pid_t) 0) {
			/* child */
			/* sleep and then "alarm" */
			sleep(seconds);
			printf("(%d) %s\n", seconds, message);
		} else {
			/* parent */
			/* reap any child processes that have finished */
			do {
				pid = waitpid((pid_t) -1, NULL, WNOHANG);
				if (pid == (pid_t) -1)
					fprintf(stderr, "Error waiting for child (%m)\n");
			} while (pid != (pid_t) 0);
		}

	}
}

