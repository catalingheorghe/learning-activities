# 21st Century C

## 2. Debug, Test, Document

### Debugger

**Always** use a debugger. Linux Torvalds considers the debugger like
dissasembler on steroids. Why not be able to increase logging verbosity on the
fly (`print verbose++`), or break out of a loop (`print i=100`) or similar?

Compilation options

  * debug symbols - `-g` (mandatory); downside: may increase size slightly
    * default level is 2 (`-g2`); level 3 includes macro information as well
    * `-ggdb` - ensures gdb specific debug info, not native os (so, even better)
  * optimizations - `-O0` (optional)

Debugging commands

  * printing expression (variable, function calls): `p var`
  * printing an array: `p *data@10`
  * see all local variables: `info local`
  * see arguments to the function: `info args`
  * see registers: `info r`
  * display variable on every step: `disp avg`
     * turn off autoprinting: `undisp 1`
  
  * call stack: `bt`
  * go to a frame: `f 1`
    * i,e,: to print data (variables) as they were in that frame, to see nothing
      got corrupted
  * go up and down the stack frames: `down`, `up`
  
  * see list of threads: `info threads`
    * debugger is in the one with `*`
  * move to another thread: `thread 2`

  * set breakpoint: `b function`, `b line`, `b file:line`
  * see breakpoints: `info break`
  * disable/enable: `dis 1`, `en 1`
  * delete breakpoint: `del 1`
  * break if the value of the given variable changes: `watch count`

  * next line, don't enter subfunctions: `n`
  * step online, enter subfunctions: `s`
  * continue unitl breakpoint: `c`
  * start over: `r`
    * run with args: `r args...`
  * until next line forward from current line (beyond current loop): `u`
  * return from current function: `ret`, `ret 3`

GDB init options

  * `set print thread-events off` - turn off thread spawning messages
