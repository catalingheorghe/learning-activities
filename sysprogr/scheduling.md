## Scheduling

### Standard

Default UNIX scheduling model: round-robin time-sharing.

*Nice* values: -20 to +19 (what is above 0 is "nice", gives way to other
processes) -> weighing factor for the scheduler, not a string hierarchy.

 - `getpriority()`
 - `setpriority()`
 - shell: `nice(1)`

Note: POSIX states nice values are per process, but in Linux NPTL threading,
they are per thread. Should not be relied upon like this.

Kernel resource: Documentation/scheduler/sched-nice-design.rst - very nice read.

### Realtime process scheduling

Two scheduling policies: `SCHED_RR` and `SCHED_FIFO`. 99 priority levels.
Priority levels always matter, higher has priority. For the same priority level,
RR has a timeslice, while FIFO does not.

Note: standard rr scheduling is `SCHED_OTHER`. Other policies also exist,
including a newer `SCHED_DEADLINE`. See sched(7).

 - `sched_setscheduler() \ get`
 - `sched_setparam() \ get`

To yield the CPU ang to the back of the priority level queue, use
`sched_yield()`. Note that this will yield to runnable items of the same
priority, not lower priority.

For RR, we can see the timeslice used with `sched_rr_get_interval()`.

In a multi-cpu system, the scheduler tries to maintain a soft CPU affinity, to
not invalidate cache data. A hard affinity can be also enforced. The `isolcpus`
kernel boot option can be used to exclude some cores from the scheduling. Then
the affinity API allows to tie processes to those CPUs - `sched_setaffinity()`.

Ways to prevent a RT application from hogging up the CPU and blocking the
system:

 - set a soft CPU resource limit `RLIMIT_CPU` (`SIGXCPU` signal)
 - set an alarm timer `alarm()` that will kill the process with a `SIGALARM`
   after a wall clock time
 - create a watchdog process that runs with high prio, wakes up and inspects
   scheduling policies, parameters and CPU clock time `clock_getcpuclockid()`
 - set a resource limit for a burst of realtime process `RLIMIT_RTTIME`

Prevent a child to inherit RT policies with `SCHED_RESET_ON_FORK` flag to
`fork()`.

Kernel resource: Documentation/scheduler/sched-rt-group.rst. Describes system
tweaks in proc, for example `sched_rt_runtime_us` - how much time a realtime
groups can occupy, if set to -1, all; default allows 0.05s to non-RT tasks.

### Linux details

Linux's scheduler is named CFS (Completely Fair Scheduler), based on the notion
of *fair queuing*. Before this, before 2.6.23, it was *O(1) process scheduler*.

Timeslices are problematic - too large and you may be unresponsibe, too small
and the system may perform porely. CFS does away with timeslices. It adopts
*proportions*, which are fraction of a window of time, *target latency*. If the
target latency is 20ms, you expect each process to be run at every 20ms. The
proportions are calculated taking into account the *nice* value. To avoid
throttling, a *minimum granularity* is enforced. When this is reached, the
target latency will not be met anymore.

Linux also allows setting I/O priorities, which affect the I/O scheduler -
`ioprio_get() / set()`. Setting the nice value sets this as well, but can then
be changed. Nevertheless, *glibc* does not offer an interface to these system
calls, so they are not portable. Shell utility: `ionice`. Note that not all I/O
schedulers support priorities; the Completely Fair Queuing I/O Sched does.

Switching to real-time, some terminology. *Latency* is the period of time from
the stimulus to the response. *Jitter* is the variation in the timing of
succesive events - e.g.: stimulus every 10ms, timestamp the response, we then
measure jitter of response, not latency. Hard real-time systems usually have a
jitter of zero and a latency of the operational delay, while soft real-time
systems have a latency within the operational delay, but sometimes higher, so a
bigger jitter.

The `chrt` utility makes it easy to retrieve and set real-time attributes or to
launch processes under real-time scheduling policies.

Real-time is all about determinism, but modern computers are not as
deterministic as older systems: multiple cache levels with hits and misses,
multiple processors, paging, swapping, multitasking make it hard to determine
how long an action will take. One mitigation is locking the pages into physical
memory and prefaulting them. Another point is to prevent multitasking by
isolating the real-time process on its own CPU (because even if the scheduler is
preemptive, it can happen that it can't interrupt a critical region inside the
kernel, making your real-time process wait).

### Linux kernel

#### Process management

Process - executing program code (text), resources, memory address space, one or
more threads of execution, processor state. Threads of executions - program
counter, process stack, set of processor registers.

The kernel schedules individual threads.

Kernel nomenclature for a process: "task". Doubly linked list of *process
descriptors* of type `struct task_struct`.

States of a proces:

 - RUNNING - runnable or actually running
 - INTERRUPTIBLE - waiting for something; can be interrupted by signal
 - UNINTERRTIBLE - waitiing but won't respond to signals (D in ps)
 - TRACED - traced by another process, like a debugger, via ptrace
 - STOPPED - execution stopped

"Process context" is when the kernel is executing on behalf of a user-space
process, either via a syscall or as a response to an exception.

Forking is done via `clone()` syscall. Copy page-table entries, new process
descriptor. COW - copy on write for the address space, to spare operations if an
`exec()` follows. Kernel: `do_fork()`, `copy_process()`.

A thread in the kernel is a process that shares some resources with other
processes. Unique `task_struct` and kernel sees a thread as any other process.

A *kernel thread* is a standard processes that exists only in the kernel. They
don't have an address spaces (`mm` pointer is NULL). Can be created only by
another kernel thread (all forked of `kthreadd` kernel process).

#### Process scheduling

The scheduler policy in Unix tends to favor responsiveness, I/O-bound processes.
Linux also tends to favor this, but in a more flexible manner.

Different algorithms can be used to schedule different type of processes. This
is an ordered list: if one produces a runnable process, then that one runs. The
CFS is for normal processes (`SCHED_NORMAL`, or `SCHED_OTHER` - POSIX name).

Implementation: `sched_fair.c`. The scheduler entity structure is used for
keeping account for the time that a process runs. `update_curr()` updates this
accounting and is invoked periodically by the system timer, when a process
becomes runnable or blocks. The field updated is `vruntime`. Process selection
is simply done by the smallest `vruntime` (which takes into account number of
runnable processes and weight). It uses a *rbtree* for the list of runnable
processes - leftmost node in the tree. If no entries in the tree, the CFS will
schedule the idle task.

The entry point to the scheduler is the `schedule()` function, invoked by the
rest of the kernel.

A task going to sleep (waiting for time, i/o data, hardware event) marks itself
as sleeping and puts itself on a waitqueue, removes itself from the rbtree, and
calls `schedule()`. A wait queue is a list of processes waiting for an event to
occur (`wake_queue_head_t`). Waking up (`wake_up()`) is called usually by the
code that causes the event to occur (e.g.: data arriving, VFS layer calls
`wake_up()` on the queue).

Note that sleeping should always be handled in a loop that checks the condition,
as there can be *spurious wakeups*.

Switching from one runnable task to another - *context switching*. Function
`context_switch()` called by `schedule()` when a process has been selected to
run. Switches the virtual memory mapping and the processor state (stack,
registers, other arch specific).

Flag that tells the kernel that a rescheduling is needed: `need_resched`. Set by
`scheduler_tick()` when a process should be preempted, or `try_to_wake_up()`
when a higher prio process is awakened.

Flag is checked when returning to user-space, return from an interrupt or after
a system call. Note: kernel entry and exit code, usually in `entry.S`. This is
*user preemption*.

Kernel code execution can also be preempted (preemptive kernel). It is safe to
reschedule a task as long as it does not hold a lock. Counter `preempt_count`
for each task holds the number of locks that it is currently holding. When
returning from an interrupt to kernel code, `need_resched` and `preempt_count`
are checked. If not safe to reschedule, will continue with current kernel task.
In the unlock code, the flags are checked again. A kernel task can also
explicitly call `schedule()`, or block (which implicitly calls it). This is
*kernel preemption*.

**Real-time scheduling policies** are managed by a special real-time scheduler -
`kernel/sched_rt.c`. Soft real-time, not guaranteed to meet timing deadlines.

#### Resources

 - TLPI book (ch 35: Process control)
 - Linux System Programming, 2nd edition (ch 6: Advanced Process Management)
 - Linux Kernel Development, 3rd edition (ch 3: Process Management)
 - Linux Kernel Development, 3rd edition (ch 3: Process Scheduling)

 other referenced materials that look interesting to explore

 - Programming For the Real World (book)
 - Understanding POSIX.4 and POSIX.4a (paper)
 - Programming with POSIX threads (book)

