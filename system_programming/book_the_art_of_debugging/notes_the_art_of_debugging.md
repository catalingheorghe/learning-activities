# The art of debugging with GDB and DDD

## Table of Contents

 0. About

## 0. About

Amazon books [link](https://www.amazon.com/Art-Debugging-GDB-DDD-Eclipse/dp/1593271743/ref=sr_1_1?s=books&ie=UTF8&qid=1544428669&sr=1-1&keywords=the+art+of+debugging).

## 1. Preliminaries

Tools:

 - GDB
 - DDD
 - Eclipse (3.3)

*Principle of confirmation*: debugging is confirming one by one that the things you believe to be true are actually true. If you find that one of those assumption is not correct, you have found a clue tot the location and nature of the bug.

Classic debugging technique: tracing (printfs, log messages etc).

GDB - text based. CTRL-P CTRL-N can scroll through commands (as can up down arrows). Has a "-tui" option that lets you see the code as well.

DDD and Eclipse - gui frontends. Eclipse - full ide with outline, compilation etc.

Another TUI option: CGDB as a front-end for GDB.




