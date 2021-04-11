## Real-Time Linux

### General

Linux does not come with any real-time guarantee, but it can be enhancened.
Remember though that real-time is not speed, but guaranteed timing. A real-time
system is characterized by the fact that the computations are deemed correct
also on the basis of their response time, not only on their result.

Consider first if your application requires real-time. Things to look for: are
deadlines that need to be respected, or imposed by other devices or system? does
application code need to have more priority than OS code? what happens if a
deadline is missed, does it make the system unsafe? How small are the timings we
are dealing with, are they fine-grained?

Example of *latency*: amount of time between an interrupt and the execution of
the associated interrupt handler - *interrupt latency*. *Dispatch latency*
includes the work that an interrupt handler may defer for later processing.
*Worst-case latency* takes into account the maximum amount that could intervene
(from bus contention and DMA to interrupt masking and software scheduling).

If the kernel does not have enough rescheduling points (safe to reschedule),
then it is more likely to get into a *priority inversion* situation, in which it
is serving a lower priority task.

Software requirements for real-time

 - small *preemption granularity* - time it takes for kernel to switch from
   low-priority to high-priority contexes. The switch to preemptible kernel in
   2.5 made average preemptability better, but worst-case can still be bad
   because of non-preemptibility when holding locks, any locks, no matter if
   they are required by the interrupting task or not

 - handling priority inversion when dealing with same exclusive resources

 - bounded handling of external events - reaction to interrupts; avoid running
   with interrupts disabled for a long period of time, possibly unbounded (like
   it could be case in networking or filesystem layer)

Note that real-time data acquisition is not like regular I/O, where packets may
be missed, resent, the partner can wait. Data acquisition usually requires
precise reliability.

Two **approaches** to adding real-time hooks into a GPOS (general purpose OS)
like Linux

 - co-kernel approach - run a RTOS kernel alongside Linux, use a virtual PIC
   (programmable interrupt conroller - in software) to hold and distribute the
   interrupts, run real-time processes in that kernel. Xenomai, RTLinux/GPL are
   examples of this. Predictability is good, but the drawback is that you don't
   benefit from new drivers for new devices; also the programming model could be
   complex because you can't rely on services from the standard C library.

 - fully preemptible kernel - even though Linux is huge, the core parts are not
   that big, and the ones that cause most problems are interrupts and locking.
   These are the main focus of changing to allow real-time processes to run
   without interference from unbounded activities of non-real-time processes. The
   RT patch turns Linux into a RTOS - deals with large areas where preemption is
   disabled, mutexes that can cause prio inversion, interrupts that can preempt
   any real-time process. Note that Linux is still a GPOS and if the system
   needs to deal with life-critical operations, it is not the way to go - hard
   to audit the code and prove that it is bug free.

Excerpt from Building Embedded Linux Systems

*The Linux kernel is indeed very large. It contains several million lines of
code, but most of that lies in drivers, as well as all the different
architectures that it supports. No one has a machine that contains all the
devices that the Linux kernel supports. When choosing to use the RT patch for an
embedded real-time platform, one only needs to audit the drivers that are used
for that platform. Drivers must be inspected by a trained RT kernel programmer
to search for areas that might disable preemption or interrupts for long periods
of time.*

Another excerpt from Building Embedded Linux Systems

*When using Linux with the RT patch for mission-critical systems, you should
remove as many of the unknowns as possible. This means that only the necessary
drivers should be loaded, and they must be audited to see that they do not
disable interrupts for long periods of time. All kernel configuration options
that you set need to be understood. There is no easy way out of thoroughly
researching your software when setting up a real-time mission-critical system.
The RT patch made Linux much more real-time friendly, but Linux is still itself
focused on being a general-purpose operating system. A real-time administrator
needs to focus on Linux as a tool for the system and try to understand it as
much as possible*

### The RT Patch (`PREEMPT_RT`)

**Interrupts as threads**

ISRs interrupt any user task. They execute with interrupts disabled, or at the
minimum with that interrupted masked off. Only thing that can preempt an ISR is
another ISR, if it turns interrupts back ok. Device drivers should do as little
as possible in ISR and push things to kernel threads, tasklets or softirqs.

A hard IRQ is a sort of envelope around an ISR, lasting from the time the
interrupt preempts the CPU to the time the ISR returns the CPU. To keep this to
a bare minimum, the RT patch transforms the ISRs to kernel threads - the small
remaining ISR will just wake up a kernel thread which will run the function
registered. The small remaining ISR will still also mask the interrupt line.
When is it unmasked? As far as I understand, this should happen after the
orignal ISR has been executed - to maintain the idea that it was a top-half.
However, you can also request threaded interrupts in a normal kernel, and in
this case they will be unmasked immediately (unless you use `IRQF_ONESHOT` to
say that you want this done after the threaded part has executed).

Note that some handlers still run as normal ISRs, as is the timer interrupt.

Threads created to service interrupts are named *IRQ-n*. Can be seen in `ps` and
can be correlated with `/proc/interrupts`. Default prio is 50 - easy to change
with `chrt` utility.

Regarding CPU affinity, the interrupt service thread is set every time to the
affinity of the interrupt.

softirqs can cause even greater latency, since they are lenghtier, and can
interrupt any kernel thread. Initially the RT patch just made all softirqs run
under *ksoftirqd*, but that did not provide much prioritization granularity.
Then the approach was to create a thread for each softirq, for each CPU
(*softirq-name/n*). The CPU affinities match the CPUs and should not be changed
(`taskset` utility).

In 3.0 kernel, the handling for softirqs went back from multiple threads to the
mainline approach of *ksoftirqd*, which was of course given rt scheduling. Then
it was changed again, in 3.6, to include an optimization that the thread that
raises a specific softirq will also handle it immediately. So, the kernel thread
created for the network ISR, if raises a softirq, when finishing the ISR
handling part, will check that and it will execute the softirq it raised in its
context.

Softirq timer threads are very important. The *softirq-timer* thread handles
most timeouts in a system and is usually not critical to be executed on-time,
even in RT patched systems. The *softirq-hrtimer* thread handles POSIX timer
implementation - `timer_create()`; it must have higher priority than the process
setting up the timer, or it must dynamically change the priority to match it.

Note that the `nanosleep` syscall is not affected by softirq-hrtimer; it is
handled in the ISR of the timer, so processes will be woken up directly from the
interrupt. `sleep` usage falls under the same behavior.

**Priority inheritance**

Note that priority inversion is really only a problem when it is unbounded, as
it is in the classic example using three process, the middle one interrupting
the lower one, which is holding a lock that the higher one is waiting for.

Priority inheritence in use for futexes in mainline kernel came from the RT
patch. The RT patch uses the priority inheritance code for fast userland mutexes
for internal kernel locks also.

If a thread holding a futex terminates then it can leave others blocked.
Solution for this is the implementation of a *robust futex* in the kernel.

In POSIX library, `PTHREAD_PRIO_INHERIT` is the flag that enables priority
inheritance for a mutex.

**Kernel config - spin locks as mutexes**

`CONFIG_PREEMPT_RT` - "Complete Preemption (Real-Time)". Turns spin locks into
mutexes. Spin locks are busy loops, so they have less overhead if the code
guarded is small, but this is not always the case. Converting them to mutexes,
so the thread trying to get one is scheduled out and rescheduled when the lock
is released, may increase overhead, but also increases responsiveness to the
waking up of a high-priority process.

As interrupt handlers and softirqs use spin locks quite often, converting them
to kernel threads is also required, and implicit. Note that the reverse is not
true: you can select threading interrupts without "Complete Preemption".

From the real-time patch came the "lockdep" code which detects possible race
conditions - a lock analyzer - between process context and interrupt handlers.

**High-Resolution Timers**

In early 2.6 Linux, the smallest unit of time was a *jiffy*, which was chosen as
one 100th of a second - `HZ` global variable with a value of 100. As demands
increased, it was set to 1000 - best reaction of time would be 1 ms. For real
time, this is still not enough, but increasing it would also mean increasing the
number of timer interrupts, as each jiffy requires one.

The first improvement was differentiating between *action timers*, which the
user sets and expects to expire, and *timeout timers*, which usually are deleted
before expiring. The existing infrastructure for timers, the time wheel, was
good for adding and deleting quickly, so it was kept for timeouts. But for
*action timers*, a new approach, *hrtimers*, was introduced. Using a red-black
tree that keeps timers sorted, even if addition and deletion is more costly.

The second one was switching from relying on jiffies to an actual clock
infrastructure. The value for an hrtimer event is in nanoseconds; this is
converted to the underlying clock resolution, so it depends on the hardware. The
clock is configured to go off at the next required event.

This meant that constant timer interrupts to update jiffies were no longer
required, which can allow embedded systems to go into power saving. *Dynamic
ticks*, when enabled, triggers the timer interrupt only when required. In an
idle state, nothing will happen - note that the number of jiffies "skipped" is
still kept track of and added at the next wakeup. When the system is not idle,
the timer interrupt is still set up to go off once per jiffy, because the jiffy
variable is used for schedule accounting of non-real-time tasks.

The config `NO_HZ_FULL` turns the timer interrupt of, so if there is a single
process in userspace, kernel won't wake up for each timer interrupt. Still there
is some administration and accounting needed, so it fires once per second. Work
in progress to get rid even of that.

**RT processes and non-real time processes**

There is a proc entry that controls how much of a period can real-time processes
run, before giving some time to non real-time: `sched_rt_runtime_us` - it is by
default 95% (950000 micro seconds of the period which is 1 s). This is to avoid
real-time taking over totally.

Steven Rostedt recommends in a 2016 Kernel Recipes talk to disable this on a
system intended to do real-time.

Note that when this limit is reached, the kernel will print something on the
console to say that throttling has kicked in.

### Tracing and testing

**Tracer**

Initially, a Latency Tracer was introduced. Main focus was: event tracing,
function tracing, wake-up tracing. It ended up being replaced with ftrace, but
in a similar manner.

**hackbench**

hackbench is a tool that can be used to put load on a system, load under which
you can then do further real-time relevant tests, like interrupt testing.

The tool can create a given number of process groups, each group having 20
processes which exchange hundreds of messages between them. So it involved a lot
of scheduling, which also takes up CPU, and memory consumption.

**OSADL latency box**

Triggers interrupts at a programmable frequency, reacts on an input pin and then
provides a measure of the interrupt latency. These kind of test should be done
for at least 12 hours, and under load, to try to see the worst case scenario.

### A real-time system 

Starts with hardware - a lot of optimizations in the CPU for being "real fast"
are not good for real-time, as there can be outliers which through you out of
bounds.

 - memory cache - try to run tests with cold cache, try to find worst case
   scenario
   note: reader-writer locks can kill the cache (should look more into this)

 - branch prediction - when branch prediction is wrong it can have a big impact
   `perf-stat` can show branch misses - and you can see if it is high when your
   test is failing

 - NUMA - large machines - need to organize the tasks

 - hyper-threading - one exec engine, two logical pipelines and two register
   sets, one cache -> not good for RT. Should be turned off.

 - TLB - cache of page table entries - maybe try to flush the TLB all the time
   to induce TLB misses - go for worst case scenario

 - transactional memory - instead of grabbing a lock, work in a critical section
   on an area, start a transaction; if another CPU accesses it, go back and grab
   a lock

 - SMI - system management interrupt - e.g.: every 14 minutes system performs an
   ECC scan on the memories
   Tool: hardware latency detector
   [link](https://www.kernel.org/doc/html/latest/trace/hwlat_detector.html)
   
 - CPU frequency scaling - going into deep sleep (power mgmt), can affect your
   latency because it takes longer to wake up

A RT kernel

 - threaded interrupts - know what interrupts your application is using

 - soft interrupts - as mentioned earlier, softirqs run in the context of who
   raises them. Exceptions are RCU and Timers softirq - they are run in
   `ksoftirqd` and `ktimer_softd`.

 - system management threads - RCU, watchdog, migrate, kworker, ksoftirqd,
   posixcputimer 

 - timers - `ktimer_softd` and `posixcputimer` threads

 - cpu isolation - isolcpus kernel parameter; or cpuset via proc

 - `CONFIG_NO_HZ` - when it goes idle, it turns off the clock -> good for
   power saving, bad for latency. This should be turned off on a RT kernel.
   cyclictest on an idle machine can show this

 - `CONFIG_NO_HZ_FULL` - lets a task in userspace to run without the kernel
   interrupting

Library and application

 - memory locking - `mlockall()` - avoid page-faults
   note: overcommit is not a problem per-se, but you should make sure you
   real-time tasks are not overcommiting

 - priority inheritance - mutex attribute

 - kernel threads vs your process dependencies

 - real-time vs multiprocessors - if your RT app goes from CPU to CPU, it can
   kill your bounds

 - RR scheduling - there is no requirement for the timeslice; furthermore, with
   3 processes RR same prio, 2 CPUs, you will get 2 of them running 50 50 on one
   CPU and the 3rd running 100% on the second CPU; don't use `sched_yield()` -
   it can only be useful when wanting to give another same priority FIFO process
   the CPU
   Note: if you don't have special reasons, don't use `SCHED_RR`, use
   `SCHED_FIFO`

 - `SCHED_DEADLINE` - new policy; to be detailed later

## Resources

- Building Embedded Linux Systems, 2nd Edition (2008) - Chapter 12

- real-time, SMP and -rt patchset (~2007) 
  [link](https://www.linuxjournal.com/article/9361)

- Building Embedded Linux Systems, 2nd Edition (2008) - Chapter 14

- softirqs and real time, back to ksoftirqd and then changed again
  https://lwn.net/Articles/520076/

- Kernel Recipes 2016 - Who needs a Real-Time Operating System (Not You!)
  [link](https://www.youtube.com/watch?v=4UY7hQjEW34)

- kernel recipes 2016 - Understanding a real-time system
  [link](https://www.youtube.com/watch?v=w3yT8zJe0Uw)

- Introduction to Real-Time Linux (ELC - 2017)
  [link](https://www.youtube.com/watch?v=BKkX9WASfpI&list=ULyJ0XuuCBu80&index=33)
  Note: much if not all is covered in the previous two talks; however, hackbench
  tool is mentioned and shortly explained, together with OSADL Latency Box

































