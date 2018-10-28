/*!
 * \file    001_05_thread_error.c
 * \brief   Example of error reporting used by Pthreads
 * \note    Compile with -wno-uninitialized, link with -lpthread
 */

#include <stdio.h>
#include <string.h>
#include <pthread.h>

#include <errno.h>

int main(int argc, char *argv[])
{
	int status;
	pthread_t thread;

	/* attempt to join with an unitialized identifier; this should give out an
	 * ESRCH error code. If it happens to be the thread ID of the main thread
	 * then it should give EDEADLK or hang */
	status = pthread_join(thread, NULL);
	if (status != 0) {
		/* save errno, it may be overwritten by fprintf */
		int local_errno = errno;

		fprintf(stderr, "Error joining (%d %s)\n", status, strerror(status));
		
		/* restore errno and see what it contains */
		errno = local_errno; 
		perror("perror output");
		/* pthreads does not use errno, so it will be success */
	}

	return status;
}

