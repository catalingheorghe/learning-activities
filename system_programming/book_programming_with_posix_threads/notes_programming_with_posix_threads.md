# Programming with POSIX threads

## Table of Contents

0. About
1. Introduction

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
Note that a function can be reentrant but not thread-safe, but it is not common. See [this link](https://deadbeef.me/2017/09/reentrant-threadsafe) for an example. (```001_01_threadsafe_reentrant.c```)

**Concurrency control functions**

Any concurrent system mut provide a core set of functions:

 - *execution context* - the state of a concurrent entity. System can create, delete, stop, continue them.
 - *scheduling* - which context executes at any set of time and does switching between them
 - *synchronization* - mechanisms for the execution context to coordinate their use of resources ("cooperation")

### Asynchronous programming is intuitive ###

Threads have their own program counter, their own stack pointer, general registers. It does not have its own file descriptors or address space. Threads share files and memory, including text and data segments.:



