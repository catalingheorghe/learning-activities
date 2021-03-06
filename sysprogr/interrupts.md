## Interrupts

### General

Interrupts enable hardware to signal to the processor. The processor receives
the electrical signal and goes on to signal the operating system.

Hardware devices send signals to an interrupt controller, which multiplexes them
into a single line to to processor. Processor detects the signal, interrupts its
execution to handle it and can then notify the OS that an interrupt has
occurred.

Different devices associated with different interrupts by a unique value -
*interrupt requests (IRQ) lines*. 

Note: *exceptions* are synchronous to the processor clock, not async like
interrupts. They are generated by the processor, when executing software, for
example on a divide by zero. Called *software interrupts*. Handling is similar
inside the kernel, because many processors handle them similarly.

*Interrupt handler*, or service routine (ISR), is the function executed in
response to a specific interrupt. 

### Linux Kernel - general

**Interrupt handlers** are normal C functions, but they are executed in a
special context, *interrupt context*, which cannot block. Can run at any time.
Must respond quickly, at least to acknowledge the interrupt, but might also need
to do a lot of work. So, interrupt handling split into *top half* and *bottom
half*.

Top half is the interrupt handler. Performs only the work that is time-critical,
like ack-ing receipt of interrupt, resetting the hardware. Bottom half can run
in the future, with all interrupts enabled.

E.g.: network card - acknowlede the hardware, copy new packet into main memory,
prepares the network card for more packets.

Interrupt handlers must be registered by the drivers that service the device
which generates the interrupt - `request_irq()`. The interrupt number is either
fixed, or determined at probing, or dynamically.

*Interrupt handler flags* - `linux/interrupt.h`. 
 - `IRQF_DISABLED` - run ISR with all interrupts disabled (as opposed to only
   the current interrupt being disabled). Older flag: `SA_INTERRUPT`.
   This is now deprecated - as all ISR are or should be fast enough now, so
   default behaviour is to run ISR with interrupts disabled.
 - `IRQF_TIMER` - handles interrupts for the system timer
 - `IRQF_SHARED` - IRQ line shared between multiple handlers.

A text is also passed; will be shown to the user, what device is this interrupt
for.

A unique value (`dev`) is required for shared interrupts. Usually the device
structure pointer is passed here. This will reach the ISR when called.

Note: `request_irq()` can sleep, only because it creats a proc entry, which
calls `kmalloc()`.

Unregistering interrupt handlers is done with `free_irq()`. If there are no more
handlers active, the interrupt line is also disabled. Matching the handler is
done by the `dev` parameter.

Return value of an ISR is `IRQ_NONE` or `IRQ_HANDLED`. "NONE" is returned when
the ISR detects that the interrupt did not come from its device. It informs the
kernel that there are spurious interrupts. If there is only NONE on an IRQ line,
the kernel will detect the problem.

The role of an interrupt handler depends on the device. At the minimum, they
need to acknowledge to the device that they received the interrupt. Transfer
data and other extended work may be needed. Pushing as much as possible into the
bottom half handler is a good design.

Interrupt handlers in Linux *need not be reentrant* as the interrupt line is
masked on all processors during handling.

Note: spin locks can be used in ISRs, to protect shared data being accessed
concurrently on another core, from a bottom half or other kernel thread, for
example.

An interrupt handler is executed in *interrupt context*, which does not have a
process associated (`current` points to the interrupted process), so it cannot
sleep or block, since there would be no way to reschedule it. Stack where they
execute is one per processor - *interrupt stack* - usually one page size.
Note that processes each get their own kernel stack - 2 pages usually.

**Implementation** of interrupt handling is architecture and system dependent.
Basic idea is that when the processor receives a signal from the interrupt
controller, it interrupts the kernel, which calls `do_irq()`; if there is an
ISR registered on the IRQ line it is handled - `handle_IRQ_event()`; in any
case, `ret_from_intr()` is called to go back to interrupted code.

Note that `do_IRQ()` is called after the processor executes some assembly code
that saves the registers of the interrupted task, and the interrupt number; the
location of this code is setup in the interrupt vector; the processor jumps to
the location stored there when interrupted by a given IRQ line. This first step
is done with interrupts disabled. The `do_IRQ()` function will acknowledge the
receipt of the interrupt and will mask that line - for an example, see
`mask_and_ack_8529A()`. 

The `handle_IRQ_event()` will turn interrupts back on, if `IRQF_DISABLED` was
not specified. Note that newer kernels just keep them disabled - `IRQF_DISABLED`
was deprecated. Then each potential handler (if shared) is executed in turn.
Before returning to `do_IRQ()`, interrupts are disabled again (older kernels).
The `do_IRQ()` function cleans up and jumps to the exit code `ret_from_intr()`,
which is written in assembly. Here, the `need_resched` flag is checked - if set,
and if returning to user-space, `schedule()` is called; if returning to kernel
code the `preempt_count` is checked against zero to see if it safe to preempt.
After `schedule()` returns, if called, the original registers are restored, and
the kernel resumes whatever was interrupted.

On x86: assembler routines in `arch/x86/kernel/entry_64.S` and C methods in
`arch/x86/kernel/irq.c`.

**Procfs** file `/proc/interrupts` show statistics for interrupts (`fs/proc
show_interrupts()`).

**Interrupt control routines**, to disable the interrupt system on a machine, to
mask out an IRQ, are architecture dependent - `asm/system.h, asm/irq.h`. Note
that disabling interrupts, which disables preemption, does not guard against
concurrent access of code, which can happen in a SMP.

E.g.: `local_irq_disable() local_irq_enabled()` - disable/enable interrupts on
current CPU. On x86, two simple assembly instructions `cli`, `sti` (clear and
set to allow interrupts).

Note that starting with 2.5 there is no method to disable interrupts on all
cores of a SMP machine. Synchronization must be handled with local interrupt
control and with locks.

Masking out an interrupt line is disabling a specific interrupt for the entire
system. E.g.: `disable_irq()` disables a given IRQ in the interrupt controller,
stopping delivery to all processors.

Note: you do not want to enable an interrupt line from its handler - recall that
an IRQ is masked out while it is serviced.

Function to check where you are: `in_interrupt()` returns 0 if you are in
process context (you can block). `in_irq()` says if you are in an interrupt
handler (hardirq, top half; not bottom half or other).

### Linux Kernel - bottom halves and deferring work

*Top half* - async execution, immediate response to hardware. *Bottom half* -
scheduled execution, ampler actions to handle event.

Usual opperations in the top half (interrupt handler): acknowledge to the
hardware the receipt of the interrupt, copy data from or to the hardware. As
guideline: work that is time sensitive, work that relates to the hardware, work
that must not be interrupted by same interrupt.

Bottom half runs later, at any point, with all interrupts enabled. This improves
system latency.

There are different mechanisms to implement bottom halves.

The original bottom half (*BH*), which has been removed, was a list of 32 bottom
halves that could be marked to run later. Only one could run at a time, until
completion. Later on, *task queues* were introduced as a way to deferr work -
queues that contained functions to be called at a later time. Too inflexible and
not sufficiently lightweight. Note that in 2.5, BH was removed and task queues
were converted to work queues (simple way to deferr work in process context).

In 2.3 development, *softirqs* and *tasklets* were introduced. Softirqs are a
set of statically defined (at compile time) bottom halves that can run on any
processor and can interrupt each other. Tasklets, which have nothing to do with
tasks, are built on-top, can be registered at runtime and only one instance of a
given tasklet can run at a given time. Note: tasklets are enought for most
drivers, but for performance, like networking, softirqs are required.

So, in 2.6 there are the following mechanisms for bottom halves:

 - softirqs
 - tasklets
 - work queues
 - note: kernel timers (which deferr work until after a given time has passed,
   so don't really fit the general approach of at any time, just not now)

**Softirqs** - `kernel/softirq.c`. They are statically registered at compile
time in a vector - `softirq_vec`; currently there are nine softirqs. The kernel
enforces a limit of 32. The action of a softirq is in a `struct softirq_action`.

A softirq never preempts another softirq, but can be preempted by an interrupt
handler. Same softirq can run on different processors in parallel; implicitly,
different softirqs can run on different processors in parallel.

To be executed, a softirq is marked as pending. Usually this is done in an
interrupt handler. The pending status is checked: when returning from a hardware
interrupt, in `ksoftirqd` thread, in code like the networking subsystem. The
function that executes them is `do_softirq()`.

What uses softirqs now: network and block subsystems, kernel timers, tasklets,
RCU. Unlikely that you need to add a new one to the system.

E.g.: networking code registers two softirqs with `open_softirq(NET_TX_SOFTIRQ,
net_tx_action)` and similarly with rx. Raising a softirq is done with
`raise_softirq(NET_TX_SOFTIRQ`), which shortly disables interrupts locally while
marking the softirq as pending; if interrupts are already off,
`raise_softirq_irqoff()` brings a small optimization.

Softirqs are most often raised in interrupt handlers, which deal with the
hardware. After they exit, the kernel will call `do_softirq()` to continue
handling.

**Tasklets** are almost always the choice for a bottom half implementation. They
are implemented on top of softirqs, using `HI_SOFTIRQ` (top priority) and
`TASKLET_SOFTIRQ`. Represented by `struct tasklet_struct`.

List of scheduled tasklets is held per processor in `tasklet_vec` and
`tasklet_hi_vec`. After adding to one of these lists, the corresponding softirqs
are raised. This is done in `tasklet_[hi_]schedule()`. `do_softirq()` will
execute the handlers - `tasklet_[hi_]action()` - which represent tasklet
processing. These functions ensure that only one tasklet of a given type runs at
the same time.

Creating a tasklet can be done statically (`DECLARE_TASKLET[_DISABLED]()`
macros) or dynamically (`tasklet_init()`). Tasklet handlers cannot sleep,
similar to softirq handlers, since they can run in interrupt context (return
path of interrupt handler). Take care if the tasklet shares data with an
interrupt handler (disable interrupts, take a lock). Two of the same tasklets
can never run at the same time, but two different tasklets can run at the same
time on two different processors. Shared data used by other tasklets or softirqs
must be synchronized.

Scheduling a tasklet will make it run once at some time in the near future. If
scheduled twice (`tasklet_schedule()` called twice) before running, it will
still run only once. If scheduled again while running (for eg on another
processor), it is rescheduled. A tasklet always runs on the processor that
scheduled it, to take advantage of locality.

Tasklets can also be disabled, enabled and removed from the pending queue.

**ksoftirqd** - per-processor kernel threads that help with softirq processing
when the system is overwhelmed with them. Note that tasklets fall in the same
processing.

Softirqs are usually raised from interrupt handlers. For some kind of devices,
like network devices, this can happen very often. Also, softirqs can raise
themselves while executing, as the network code does. The combination of these
two factors can lead to a lot of softirq processing demand, which can lead to
user-space starving, but is important because softirq handling needs quick
handling.

The compromise between handling re-raised softirqs immediately, thus starving
userspace, and handling re-raised "excess" softirqs at the next interrupt event,
thus not taking advantage of an idle system, was to create a set of kernel
threads, with low priority (nice value 19), which are woken up when the number
of softirqs grows excessive. The kernel threads are woken up in `__do_softirq()`
after a number of iterations.

**Work queues** are different because they defer work into a kernel thread, so
this type of bottom half has a process context, are schedulable, can sleep.
Examples would be: you need to allocate a lot of memory, take a sempahore, or
perform block I/O.

The idea is to have *worker threads* that execute work queued from elsewhere.
In its most common form, it uses a default worker thread for handling the work.
They are `events/n`, one per processor. A driver can create its own worker
thread, but it must have a strong reason.

The worker threads are represented by `struct workqueue_struct`, which represent
a *type* of worker threads, which has one `struct cpu_workqueue_struct`, one
thread, per processor. A worker thread is a kernel thread running
`worker_thread()`, which when woken up, it goes through a list of `struct
work_struct` and executes them.

Using work queues first means creating some work to deferr - `DECLARE_WORK()` or
`INIT_WORK()`. The handler will run in process context, in a worker kernel
thread, but note that it can't access user-space memory, as there is no
associated user-space memory map (only when a kernel thread runs on behalf of a
user-space process is that mapped in). To schedule work with the default
*events* worker threads - `schedule_work()`; can also be scheduled after a
delay - `schedule_delayed_work()`. To wait for all work in a work queue to be
executed - `flush_scheduled_work()`; common when unloading a module, for
example.

A new work queue (set of worker threads) can be created with
`create_workqueue()`. Then `queue_work()` is used to schedule work in it.

**What bottom half** to use? You need to sleep, you need process context? Work
queue. Otherwise, tasklets. Scalability on multiple processors a concern? Look
into softirqs.

**Locking**. 

Tasklets are serialized with respect to themselves - same tasklet can't run
concurrently. But another tasklet can. Data shared between tasklets must be
protected.

Softirqs provide no serialization, all shared data must be syncrhonized. Note
that softirqs can't preempt another softirq, but any softirq can run on another
processor.

If process context code and a bottom half share data, you must disable bottom
half handling and use a lock.

If interrupt context code and a bottom half share data, you must disable
interrupts and use a lock.

**Bottom half disabling** - `local_bh_disable()`. Implemented using a per-task
counter, `preempt_count`, which is the same counter used by kernel preemption.
Note that `local_bh_enable()` also checks for any pending bottom halves and
executes them.

Note that this "bottom half" (basically softirq) disabling is used to guard
against a softirq preempting another softirq.

Also note that there is no mechanism or need to disable work queues, as they
are normal process context work and don't occur asynchronously.

## Resources

- Linux Kernel Development (3rd edition), Chapters 7 and 8 
- user context, hardirq, softirq
  https://www.kernel.org/doc/htmldocs/kernel-hacking/basic-players.html
  https://www.kernel.org/doc/html/v4.14/kernel-hacking/hacking.html
- `IRQF_DISABLED` removal https://lwn.net/Articles/380536/
  https://lwn.net/Articles/380931/
- procfs interrupts
  https://opensource.com/article/20/10/linux-kernel-interrupts

Detailed

- online gitbook linux internals
  [link](https://0xax.gitbooks.io/linux-insides/content/Interrupts/linux-interrupts-1.html)
- interrupt vector in Linux tree
  [link](https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/irq_vectors.h)
- college course notes and labs
  [link](https://linux-kernel-labs.github.io/refs/heads/master/lectures/interrupts.html#)


