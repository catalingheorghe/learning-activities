# Programming with POSIX threads

## Table of Contents

0. About
1. Introduction
2. Threads
3. Synchronization

## 0. About

Amazon books [link](https://www.amazon.com/Programming-POSIX-Threads-David-Butenhof/dp/0201633922/ref=sr_1_1?s=books&ie=UTF8&qid=1538119221&sr=1-1&keywords=programming+with+posix+threads).

## 1. Introduction

A UNIX process can be thought of as a thread plus an address space, file descriptors and other data. Having multiple threads means sharing all those extra things, including the address space.

**Definition**  
Any two operations are *asynchronous* when they can proceed independently of each other.

### The "bailing programmers"

A nice analogy between the threading model and the duties of three programmers on a damaged boat out at sea (threads, shared data/synchronization objects - mutexes, signals - condition variables).

**Definitions**

*Asynchronous* means that things happen independently (concurrently) unless there's some enforced dependency.

*Concurrency* is used to refer to things that appear to happen at the same time, but which may occur serially (threads or processes on a uniprocessor system).

*Parallelism* describes concurrent sequences that proceed simultaneously. True parallelism can occur only on a multiprocessor system, as opposed to concurrency.

*Thread-safe* means that the code can be called safely from multiple threads. The best way is to protect the data, not the code, if possible. In that way, multiple threads can execute the same function in parallel, when the threads don't use the same data at the same time. I.e.: a function that writes to a stream can be made safe by serializing the entire function, or can be made safe and efficient if each stream has an associated mutex.

*Reentrant* is sometimes used to mean "efficiently thread-safe". The idea is for a function to have defined results even if it is executed (*entered*) before an earlier execution has finished. This can usually be accomplished by making the caller provide the context which is used for saving state, like ```readdir_r, localtime_r, strtok_r ...```.  
Note that a function can be reentrant but not thread-safe, but it is not common. See [this link](https://deadbeef.me/2017/09/reentrant-threadsafe) for an example.  
Source code: (```001_01_threadsafe_reentrant.c```)

**Concurrency control functions**

Any concurrent system mut provide a core set of functions:

 - *execution context* - the state of a concurrent entity. System can create, delete, stop, continue them.
 - *scheduling* - which context executes at any set of time and does switching between them
 - *synchronization* - mechanisms for the execution context to coordinate their use of resources ("cooperation")

### Asynchronous programming is intuitive

Threads have their own program counter, their own stack pointer, general registers. It does not have its own file descriptors or address space. Threads share files and memory, including text and data segments.:

Asynchronous work is similar to real-life, so programming can be thought of as the same.

### Asynchronous programming by example

A simple "alarm CLI app" - you give it a number of seconds and a message, after that seconds it prints the message. Three variants of implementation: serial/synchronous, multi-process, multi-thread. 

In the forked vesion we use local variable to store data, and those variables will be specific to each process, so the parent can safely modify them after child creation. While in the multi-threaded version, we need to do some more bookkeeping by managing the data for each thread (malloc, free).

In the forked version, cleanup is required for terminated processes. In the threaded one, once pthread_detach was called, Pthreads will handle the cleanup of the terminated threads.

Of course, if hundreds of alarms are created, hundreds of threads are much more lightweight on the system than hundreds of processes.

Source code: ```001_02_alarm_sync.c, 001_03_alarm_fork.c, 001_04_alam_thread.c```

A more sophisticated version could use only two threads: one for input and for expiration of the next alarm. The same could be done with two processes, but with more effort to pass data.

### Benefits of threading

 1. parallelism on multiprocessor hardware
 2. concurrency - perform computations while waiting for other operaitons (like I/O)
 3. modular programming model - relationships between "events" in the program

For parallelism, scaling is predicted by *Amdahl's law* which take into account the ratio of "parallelizable code" from "total execution time" - p, and the number of precessors the code can use - n.

```
               1
 speedup = -------------
           (1 - p) + p/n
```

When the program is totally serial (p = 0), the speedup will be 1, no matter the number of cores, n. If the program requires to synchronization (p = 1), which is utopic, the speedup is the number of cores, n.

Amdahl's law is only helpful to understand scaling. It is not a practical tool because it is nearly impossible to accurately compute *p* for any program (your code, kernel code ,hardware operation).

For concurrency, the biggest advantage performance wise, is making computational progress while the blocking for operations like I/O.

Some systems have *asynchronous I/O*, but aync I/O is more complicated to use than threads, even if the system provides it. For example, the alarm clock could be written with aync reads and timer signals, but the code would be harder to understand and maintain. Async I/O does have an advantage, it is "cheaper" than threads.
1
Another method o coding an async app is for each action to be treated as an event. Events are queued by some hidden process and then dispatched to applications, usually through callback routines registered with the dispatcher. The events are dispatched sequentially. Simple applications may be easier to write and understand with this model, but complex one may prove challenging. If treating an event takes long, it can be moved to a separate thread, or sprinkled with calls to the event dispatcher.

The threading programming model makes synchronization contructs mandatory in the code, which makes the program more readable and more mainteinable. In serial programs, it is clear that function B goes after function A, so that there is no need to make the dependencies between them explicit in the code. You could do this with some comments, but comments are optional and usually get left behind when changing the code. Using, or at least considering, threading mode and synchronization constructs can make the dependencies clearer.

> An assembly language programmer can write better, more maintainable assembly code by understanding high-level language programming; a C language programmer can write better, more maintainable C code by understanding object-oriented programming. Even if you never write a threaded program, you may benefit from understanding the threaded programming model of independent functions with explicit dependencies. These are “mental models” (or that dreadfully overused word, “paradigms”) that are more or less independent of the specific code sequences you write. Cleanly isolating functionally independent code may even make sequential programs easier to understand and maintain.

### Costs of threading

 1, computing overhead (synchronization, scheduling, lower level limitation - C library, OS, file system, device driver, memory access, device controller)
 2. programming discipline (proper synchronization, using other unsafe code - libraries, no memory separation between threads)
 3. harder to debug (timing, access to memory from other threads)

> Your most powerful and portable thread debugging tool is your mind, applied through the old fashioned manual human-powered code review. You’ll probably spend a lot of time setting up a few breakpoints and examining lots of states to try to narrow the problem down a little and then carefully reading the code to find problems. It is best if you have someone available who didn’t write the code, because a lot of the worst errors are embarrassingly obvious to someone who’s not burdened with detailed knowledge of what the code was supposed to do.

### To thread or not to thread

If the problem is really a sequence of events that need to happen, then adding threads will only complicate things. Best candidates are applications that perform computations that can be parallelized and/or perform I/O that can be overlapped (like servers that handle multiple clients). Usually, threading will make the application faster and more responsive.

### POSIX thread concepts

"Pthreads" refers to POSIX 1003.1c-1995. 

Execution context is created by *pthread_create*, which also schedules the thread for execution by calling a "thread start function". Scheduling parameters can be specified at creation or even later, when running. Termination is usually via *pthread_exit* or when the thread start function terminates. The Pthreads synchronization model uses *mutexes* for protection and *condition variables* for communication. Other mechanisms can be used, such as semaphores, pipes, message queues etc. 

Regarding the actual types provided by Pthreads (pthread_t, pthread_mutex_t etc), it is important to remember that they are "opaque", i.e. code must not use any internal data or make any assumption about their representation.

Pthreads deviate form the standard UNIX and C convention that 0 represents success and failure is indicated by -1 and setting the global variable errno to a code specifying the type of error. Original POSIX standard have *errno* as an *extern int*, which supports a single execution context. Pthreads do not set *errno* and do not reserve -1 return value for error. They return 0 on success and include an extra output parameter for storing results. An error code from *errno.h* is returned instead of 0 on error. Pthreads also provides a per-thread *errno* for supporting other code that uses it. This means that *errno* can still be used as normal inside any thread because each thread has its copy.  
Source code: ```001_05_thread_error.c```

The one exception to the error rules above is *pthread_get_specific*, which does not report errors at all, since it has to be as fast as possible.

## 2. Threads

### Creating and using threads

A thread is created with ```pthread_create``` and begins by calling a function that you provide. This function has a single argument, of type ```void *``` and returns a value of the same type. The identifier returned in the *thread* argument is of type *pthread_t*. A thread can find out its own identifier using *pthread_self*. There is no way to find out the identifier or another thread, if you don't store it yourself.

To test equality of two thread identifiers, Pthreads provides ```pthread_equal```.

The *main thread* is the one that executes main. It is special because Pthreads retains traditional UNIX process behavior when the function main returns: the process terminates without waiting for other threads to complete.

You should always detach each thread when there is no more need for it. "Undetached" (threads that have terminated but have not been detached) retain virtual memory, including their stacks, as well as other system resources. Detaching lets the system take back those resources. A thread may detach its-self, or can be detached by any other thread, using ```pthread_detach``` function. If you need to know the return value of a thread execution, or when it has finished, use ```pthread_join```. It will block until the thread finishes, then optionally, store the return value. The join automatically *detaches* the thread.

You need to make sure that the process runs until created threads complete. ```pthread_join``` can be used for this, but others way may also be used (waiting for some data for example).

It is a good idea to always return something from the thread function, even if it is NULL; ```pthread_join``` will always put some value in the return argument, if instructed to, so it will take whatever happens to be in the place where the thread's start function would have stored a return value.

Once pthread_join is called, calling it again is not allowed, the *joinee* thread has already been detached.

If more than one thread is interested in the return value of a thread, join cannot be used. Another way must be implemented, like using a condition variable that the terminating thread can signal to let interested threads know that the data is available.

### The life of a thread

There are four basic states in which a thread can be. Systems usually provide "sub-states" or "additional states", but those usually only distinguish between resons for entering the basic states.

 - Ready: able to run but waiting for a processor
 - Running: currently running (on a multiprocessor, may be more than one)
 - Blocked: not able to run because it is waiting for something
 - Terminated: has terminated, not detached and has not yet been joined; after this, it is *recycled*

**Creation**

The initial/main thread of a process is created when a process is started. It holds registers, program counter, stack pointer etc needed to execute code. Additional threads are created by explicit calls to *pthread_create*. Threads may also be created as result of receiving a signal if the signal disposition is set to ```SIGEV_THREAD```.

The state of a new thread is *ready*. It may remain in that state for a period, depending on the scheduler.

*IMPORTANT*  
There is no synchronization between the creating thread's return from *pthread_create* and the scheduling of the new thread. The thread may start *before* the creating thread returns. It may even run to completion and terminate before *pthread_create* returns.

**Startup**

When it eventually starts executing instructions, it will execute the thread start function with the argument provided.

In the initial thread, the thread start function (```main```) is called from outside your program. The arguments to it are also different: argvc and argv, not a single void* argument. When the initial thread returns from main, the process will be terminated. 

*NOTE*  
If you want to terminate the initial thread while allowing other threads to continue running, call *pthread_exit* instead of returning from main.

Also, on many systems, the main thread's stack, being the process default stack, will be larger than the stack of other threads.

**Running and blocking**


If a thread needs a resource that is not availbale, it becomes *blocked*. If the system gives the processor away to another execution context, it is *preempted* (like in timeslicing), so it is in state *ready*. When a thread is unblocked so that is eligible to run, it is also in *ready* state. 

A thread goes in *running* state when it the scheduler assigns it to a processor. The processor saves the context of the blocking or preempted thread and restores the context of the ready thread, so that it can run.

Example of becoming blocked: lock a mutex that is locked, wait on condition variable, calling sigwat for a signal that is not currently pending, I/O operation that cannot be immediately completed, or other system operations, such as a page fault.

When a thread is unblocked after a wait for an event, it is made *ready*.

**Termination**

Usual termination is by returning from the start function. Threads that call ```pthread_exit``` or that are cancelled using ```pthread_cancel``` also terminate after calling each cleanup handler that the thread registered by calling ```pthread_cleanup_push``` (and that hasn't yet been removed by ```pthread_cleanup_pop```).

If threads have any *thread-specific data* values that are non-NULL, the associated destructor functions for those keys are called, if any.

if the thread was already detached, it moves straight to the next stage: *recycling*. Otherwise it becomes *terminated* and will remain available for another thread to join with i. The recommendation is that if you create threads that you don't intend to joing with, create them detached (thread attribute).

The only external difference between a thread that terminate on its-own and a thread that was cancelled, is that the return value is always ```PTHREAD_CANCELLED``` for the latter.

Once ```pthread_join``` has extracted the return value, the terminated thread is detached by pthread_join, and may be recycled before the call to pthread_join returns. 

*NOTE*  
The returned value should never be a stack address from the terminated thread's stack.

Even if you need values from a created thread, it usually makes more sense to pass that data in another way. pthread_join is not a must, other ways to pass return values can be used by the application.

**Recycling**

Recycling releases any system or process resources left. Like the storage used for the thread's return value, the stack, memory for register state etc. Some of these may have been released at termination; none should be accessed after termination.

*NOTE*  
Once a thread is recycled, the thread ID (pthread_t) is no longer valid. The terminated thread's ID may be assigned to a new thread. You could cancel a different thread, for example.

You must release program resources owned by the thread. Dynamic memory can be freed at any time, from any thread. Mutexes, condition variables, semaphores may be destroyed by any thread, as long as they are unlocked. If a thread terminates while having a mutex locked, that mutex won't be usable anymore, since it should have been unlocked by that thread.

## 3. Synchronization

### Critical sections and invariants

*Invariants* are assumptions made by a program, especially assumptions about the relationships between sets of variables. For example, the invariant of a queue package are: the queue header must either be NULL or contain a pointer to the first queued data element; each data element must contain a pointer to the next data element, or NULL if it is the last.

*Critical sections* (or serial regions) are areas of code that affect a shared state. It almost always translates to a data invariant. For example, the code performing the removal of an element from the queue can be seen as a critical section in the code, or you can see the state of the queue as an invariant. For a while, during the removal, the invariant is broke, but through syncrhonization, it is "repaired" before other code encounters it.

*Predicates* are logical expressions that describe the sate of invariants needed by your code. Example: a boolen value, a comparison with NULL, if a counter is greater than somethin etc.


