Scheduler in Linux for the default `SCHED_OTHER` is CFS. Based on proportions of
a time window (or target latency) in which all processes shall run. Proportions
are calculated based on nice values.

Real-time scheduling is `SCHED_RR` and `SCHED_FIFO`. They have higher priorities
than `SCHED_OTHER` (1-99 vs 0) and use different scheduling algorithms. Note
that there is a knob in `proc` that allows the kernel to run for a limited
amount of time non-real-time processes (`sched_rt_runtime_us`).

Kernel schedules individual threads. In the kernel, processes or threads are the
same (with more or less resources shared), and are referred to as *tasks*. Data
structure for the *process descriptor* is `struct task_struct`. *Process
context* is when the kernel is executing for a user-space process, or when a
kernel thread is running (kernel threads are created by kernel; don't have
address space mapped - `mm` pointer NULL).

CFS keeps track of runtime of each process, taking into account the weight (nice
priority). It updates this periodically (system timer) or when a process becomes
runnable or blocks. Runnable entries are held in rbtree. The left-most entry is
the one with the smallest runtime, so when it needs to choose next task to run,
it chooses that one.

Sleeping is implemented via waitqueues. A task put itself on it, removes itself
from the rbtree and calls `schedule()`. Waking up of all entities on the
waitqueue is done by the code that delivers the even (data arriving, timer
interrupt etc).

To signal that a rescheduling is required, a flag `need_rescheduled` is used.
Set by `scheduler_tick()` when a process should be preempted (exhausted its
share; called in timer interrupt usually). Also set by the waking up mechanism
when a higher priority process is runnable. The flag is checked when returning
to user-space (interrupt or syscall) - user preemption. But the flag is also
checked when returning from an interrupt to kernel code, or when unlocking
synchronization objects - only if the code does not hold any lock is it safe to
be preempted. This is kernel preemption.

Interrupt handlers (ISRs) can be executed asynchronously by the processor, as
they are trigerred by an external event. They run with that interrupt (IRQ)
disabled on all processors, and with interrupts disabled on that processor
(initially this was an option, but was switched to being the default). An ISR
runs in *interrupt context*, which does not have a process associated, so it
can't sleep or block (it can't be rescheduled). Note: ISRs usually use  a stack
that is preset, one per processor.

As mentioned, on the way back from handling an interrupt (`ret_from_intr()`),
the kernel checks if the scheduler needs to be called.

Disabling the entire interrupt system disables also any preemption, since the
timer interrupt is also disabled. This should be a last resort. Note that there
is no method to disable interrupts on all cores of a SMP machine.

Masking or disabling a single IRQ line affects all cores. Interrupt handlers run
with that IRQ masked, so they don't need to be re-entrant.

The async response to a hardware event is the *top half* of interrupt handling.
Work can be deferred to a schedulable context through *bottom half* mechanisms.
They run with interrupts enabled, so can be preempted by an interrupt handler.
Softirqs are statically defined at compile time; they don't preempt other
softirqs, but can run in parallel on different cores. Tasklets are implemented
on top of softirqs; they have the extra simplification that the same action
can't be executed in parallel not even on different cores.

If there are is an overload of softirqs (either from interrupt handlers and/or
from softirq handlers re-raising themselves), the softirq mechanism wakes up a
set of kernel threads to handle them: one per processor `ksoftirqd/n`. These are
just woken up, made runnable, and the scheduler will decide when they can run.

The third type of bottom half is a work queue, which defers work to a kernel
thread, by default `events/n`, which will be under the scheduler's control.



