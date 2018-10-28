/*!
 * \file    002_01_thread_life.c
 * \brief   Simple thread creation example, with a twist for main thread's life
 * \note    Link with -lpthread
 */

#include <stdio.h>
#include <unistd.h>
#include <pthread.h>

#include "errors_debug.h"

void *thread_routine(void *arg)
{
	/* detach thy-self, no need to be waited on by anyone */
	pthread_detach(pthread_self());

	sleep(3);

	printf("Bye bye from child thread\n");

	return arg;
}


int main(int argc, char *argv[])
{
	int status;
	pthread_t thread;

	status = pthread_create(&thread, NULL, thread_routine, NULL);
	if (status != 0)
		err_abort(status, "Create thread");

	/* this would fail because thread is already detached */
	/*
	 * pthread_join(thread, &thread_result);
	 */

	printf("Bye bye from main thread\n");

	/* calling pthread_exit instead of returning from main will let the 
	 * process go on until the last thread terminates */
	pthread_exit(NULL);
	/*
	 * return 0;
	 */
}

