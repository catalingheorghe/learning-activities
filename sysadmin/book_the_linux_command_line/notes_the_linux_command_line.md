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

Why use the CLI for things like this? Because doing complicated things is not that easy in the GUI. Example: copy all html files from a directory to another, but only files that do not exist or are newer than the versions in the desitnation directory: `cp -u *.html destination`.

The shell makes this possible by using **wildcards**, special characters that allow specifying group of filenames. Also known as *globbing*.

 - `*` - any characters, `?` - single character
 - `[characters]` - any character that is a member of the set, `[!characters]` - negate
 - `[[:class:]]` - class of characters
    - class: alnum, alpha, digit, lower, upper

Ex: `*[[:lower]123]` - any file ending with a lower case letter or 1, 2, 3 numerals

Note: traditional Unix style character ranges like `[A-Z]` or `[a-z]` can still work, but recommendation is to use the character classes.

Note: wildcards also work in some GUI file managers, like the Nautilus file manager.

**<cmd: mkdir; cp; mv; rm; ln>**

 - `mkdir dir1` - create directory, or multiple directories (`mkdir dir1 dir2 ...`)
    - `mkdir -p dirpath` - create directory, including parent directories
 - `cp item1 item2` - copy file or directory item1 to the file or directory item2
 - `cp item... dir` - copy multiple files or directories into dir
    - `-a / --archive` - maintain all attributes, including ownership, permissions; usually, the copies take the default attributes of the user
    - `-i / --interactive` - prompt when overwriting; default, NO
    - `-r / --recursive` - recursively copy directories and their contents
        - `cp -r dir1 dir2` - if dir2 does not exist, it will be created and the contents from dir1 copied there; if it does exist, the directory dir1 and its contents will be copied there
    - `-u / --update` - when copying files from one directory to another, only update the destination directory (newer files or non-existent files)
    - `-v / --verbose`
 - `mv item1 item2` - move or rename item1 to item2
    - `-i, -u, -v` - like for `cp`
 - `rm item...` - remove files or directories
    - `-i` - prompt before deleting a file
    - `-r, -v`
    - `-f / --force` - ignore nonexistent files and do not prompt
 - `ln file link` -  create a hard link
 - `ln -s item link` - create a symbolic link

Note: Try to use `cp -i` as often as possible. Avoid accidentally replacing a file.

Note: when using `rm` with wildcards, test if first with `ls`!

*Hard links* are the original links of the Unix world. They represent an additional directory entry for a file. By default every file has a single hard link. A hard link can only reference a file in the same file system and it can not reference a directory. A hard link is exactly like the file its-self. When it is deleted, link is deleted, but the file contents remains until there are no more hard links. Hard links reference the same inode, the same data blocks from the disk. To display inode information use `ls -li`.

*Symbolic links* overcome both limitations of hard links. They create a special file type that contains a text pointer to the referenced file or directory. They are indistinguishable, but if the file is deleted the link remains, broken.

Keep in mind that a symbolic link is an actual text reference containing a path to the target item. It can be an absolute path or a relative path, relative to the link's location. Relative paths is more desirable - it allows moving or renaming a directory tree.

Most operations on symbolic links are done on the target file; `rm` is an exception, it will remove the link.

NOTE: in Nautilus, CTRL+SHIFT when dragging a file creates a link.

### Working with commands

A command can be

 - executable program (compiled C, python script etc)
 - a command built into the shell (bash has *shell builtins*, like `cd`)
 - a shell function (like a small shell script, incorporated into the environment)
 - an alias (commands that we can define, formed out of other commands)

**<cmd: type; which>**

 - `type cmd` - identifies the type of command. type is shell builtin
 - `which executable` - gives the location of the command of type executable program

**<cmd: help, man, apropos, whatis, info, zless>**

Now that we know the type, we can get the **documentation** of the command.

 - `help cmd` - if cmd is a shell builtin, the shell has a help utility
 - `cmd --help` - many executables support this long option
 - `man cmd` - most executable programs support a manual entry, or *man page*, a formal piece of documentation
    - `man N item` - N is number of section ex: man 5 passwd
 - `apropos item` - search list of all man pages for possible matches; `man -k` does the same thing
 - `whatis item`  - name and one line description matching a man page; `man -f` does the same thing
 - `info item` - show Info documentation, from *info files*; introduced and used by tools from GNU Project

Manual pages are displayed by a pager. Usually, on Linux distributions, less is used. The manual is broken into different sections (1 user commands, 2 syscalls, 3 C library calls, 4 special files, 5 file formats, 6 games and amusement 7 misc, 8 system administration commands).

For example, to get info about /etc/passwd file format, not passwd command, use `man 5 passwd`.

The GNU Project offers an alternative to man, called info. It is a hyperlinked collection of pages, or nodes, displayed with the program `info`. Most of the basic commands are part of the GNU *coreutils* package, so you can start with `info coreutils`.

Many programs come with documentation in `/usr/share/doc`. Either as text, or html or gzipped text files. For zip, use `zless` to unpack and page them.

**<cmd: alias; unalias>**

New commands can be created with `alias`. To check if the new command name that you want to use is free, use `type`. However, aliases can be used to overwrite a command, like it is usually done with `ls`. Note that defining an alias on the command line will only persist for the current terminal session.

 - `alias name='string'` - no whitespaces between name, = and string;
 - `unalias cmd` - remove the alias
    - eg: `alias foo='cd /usr; ls; cd -'` ; `unalias foo`
 - `alias` - show all aliases configured in the environment

Resources

 - [bash reference manual](https://www.gnu.org/software/bash/manual/bashref.html)
 - [bash faq](http://mywiki.wooledge.org/BashFAQ)
 - [GNU programs documentation](https://www.gnu.org/manual/manual.html)

### Redirection

A Linux program gets its input for a file, *stdin*, sends the output data to a *stdout*, and the error message to *stderr*. All these can be redirected from the default keyboard and screen.

 - `ls -l /usr/bin > ls-output.txt` - output redirection
 - `ls -l /usr/bin >> ls-output.txt` - output redirection, but appending, not truncating

Note that the output redirection always truncates the file first. So, if we want to empty a file, or create a new one, we can use the trick `[prompt] $ > new-file.txt`.

 - `ls -l /bin/usr 2> ls-error.txt` - redirect stderr, which is file descriptor 2 (stdout = 1, stdin = 0)
 - `ls -l /usr/bin > ls-output.txt 2>&1` - redirect stdout and stderr to same file, traditional way; the order matters, as the redirections are done in turn - first stdout to a file, then stderr to the file descriptor of stdout
    - `ls -l /usr/bin &> ls-output.txt` - recent versions of bash
    - `&>>` also exists
 - `ls -l /bin/usr 2> /dev/null` - suppress errors, by sending them to the *bit bucket* device

Standard input can also be redirected. But first, cat.

**<cmd: cat>**

 - `cat file...` - copy the contents of one or more file to stdout
 - `cat > newfile.txt` - create newfile with input from keyboard
 - `cat < newfile.txt` - redirect stdinput to a file and copy it to stdout; not very useful in this format, but opens the way for using pipes

The `cat` command can be used to join files together, con*cat*enate them. Any type of files, not only text files; like parts of a big archive for example. `cat archive.* > archive`

With no arguments, `cat` expects input from standard input. It copies the input line by line to stdout, until EOF is received (Ctrl-d from the keyboard).

Note: GNU programs usually accepts multiple files as arguments; one of those arguments can be `-`, which signifies standard input.

**<cmd: sort; uniq; wc; grep; tee>**

Standard input and standard output are put to good use by the shell **pipelines**, using the pipe operator (|). The stdout of one command is piped into the stdinput of another. More commands chained together that perform complex operations on data, are usually referred to as `filters`.
`
 - `ls -l /usr/bin | less`
 - `ls -l /bin /usr/bin | sort | less`

Some other common "filter" commands are

 - `uniq` - take a sorted list of data either from stdin or from a file and removes the duplicates
    - `uniq -d` - display the duplicates
 - `wc file...` - print line, word and byte counts; accepts multiple files
    - `wc -l` - line count
 - `grep pattern textfile...` - pattern matching in text files or standard input
    - `-i` - case insensitive
    - `-v` - display lines that do not match
    - eg: `ls -l /bin /usr/bin | sort | uniq | grep zip`
 - `head / tail file...` - print first or last 10 lines
    - `-n` - number of lines to print
    - `tail -f file` - watch the file in realtime
 - `tee` - read from stdin and copy to stdout and to files; like a T in plumbing, for replicating the input both down the pipe and to each file given as argument
    - eg: `ls -l /usr/bin | tee ls.txt | grep zip`

### Expansion, quoting

When pressing enter, bash performs several substitutions to our text before it carries out the command (wildcards, for example). This is called **expansion**.

**<cmd: echo;>**

 - `echo arg...` - arguments are displayed on stdout
 - `echo *` - will not print `*`, but the expanded list of files and directories in the current directory; echo never saw the wildcard, the shell gave it the expanded result

Wildcards work by *pathname expansion*. What the shell expands to and then matches wildcards are pathnames.

 - `echo /usr/*/share`
 - `ls -ld .[!.]*` - list hidden files or directories (avoid `.` and `..`); will not work for files starting with multiple periods - better choice is `ls -A`

*Tilde expansion* refers to home directories.

 - `echo ~` , `echo ~user~` - path of home directory of current user, or of named user

*Arithmetic expansion* - `$((expression))` - can also be used. It works for only for integers but has support for the usual operators, inluding `%` and `**` - exponentiation.

 - `echo $(((5 ** 2) * 3))` - 75

*Brace expansion* allows creating multiple text strings from a pattern containing braces. The expansion can contain a comma separate list, or a range of integers or single characters.

 - `echo front-{a,b,c}-back`
 - `echo number-{1..7}`; zero padding - `{01..15}` or `{001..15}`; `{Z..A}`
 - `echo a{A{1,2},B{3,4}}b` - aA1b aA2b aB3b aB4b
 - `mkdir {2007..2019}-{01..12}`

The most common application is to make list of files or directories to be created.

*Parameter expansion* is more useful in shell scripts than used directly, but still: expansion of the contents of variables. If there is no variable with given name, it will expand to an empty string.

 - `echo $USER` - name of current user

(to see all environment variables: `printenv | less`)

*Command substitution* allows to use the output of a command as an expansion - `$(cmd)`

 - `echo $(ls)` - output of ls
 - `ls -l $(which cp)`
 - `file $(ls -d /usr/bin | grep zip)`

(in older shells, backquotes ``cmd`` where used instead of dollar form; bash still supports this)

**Quoting** is used to control expansion.

The following examples do not produce the expected result.

 - `echo this is a    test` - *word-splitting* by the shell removes extra whitespaces
 - `echo total is $100.00`  - the variable $1 will be expanded to the empty string

Quoting can selectively suppress expansion. It does this by using double quotes or single quotes.

 - double quotes - all special shell characters loose their meaning
    - exceptions: `$, \, `` - this means that parameter expansion, arithmetic expansion, command substitution are still done
    - `ls -l "bad filename.txt"`
 - single quotes - all expansions are suppressed
    - `echo '$USER command $(cmd) $((2+2))'` - will print exactly that

Note: for command substitution, word-splitting has a subtle effect. Word-splitting works by considering whitespace, tabs, line feeds as delimiters between arguments. By double quoting them, they become part of the argument. As an example, `echo $(cal)` does not have the expected print formatting for `echo "$(cal)"`.

To quote only a single character, we can precede it with a backslash `\` - *escape character*.

The backslash is also used to represent control codes (`\t, \n, \r, \b` - backspace). To be interpreted `-e` must be passed to `echo` and used under double quotes.

### Keyboard tricks

The library used by bash to implement command line reading is *Readline*. 

Cursor movement

 - Ctrl-a - beginning of line
 - Ctrl-e - end of line
 - Ctrl-f / Ctrl-b - forward / back one character
 - Alf-f - forward one word
 - Alt-b - back one word
 - Ctrl-l - clear screen and move to top left corner (like `clear`)

Modifying text

 - Ctrl-d - delete char at cursor position
 - Ctrl-t - transpose char at cursor location with the one preceding it
 - Alt-t - transpose word at cursor location with the one preceding it
 - Alt-l / Alt-u - chars to lowercase / uppercase from cursor location to end of word

Cutting and pasting

In Readline terminology, killing and yanking. Items that are cut, or killed, are stored in a temporary buffer called the kill-ring.

 - Ctrl-k - kill text from current location to end of line
 - Ctrl-u - kill text from current location to beginning of line
 - Alt-d - kill text from current location to end of word
 - Alt-Backspace - kill text from current location to the beginning of word, or prev word if at the start of a word
 - Ctrl-y - yank text for kill-ring and insert it at the cursor location

Note: the META key mentioned in the READLINE documentation in the bash man page maps to Alt on modern computers. But in the age of terminals connected to mainframe computers, could be different. Linux still supports the META key by pressing and releasing ESC, it is the same like ALT pressed in the terminal.

Completion

Pressing TAB will tell the shell to complete the current word, it it has a unique match. Usually you use it in path completion, but it also works in variable names ($), user names (~) and hostnames (@) for host in `/etc/hosts`.

 - Alt-? or double Tab - list of possible completions
 - Alt-\* - insert all possible completions (does not work in terminal emulator)

Recent bash versions allows for programmable completion. For example: options for a particular command, or supported file types. They are implemented by shell functions. 

**History**

Bash keeps the command history in the file `.bash_history` in your home directory. It can be viewed using `history | less`. An *history expansion* can be used.

**<cmd: history;>**

 - `history | grep /usr/bin` - search history
 - `!88` - execute command 88 from `history` listing
 - `!!` - repeat the last command (like up + enter)
 - `!string` - repeat last history list item starting with string
 - `!?string` - repeat last history list item containing string

(the last two don't give any indication of what will be executed beforehand, so you may have surprises)

Using `Ctrl-r` you can do a reverse incremental search. We can execute the found command by `Enter` or copy it to the command line by `Ctrl-j`.

 - Ctrl-p - move to the prev item in history list, like up arrow
 - Ctrl-n - move to the next, like down arrow
 - Alt-< - move to the top (oldest)
 - Alt-> - move to the bottome
 - Ctrl-r - incremental reverse search
 - Alt-p - non incremental reverse search
 - Alt-n - non incremental forward search
 - Ctr-o - execute current history item and move to next, then cycle - useful with Ctrl-o and then to execute a series of commands

**<cmd: script;>**

 - `script [file]` - can record a shell session to a file; it includes everything on the terminal. See the man for options and features, including a cool example with a named pipe, `mkfifo + script + cat`.


