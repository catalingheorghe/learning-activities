/*!
 * \file    errors_debug.h
 * \brief   Error reporting and debug utilities
 * \note    -DDEBUG activates debug prints
 */
#ifndef _INCLUDE_ERRORS_DEBUG_H
#define _INCLUDE_ERRORS_DEBUG_H

#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

/* macro that can be used to enable diagnostic prints when code is compiled
 * with -DDEBUG */
#ifdef DEBUG
# define DPRINTF(arg...) printf(arg)
#else
# define DPRINTF(arg...)
#endif

/* macro to print error string of given code and abort */
#define err_abort(code, text) do { \
	fprintf(stderr, "%s at \"%s\":%d: %s\n", \
			text, __FILE__, __LINE__, strerror(code)); \
	abort(); \
	} while (0)

/* macro to print error string of errno and abort */
#define errno_abort(text) err_abort(errno, text)

#endif /* _INCLUDE_ERRORS_DEBUG_H */
