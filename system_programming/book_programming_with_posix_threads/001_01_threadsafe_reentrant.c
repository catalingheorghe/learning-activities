/*!
 * \file    001_01_threadsafe_reentrant.c
 * \brief   Thread-safe vs reentrant functions
 * \details https://deadbeef.me/2017/09/reentrant-threadsafe
 * \note    compile with "-ldl -rdynamic" for dladdr to work
 */

#include <stdio.h>
#include <stdlib.h>

#define _GNU_SOURCE
#define __USE_GNU
#include <dlfcn.h> /* dladdr */

typedef void (*swap_func)(int*, int*);

/* global flag to trigger a reentrant call */
static int reentrant_call = 0;

/*
 * Not reentrant, not thread-safe
 *
 * not thread-safe
 *   - global variable changed unprotected
 *
 * not reentrant
 *   - glbal variable
 *   - my_func() could be called while swap() is running in same context, which
 *   will lead to the value of t to be unpredictable
 *
 */
int t;

void swap_basic(int *x, int *y)
{
	t = *x;
	*x = *y;
	/* my_func could be called here */
	*y = t;
}

/*
 * Not reentrant, thread-safe
 *
 * thread-safe
 *   - tlocal - thread local storage
 *
 * not reentrant
 *   - my_func() could be called while swap() is running in same context, which
 *   will lead to the value of tlocal to be unpredictable
 *
 */
__thread int tlocal;

void swap_tsafe(int *x, int *y)
{
	tlocal = *x;
	*x = *y;
	/* my_func could be called here */
	if (reentrant_call)
	{
		int a = 10, b = 20;
		printf("<reentrant call>\n");
		reentrant_call = 0;
		swap_tsafe(&a, &b);
	}
	*y = tlocal;
}

/*
 * reentrant, not thread-safe
 *
 * not thread-safe
 *   - global variable
 *
 * reentrant
 *   - global data is the same when the program enters or leaves swap
 *
 */
void swap_reentrant(int *x, int *y)
{
	int s;

	/* save global variable ("context") */
	s = t;

	t = *x;
	*x = *y;
	/* my_func could be called here */
	if (reentrant_call)
	{
		int a = 10, b = 20;
		printf("<reentrant call>\n");
		reentrant_call = 0;
		swap_reentrant(&a, &b);
	}
	*y = t;

	/* restore global variable ("context") */
	t = s;
}

/*
 * reentrant, thread-safe
 *
 * thread-safe
 *   - data on the stack / no global data
 *
 * reentrant
 *   - data on the stack / no global data
 *
 */
void swap_safe(int *x, int *y)
{
	int tmp;

	tmp = *x;
	*x= *y;
	/* my_func could be called here */
	if (reentrant_call)
	{
		int a = 10, b = 20;
		printf("<reentrant call>\n");
		reentrant_call = 0;
		swap_safe(&a, &b);
	}
	*y = tmp;
}

int identify_function_ptr(void *func, char * const name, size_t len)
{
	Dl_info info;
	int rc;

	if (func == NULL)
		return -1;

	rc = dladdr(func, &info);
	if (!rc) {
		fprintf(stderr, "Error retreiving info for %p: %s\n", 
				func, dlerror());
		return -1;
	}

	if (name == NULL)
		return -1;

	snprintf(name, len, "%s", info.dli_sname);

	return 0;
}

void my_func(swap_func function)
{
	char fname[64] = { 0 };
	int x = 1, y = 2;

	if (function == NULL) {
		fprintf(stderr, "Invalid parameter: function is NULL.\n");
		return;
	}

	if (!identify_function_ptr(function, fname, sizeof(fname))) {
		printf("swap function: %s\n", fname);
	} else {
		printf("swap function ptr: %p\n", function);
	}

	printf("[before swap] x: %d, y: %d\n", x, y);

	function(&x, &y);
	
	printf("[after swap ] x: %d, y: %d\n", x, y);
	printf("\n");
}

int main(int argc, char *argv[])
{
	my_func(swap_basic);

	reentrant_call = 0;
	my_func(swap_tsafe);
	reentrant_call = 1;
	my_func(swap_tsafe);

	reentrant_call = 0;
	my_func(swap_reentrant);
	reentrant_call = 1;
	my_func(swap_reentrant);

	reentrant_call = 0;
	my_func(swap_safe);
	reentrant_call = 1;
	my_func(swap_safe);

	return EXIT_SUCCESS;
}

