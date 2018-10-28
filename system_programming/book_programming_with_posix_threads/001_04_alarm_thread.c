/*!
 * \file    001_03_alarm_thread.c
 * \brief   Async multi-thread version of an "alarm clock"
 * \note    Link with -lpthread
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <pthread.h>

typedef struct alarm_tag {
	int  seconds;
	char message[64];
} alarm_t;

void *alarm_thread(void *arg)
{
	alarm_t *alarm = (alarm_t *)arg;
	int status;

	if (alarm == NULL)
		return NULL;

	status = pthread_detach(pthread_self());
	if (status != 0) {
		fprintf(stderr, "Error in detach thread\n");
		exit(EXIT_FAILURE);
	}

	/* sleep and then "alarm" */
	sleep(alarm->seconds);
	printf("(%d) %s\n", alarm->seconds, alarm->message);
	
	free(alarm);

	return NULL;
}

int main(int argc, char *argv[])
{
	int status;
	char line[128];
	alarm_t *alarm;
	pthread_t thread;

	while (1) {
		printf("Alarm> ");
		if (fgets(line, sizeof(line), stdin) == NULL)
			exit(EXIT_SUCCESS);
		if (strlen(line) <= 1)
			continue;

		alarm = (alarm_t *) malloc(sizeof(alarm_t));
		if (alarm == NULL) {
			fprintf(stderr, "Error allocating alarm (%m)\n");
			exit(EXIT_FAILURE);
		}

		/* message of form: seconds message */
		if (sscanf(line, "%d %64[^\n]", 
					&alarm->seconds, alarm->message) < 2) {
			fprintf(stderr, "Bad command\n");
			free(alarm);
			continue;
		}

		/* create thread */
		status = pthread_create(&thread, NULL, alarm_thread, alarm);
		if (status != 0) {
			fprintf(stderr, "Error creating alarm thread\n");
			exit(EXIT_FAILURE);
		}
	}
}

