# The Linux Command Line

William Shots

## Table of Contents

0. About

## 0. About

Website (link)[http://linuxcommand.org/tlcl.php] - Creative Commons license, free for download

## 1. Shell notes

### Basic

Default shell offered by most Linux distributions: GNU Project called `bash` - Bourne Again SHell - `sh` was the original Unix shell written by Steve Bourne.

A **terminal emulator** allows you to interact with the shell when using a GUI (ex: gnome-terminal, konsole).

Prompt usually contains *user@machinename cwd*. If the prompt has the last sign a `#`, the session has superuser priviledges.

Some simple commands

 - `date`
 - `cal`
 - `df`
 - `free`

To **end a terminal session**: `exit`, or close the window or `Ctrl-d`.

*Even if there are no terminal emulators running, there are 6 terminal sessions that are always running behind the GUI. They are called "virtual terminals" or "virtual consoles". To access them: `Alt-F1 - Alt-F6` with `Alt-F7` to return to graphical desktop.*

### Navigation and file system tree layout

Hierarchical directory structure = file system tree for directories and files. *Root* directory is the first direcotry in the tree. Linux has a single tree for all devices; storage devices are mounted at various points in the tree.

**<cmd: pwd; cd>**

 - `pwd` - display current working directory (where we are now in the tree)
 - `cd` - change working directory; specify an absolute or relative pathname
    - `cd` -> home directory
    - `cd -` -> previous working directory
    - `cd ~user_name` -> user's home directory

Filenames: "." as a first character denotes a hidden file or directory; avoid spaces; extensions doesn't count, but many applications programs use them.

**<cmd: ls>**

 - `ls` - list content of directory or multiple directories (ex: `ls ~ /usr/bin`)
    - `ls -l` - more details (long format)
    - `ls -lt` - long format, sorted by modification time (multiple short options strung together)
    - `ls -a` - list all, hidden as well; `ls -A` - list almost all (except `.` and `..`)
    - `ls -ld /usr/bin` - see details about a directory, not list the directory
    - `ls -lFSrh` - list items, classify them with an indicator character, sort by size, reverse order, human readable size

The date in the long format is the date and time of the file's last modification.

**<cmd: file; less; reset>**

 - `file filepath` - determine a filetype. Not determined by extension.
 - `less filepath` - scrollable display of a text file
    - Page Up or b; Page Down or space - page back, page forward (b is right above space on the keyboard)
    - up down arrow - line scrolling
    - G - go to end of file
    - g or 1G - go to beginning of file
    - /charactes - search forward to characters
    - n; N - search forward, search backwards
    - h - help screen
    - q - exit less

ASCII "As-Key" is one of the first and simplests represantation of data (mapping data to numbers; computers work with numbers). Text is a simple one to one mapping. 

The *less* name comes from `less is more` - it is an improvement of an earlier Unix program called *more*. less falls into the class of *pagers*.

If your terminal is scrambled, for example by displaying a binary file, use `reset` to reinitialize the terminal.

**File system layout**

The design of the Unix file system tree layout is published in *Linus Filesystem Hierarchy Standard*. Linux follows this, but it is not an enforced, it varies from distribution to distribution.

 - `/`
 - `/bin` - binaries needed for system to boot and run
 - `/boot` - linux kernel, initial ram disk, bootloader (usually grub)
 - `/dev` - device nodes
 - `/etc` - configration files plus init scripts; it should be all text in here
 - `/home` - place for users' home directory
 - `/lib` - libraries used by system programs
 - `/lost+found` - used by linux file systems (like ext4) for partial recovery; if not empty, something bad happened
 - `/media` - place where removable media devices are usually mounted automatically
 - `/mnt` - usually for manual mounts
 - `/opt` - usually place to install optional and/or commerical software
 - `/proc` - virtual filesystem as an API to the kernel
 - `/root` - home of root directory
 - `/sbin` - "system" binaries; generaly vital things, reserved to the superuser
 - `/tmp` - storage of temporary files
 - `/usr` - largest directory tree; usually programs and support files used by regular users
 - `/usr/bin` - executable programs installed by the Linux distribution - thousands of them
 - `/usr/local` - programs not included with the distribution but are intended for system-wide use; programs compiled from source usually are installed in `/usr/local/bin`. On a newly installed Linux system, this should be empty.
 - `/usr/sbin` - more system administration programs
 - `/usr/share` - shared data used by programs in `/usr/bin` (default config files, icons, sound files etc)
 - `/usr/share/doc` - docmentation from packages installed on the system
 - `/var` - place to store data that is likely to change (database files, user email, spool files etc)
 - `/var/log` - log files (ex: /var/log/messages or /var/log/syslog)

Resources - file system layout

 - http://www.pathname.com/fhs/
 - https://wiki.debian.org/FilesystemHierarchyStandard
 - [wikipedia](https://en.wikipedia.org/wiki/Unix_filesystem#Conventional_directory_layout)

### Manipulating files and directories



