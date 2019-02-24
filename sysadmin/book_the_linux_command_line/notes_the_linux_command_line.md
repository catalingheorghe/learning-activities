# The Linux Command Line

William Shots

## Table of Contents

0. About
1. Shell notes and Linux intro
2. Configuration and the environment
3. Common tasks and essential tools

## 0. About

Website [link](http://linuxcommand.org/tlcl.php) - Creative Commons license, free for download

*Version: 19.01 - Fifth Internet Edition*

## 1. Shell notes and Linux intro

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

**`<cmd: pwd; cd>`**

 - `pwd` - display current working directory (where we are now in the tree)
 - `cd` - change working directory; specify an absolute or relative pathname
    - `cd` -> home directory
    - `cd -` -> previous working directory
    - `cd ~user_name` -> user's home directory

Filenames: "." as a first character denotes a hidden file or directory; avoid spaces; extensions doesn't count, but many applications programs use them.

**`<cmd: ls>`**

 - `ls` - list content of directory or multiple directories (ex: `ls ~ /usr/bin`)
    - `ls -l` - more details (long format)
    - `ls -lt` - long format, sorted by modification time (multiple short options strung together)
    - `ls -a` - list all, hidden as well; `ls -A` - list almost all (except `.` and `..`)
    - `ls -ld /usr/bin` - see details about a directory, not list the directory
    - `ls -lFSrh` - list items, classify them with an indicator character, sort by size, reverse order, human readable size

The date in the long format is the date and time of the file's last modification.

**`<cmd: file; less; reset>`**

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

The design of the Unix file system tree layout is published in *Linus Filesystem Hierarchy Standard*. Linux follows this, but it is not enforced, it varies from distribution to distribution.

 - `/`
 - `/bin` - binaries needed for system to boot and run
 - `/boot` - linux kernel, initial ram disk, bootloader (usually grub)
 - `/dev` - device nodes
 - `/etc` - configuration files plus init scripts; it should be all text in here
 - `/home` - place for users' home directory
 - `/lib` - libraries used by system programs
 - `/lost+found` - used by linux file systems (like ext4) for partial recovery; if not empty, something bad happened
 - `/media` - place where removable media devices are usually mounted automatically
 - `/mnt` - usually for manual mounts
 - `/opt` - usually place to install optional and/or commerical software
 - `/proc` - virtual filesystem as an API to the kernel
 - `/root` - home of root directory
 - `/sbin` - "system" binaries; generaly vital things, reserved for the superuser
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

**`<cmd: mkdir; cp; mv; rm; ln>`**

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

**`<cmd: type; which>`**

 - `type cmd` - identifies the type of command. type is shell builtin
 - `which executable` - gives the location of the command of type executable program

**`<cmd: help, man, apropos, whatis, info, zless>`**

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

**`<cmd: alias; unalias>`**

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

**`<cmd: cat>`**

 - `cat file...` - copy the contents of one or more file to stdout
 - `cat > newfile.txt` - create newfile with input from keyboard
 - `cat < newfile.txt` - redirect stdinput to a file and copy it to stdout; not very useful in this format, but opens the way for using pipes

The `cat` command can be used to join files together, con*cat*enate them. Any type of files, not only text files; like parts of a big archive for example. `cat archive.* > archive`

With no arguments, `cat` expects input from standard input. It copies the input line by line to stdout, until EOF is received (Ctrl-d from the keyboard).

Note: GNU programs usually accepts multiple files as arguments; one of those arguments can be `-`, which signifies standard input.

**`<cmd: sort; uniq; wc; grep; tee>`**

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

**`<cmd: echo;>`**

 - `echo arg...` - arguments are displayed on stdout
 - `echo *` - will not print `*`, but the expanded list of files and directories in the current directory; echo never saw the wildcard, the shell gave it the expanded result

Wildcards work by *pathname expansion*. What the shell expands to and then matches wildcards are pathnames.

 - `echo /usr/*/share`
 - `ls -ld .[!.]*` - list hidden files or directories (avoid `.` and `..`); will not work for files starting with multiple periods - better choice is `ls -A`

*Tilde expansion* refers to home directories.

 - `echo ~` , `echo ~user` - path of home directory of current user, or of named user

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

 - `echo this is a  [spaces]  test` - *word-splitting* by the shell removes extra whitespaces
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

**`<cmd: history;>`**

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
 - Ctrl-o - execute current history item and move to next, then cycle

**`<cmd: script;>`**

 - `script [file]` - can record a shell session to a file; it includes everything on the terminal. See the man for options and features, including a cool example with a named pipe, `mkfifo + script + cat`.

### Permisssions

**Users** in Unix are split up into owners, group members and the world.

Each user has a number, *user id - uid*, which is mapped to a human readable name. Also, each user has assigned a *primary group ID - gid* and may belong to several groups.

Different distributions manage things differently regarding permissions. Fedora start with regular user ids at 500, while Ubuntu at 1000. In ubuntu, a user is assigned to more groups in order to manage permissions for different services.

The files used to manage this are: `/etc/passwd` - user (login) name, uid, gid, account name, home dir, login shell; `/etc/groups`; `/etc/shadow` - holds the passwords for users.

Traditional Unix style systems assign regular user to a *users* group, while modern Linux distribution create a unique, single member group with the same name as the user. Makes permission management easier.

**`<cmd: id;>`**

 - `id [username]` - display information about a user, or about current user (name, uid, gid, groups)

Access to a file is defined in the terms of **read, write and execute**. These permissions are stored in *file attributes*, which are shown first in the `ls -l` output.

First character denotes the file type

 - `-` - regular file
 - `l` - symlink (for symlinks the rest is rwxrwxrwx, dummy; real attributes are those of the target file)
 - `c` - character device file
 - `b` - block device file
 - `s` - local socket file
 - `p` - name pipe
 - `d` - directory
 
The remaining nine characters, the *file mode*, are the read write execute permissions for owner, file's group and everybody else. Read, write, execute have different meanings for files and directories.

 - read
    - file: can be opened and read
    - directory: content can be listed if the execute attribute is also set
 - write
    - file: can be written or truncated (but the ability to delete or rename files is controlled by directory permissions)
    - directory: files within can be created, deleted, renamed if the execute bit is also set
 - execute
    - file: can be treated as a program and executed
    - directory: allows a directory to be entered

**`<cmd: chmod; umask;>`**

In order to change the file mode, the command to use is `chmod`. Only the file owner or the superuser can do this. It can be done via octal representation (one octal digits - 3 bits - rwx), or via symbolic representation (who - u user owner, g group owner, o other word, a all; operation - `+`, `-` or `=`; what - r, w, x).

 - `chmod 600 ls-output.txt` - rw only for owner
 - `chmod u+x,go=rx` - add x to owner, group and other are set to rx

Note that the `--recursive` option for chmod acts on both files and directories, so it may not be the best thing to use, as you don't usually want files and directories to have the same permissions.

Default permissions given to a file are controlled by `umask`. It manipulates a mask of bits, in octal notation, that are removed when creating a file. The default umask is usually `0002`, which means it will remove the w permission from others. Note that the executable permission is not given by default.

 - `umask 0022` - change the umask so that w is removed from group and others; persistent in current session

The fact that a permission mask is actually a 4 digit expression is because there are other special permission settings besides read, write, execute.

The first of these is *setuid* (4000), which has the effect that when executing a program the *effective user id* is changed from the user running the program to the user that is the owner of the file. This means that for a *setuid root* program, normal users can execute it and benefit from superuser privileges. 

The second, less used setting, is *setgid* (2000), which changes the effective group id. When set on a directory, files created will be given the group ownership of the directory, not the primary group of the user creating the file - useful in a shared directory where members of a common group need access to all files and directories, no matter their primary groups.

The third is called the *sticky bit* (1000) - holdover from traditional Unix *not swappable*. On files, Linux ignores it. On a directory, it prevents user from deleting or renaming files unless the user is the owner of the directory, the owner of the file or the superuser. This is often used for directories like `/tmp`.

 - `u+s / g+s` - set setuid / setgid bit; in `ls` output there is an `s` for executable
 - `chmod +t dir` - set sticky bit; `t` for other - executable

There are several ways to **change identities**, aka assume other users' identities: logout and log back in, use `su`, use `sudo`.

**`<cmd: su; sudo;>`**

The `su` command start a shell as another user. With `-l`, a *login shell* is started, which loads the user's environment and changes to his home directory - this is usually what we want. If no username is specified, superuser is assumed.

 - `su -l username` - start a login shell as user usernam
 - `su -` - start a login shell as root (-l can be shortened to -)
 - `su` - start a shell as root
 - `su -c 'command'` - execute a single command in a new root shell; quoting is important to avoid expansion in our shell!

The prompt indicates superuser privileges by `#`.

The `sudo` command is designed for executing a command as another user. It can be configured by the administrator to allow a user to execute commands as a different user (usually root). A user may be restricted to one or more commands. 

`sudo` does not require access to the superuser's password, it requires the user's own password - the permission has already been configured by the administrator (`/etc/sudoers`), so no need to know the superuser password.

Also, `sudo` does not start a new shell, so commands do not need quoting. This can be changed by various options.

 - `sudo command`
 - `sudo -i` - start an interactive root shell, like `su -`
 - `sudo -l` - list what commands can current user execute

Note: on Ubuntu, the root account is by defaul disabled (it has no password set), so you cannot use `su`. The reason was that users tended to do everything as root. Ubuntu grants the initial user account superuser privileges via `sudo`.

The owner and group owner of a file can be changed, with superuser privileges, using `chown`.

**`<cmd: chown; chgrp;>`**

 - `chown [owner][:[group]] file...`
 - `bob` - change file owner to bob
 - `bob:users` - file owner: user bob, file group owner: group users
 - `:admins` - change file group owner to admin
 - `bob:` - file owner to bob and file group owner to bob's login group

In older versions of Unix, `chown` only changed file ownership, so another command was required for changing file group ownership: `chgrp`. It works in a similar way, but is more limited.

----

Example: setting up a shared folder for music between multiple users on the system

 - create group *music*
 - add users to group
 - create directory in /usr/local/share/music (with sudo)
 - `chown :music /usr/local/share/music` - change group owner of directory
 - set *setgid* bit on the directory so that files created by users get the group of the directory
 - make sure umask of users is ok so that files and directories created by them have the proper permissions - accessible by group owner (especially for directories - permission to write into directories owned by other users, but with same group owner)
    - set umask of users to 0002 (we will see how to make this persistent)

----

To **change your password**, use the `passwd` command. It can change your own password, or the one for other users, if you have superuser privileges.

**`<cmd: passwd;>`**

 - `passwd [user]` - will be prompted for old and new password

The superuser can do other things with passwd: account locking, password expiration and so on (see man page).

To **maintain users and groups** the commands to look into are: `adduser`, `useradd`, `usermod`, `groupadd`.

### Processes

At system startup the kernel initiates a few of its own activities as processes and start *init*, PID 1, that reads *init script* from /etc and initializes various services as *daemon programs*. The kernel keeps tracks of the processes - PIDs, user id, memory, state etc.

The most common way to **view processes** is using the `ps` command. By default it only shows the processes associated with the current terminal session. "TTY" is short for *teletype* and shows the *controlling terminal* of the process. "Time" shows the CPU time used by the process.

To see all the processes that we own, no matter the controlling terminal, add "x" as an option to ps. A "?" for TTY means that the process has no controlling terminal. The column STAT shown the state of the process.

STAT column

 - R - running or ready to run
 - S - sleeping; waiting for an event
 - D - uninterruptible sleep. Waiting for I/O such as a disk drive
 - T - stopped
 - Z - defunct, zombie process; not cleaned up by its parent
 - < - high priority process; related to *niceness* - not as nice
 - N - a *nice* process, marked as lower priority

Another common format of ps options is `ps aux`, which shows even more information. It displays processes belonging to every user. Without leading dash for options, Linux ps emulates the BSD version style. Additional information is

 - USER - owner of the process
 - %CPU - CPU usage in percentage
 - %MEM - memory usage in percentage
 - VSZ - virtual memory size
 - RSS - resident set size - amount of physical memory (RAM) used, in kilobytes
 - START - time when process started

**`<cmd: ps; top;>`**

 - `ps` - processes from current terminal session
 - `ps x` - all processes that the current user owns
 - `ps aux` - processes belonging to every user, with extra info

For a dynamic view of the processes, `top` can be used. Top contains two parts, one with system information and one with a list of processes ordered by CPU activity.

From the system summary part:

 - time of day, uptime, number of users logged in, load average (the number of processes that are in runnable state and waiting for the CPU; average for last 60 seconds, 5 minutes, 15 minutes; under 1.0 means system is not busy)
 - tasks: summary of number of processes and states
 - percent of CPU(s)
    - % us - percent of CPU for user space processes (outside of kernel)
    - % sy - for system (kernel) processes
    - % ni - used by nice processes
    - % id - percent idle
    - % wa - percent of CPU waiting for I/0
 - details how physical RAM is used
 - details how swap space (virtual memory) is used

Top program accepts a number of keyboard options. For help, `h`; to quit `q`.

There are several ways to **control processes** from the shell.

To interrupt a running process, `Ctrl-c` politely asks the program to terminate. Most programs do that.

If we want to run a program but immediately get our shell prompt back, even if the program is running, it can be started in the *background* (as opposed ot the *foreground* where stuff visible to the surface are, like the shell prompt).

A program running in the background does not receive keyboard input, so is also immune to Ctrl-c for example. A process can be returned to the foreground with `fg`. 

A foreground process can be stopped/paused with `Ctrl-z`. We can continue its execution in the foreground or resume it in the background with `bg`.

**`<cmd: &; jobs; fg; bg; Ctrl-c; Ctrl-z;>`**

 - `cmd &` - execute in background; the output printed contains the job number and the PID of the process
 - `jobs` - see processes launched from our terminal (job control facility of the shell)
 - `fg %1` - %1 is a *jobspec*, the number of the job; optional if only 1
 - `bg %1` - resume a stopped program in the background

A process can be killed by sending it a **signal**. This is what the kill command does. Ctrl-c sends an INT (interrupt) signal, while Ctrl-z send a TSTP (terminal stop) signal. Programs listen to signal and react to them. This allows, for example to save work and terminate when receiving a kill or a Ctrl-c.

**`<cmd: kill; killall;>`**

 - `kill PID` - kill process (send it TERM signal)
 - `kill -signalnumber PID`
    - 1 - HUP - hangup; indicate that the controlling terminal has "hung-up" (from vintage days of modems and remote terminals); closing a terminal session triggers this, the foreground program will be sent this signal and will terminate. Also used by many daemons to reinitialize - restart and reread its configuration
    - 2 - INT - interrupt; usually terminates the process
    - 9 - KILL - kill; special signal, cannot be handled; kernel terminates the target process without process knowing about it
    - 15 - TERM - terminate; usually terminates the process; default signal for kill command
    - 18 - CONT - continue; restore a process after a STOP or TSTP signal; sent by bg and fg commands
    - 19 - STOP - stop; pause without terminating, like KILL it is not sent to the target process so it cannot be ignored
    - 20 - TSTP - terminal stop; sent by Ctrl-z, received by the process who should pause
    - 3 - QUIT
    - 11 - SEGV - segmentation violation; sent if a program makes illegal use of memory
    - 28 - WINCH - window change; sent by the system if the window changes size (program can redraw its-self, like top and less do)
 - `kill -l` - list of signals
 - `killall xlogo` - send TERM to all matching processes - all xlogo processes

You must be the owner of a process, or the superuser, to send it a signal with kill.

**Shutting down the system** means terminating all processes in a control fashion and giving the system chance to do some housekeeping.

The shutdown command can halt, poweroff or reboot and it can also handle a time delay. Once the shutdown command is executed a message is broadcasted to all logged-in users.

 - `halt`
 - `poweroff`
 - `reboot`
 - `shutdown`
    - `shutdown -h now` - poweroff the system now
    - `shutdown -r +4 Bye!` - reboot the system in 4 minutes and send the wall message to all users "Bye!"
    - `shutdown -c` - cancel a running shutdown

Other process related commands

 - `pstree`
 - `vmstat` - snapshot of system resource usage, including memory, swap, disk I/O
    - `vmstat 5` - continuous display, 5 seconds update interval
 - `xload` - graphical program - system load over time
 - `tload` - like xload, but in the terminal

## 2. Configuration and the environment

### Environment

The shell stores *shell variables* and *environment variables* in the environment. With bash, they are very similar. Shell variables consists of data placed there by bash, env variables are all the rest. The shell also stores *aliases* and *shell functions*.

**`<cmd: printenv; set; alias;>`**

 - `printenv | less` - list environment variables
	- `printenv USER` - list value for variable USER
 - `set | less` - list env variables, shell variables and shell functions (sorted alphabetically)
 - `echo $HOME` - list content of a variable
 - `alias` - list aliases

Some interesting env variables are: DISPLAY (name of X server display), EDITOR, SHELL, HOME, OLDPWD, LANG, PAGER, PATH, PS1 (prompt string 1), TERM (terminal emulator protocol), TZ (timezone), USER. Depending on distribution, some may not be set - that is not an issue.

The **environment is established** by reading a series of configuration scripts, *startup file*. First, default env for all users, followed by startup files in the user's home directory. The exact sequence depends on the type of shell session.

Types of shell sessions

 - login shell session - one in which we are prompted for our username and password (eg: virtual console)
	- `/etc/profile` - global for all users
	- `~/.bash_profile` - user's personal startup file
	- `~/.bash_login` - if `~/.bash_profile` not found, bash tries to read this
	- `~/.profile` - if neither of the previous two found, bash tries this; default in Debian based distros, like Ubuntu
 - non-login shell session - typically when a terminal session is started from the GUI
	- `/etc/bash.bashrc` - global
	- `~/.bashrc` - per user
		- note that most startup files for login shells are written in such a way as to read this as well

Note: in addition to reading the startup files, non login shells inherit the environment from their parent process (usually a login shell).

PATH (the list of locations where the shell looks to find commands) is usually set by /etc/profile and extended by local .profile startup file, for example by adding `$HOME/bin` to it. This means that if you create a bin directory (log back in) and add your own executables there, the shell will find them. The PATH is `export`-ed, so that the contents is available to child processes.

To **modify the environment**, a rule of thumb is to place PATH modifications and definitions of additional env variables in the `.bash_profile` startup file, or similar. For everything else, use `.bashrc`. Of course, to affect all users, the global files must be modified.

Text editors can be GUI or text. GUI: gedit (GNOME), kedit, kwrite, kate (KDE). Text: nano, vi (replaced on most distros by vim - vi improved), emacs (gigantic, all-purpose, does-everything programming environment).

**`<cmd: nano;>`**

nano was designed as a small text editor for an email client, so it does not have many fancy features, but it is easy to use. At the bottom it shows a menu with commands in which the `^X` notation means `Ctrl-x`.

 - Ctrl-x - exit
 - Ctrl-o - save, write out

Example of possible additions to .bashrc

```
    # Change umask to make directory sharing easier
    umask 0002

    # Ignore duplicates in command history and increase
    # history size to 1000 lines
    export HISTCONTROL=ignoredups
    export HISTSIZE=1000

    # Add some helpful aliases
    alias l.='ls -d .* --color=auto'
    alias ll='ls -l --color=auto'
```

To take effect, either start a new terminal session, or instruct bash to reread the modifications by `source ~/.bashrc`.

*Resources*

 - INVOCATION section of `bash` man page - full details of bash startup files

### Vi intro

POSIX (standard required for program compatibility on Unix systems) requires that vi ("vee eye") be present.

History: first version written by Bill Joy (co-founder of Sun Microsystems). Stands for visual editor; video terminals were just appearing, rather than printer-based terminals like teletypes. It allows editing with a moving cursor. Before there were line editors, which operated on a single specific line. vi still incorporates a line editor, called ex.

Most distributions ship with `vim` - vi improved. It is symbolically linked to vi.

Note: compatibility mode make vim closer to vi. We don't want that. If you see this, `set nocp` in your `~/.vimrc` file.

Vi is a *modal editor*, it acts depending upon which mode it's in. When first started it is in *command mode*, so almost every key will actually be a command.

To enter in *insert mode* press `i`. ESC will exit insert mode, back into command mode. To save our work we must enter an *ex* command - press `:` then `w` for writing and ENTER.

Note: in the Vim documentation, command mode is called *normal mode* and ex commands are called *command mode*.

Moving the cursor

 - h j k l - left, down, up, right
 - 0 (zero) - beginning of current line
 - ^ - first non-whitespace char of current line
 - $ - end of line
 - w - beginning of next word or punctuation
 - W - beginning of next word (ignores punctuation)
 - b - prev word or punctuation
 - B - prev word
 - Ctrl-f - page down
 - Ctrl-b - page up
 - numberG - go to line number; eg: 1G - start of file
 - G - last line

Many commands can be prefixed with a number. Example: 5h will move the cursor 5 positions to the left.

 - `a` - append, insert mode
 - `A` - move to end of line and append
 - `o` - open a new line below
 - `O` - open a new line above
 - `u` - undo last change

Deleting

 - `x`, `3x` - current character, current character plus next two
 - `dd` - current line
 - `5dd` - current line plus next four
 - `dW` - until beginning of next work
 - `d$` - until end of line
 - `d0` - until beginning of line
 - `d^` - until first non-whitespace char
 - `dG` - from current line to end of file
 - `d20G` - from current line to the 20th line

Cut, copy, paster

 - `d` also cuts the text - it puts it into a paste buffer. We can paste that content with `p` after the cursor, or with `P` before the cursor.
 - to copy the text, `y` is used (yank), much like `d`.

Join lines

 - J - joins current line with the next

Search

 - `f` searches within a line; eg: `fa` - moves to the next cursor of "a"
    - ';' will repeat the search
 - '/' searches within the entire file; similar to less (but also works with regular expressions)
    - 'n' to move to next match, `N' to the previous
    - `:set [no]ignorecase'

Search and replace (*substitution*)

 - `:%s/line/Line/g`
    - `:` - starts an `ex` command
    - `%` - range of lines affected; this means entire file, or `1,$`, but could also be only the first 3 lines (`1,3`)
    - `s` - the operation of substitution
    - `/line/Line/` - the search pattern and the replacement text
    - `g` - global, aka on every instance of the search string in the line, otherwise, only the first match in a line is considered
        - `gc` - ask for user confirmation

Editing multiple files

 - `vi file1 file2...` - open multiple files
 - `:bn` `:bp` - next, previous file
 - `:buffers`, `:buffer 1`
 - `:e filepath` - open an additional file (edit)
 - `:close 1` - close buffer

You can insert the content of an entire file into another.

 - `:r file` - it will copy the file content below the cursor position

Saving

 - `:w`
    - `:w filename.txt` - like a save as, but continues to edit current file
    - `:wq` - save an quit
 - `ZZ` - save the current file and exit (like `:wq`)

### Customizing the prompt

*backslash-escaped special characters*

 - \d - date, day month date
 - \h - hostname
 - \H - hostname with domain name
 - \j - number of jobs
 - \l - name of current terminal device
 - \s - name of shell program
 - \t \T \@ \A - time, different formats
 - \u - username
 - \v \V - version, version and release number of shell
 - \w - name of current working dir
 - \W - last part of cwd
 - \! - history number of current command
 - \# - number of commands in this shell session
 - \$ - $ or # for superuser privileges
 - \[ - start of non-printing characters (terminal emulator manipulation, like moving cursor or setting colors)
 - \] - end of sequence

You can try prompt designs by saving current PS1 into a local variable, then experimenting.

Most terminal emulator programs respond to certain non-printing character sequences to control things like **color**, bold text, blinking text, cursor position.

When terminals were hooked to remote computers, there were different brands and types of terminals. Unix-like systems have to complex subsystems to handle the complexities of terminal color: termcap and terminfo. ANSI developed a standard set ot character sequences to control video terminals.

Character color is controlled by sending the terminal emulator an *ANSI escape code*. It begins with an octal 033 - code generated by ESC key, then an optional character attribute, then an instruction. Eg: `\033[0;30m` - code to set the text color to normal (attribute = 0), black text.

 - `PS1="\[\033[0;37m\][\u@\h \W]\$\[\033[0m\] "` - light grey prompt, then back to previous color
 - `PS1="\[\033[0;42m\][\u@\h \W]\$\[\033[0m\] "` - green background color of prompt, then back to normal

Escape code can also move the cursor. Commonly used to provide a clock or some kind of information in a different location on the screen, such as in an upper corner each time the prompt is drawn.

 - `PS1="\[\033[s\033[0;0H\033[0;41m\033[K\033[1;33m\t\033[0m\033[u\][\u@\h \W]\$ "`
    - each non-printing character sequence must be inside `\[ \]` to allow bash to calculate the size of the visible prompt
    - \033[s - store the cursor position (not recognized by some terminal emulators)
    - \033[0;0H - move to line 0, column 0
    - \033[0;41m - set background color to red
    - \033[K - clear till the end of the line (it will be cleared to red); cursor position not changed
    - \033[1;33 - text color yellow
    - \t - current time
    - \033[0m - turn off color (both text and background)
    - \033[u - restore cursor position saved earlier

To **save the prompt** the PS1 setting must be added to .bashrc and the variable exported.

Note: much more can be done with prompts, including shell functions and scripts.

*Resources*

 - [The Linux Documentation Project - BASH prompt HOWTO](http://tldp.org/HOWTO/Bash-Prompt-HOWTO/)

## 3. Common Tasks and Essential Tools

### Package Management

The most important thing about a distribution is the *packaging system* and the vitality of the distribution's support community. The software landscape in Linux is very dynamic, there are regular updates, even daily for some packages.

Package management is a method of installing and maintaining software. There is not much need anymore to compile programs from source. Not that it is not useful, but using a package management system is faster and easier to deal with.

The two big camps of packaging systems are `.deb` (Debian) and `.rpm` (Red Hat).

The basic unit in a packaging system is the **package file**. It is a compressed collection of files that form the software package - programs, data files, metadata, pre and post installation scripts that perform configuration before and after the installation. They are created by a *package maintainer* - gets the software in source code from *upstream* (from the author), compiles it and creates the package metadata and any necessary installation scripts. Can apply modifications to the source code to improve the fit with the other parts of the Linux distribution.

Packages for a distribution are made available in central **repositories**. A distribution may maintain different repositories for different stages of development - "testing", "development", "stable" etc. A distribution may also have related third-party repositories - often needed to supply software that cannot be bundled in with the distribution, for legal reasons (patents, DRM). These repos are usually wholly independent of the distribution they support; the user must know about them and manually include them in the configuration files of the package management system.

Programs usually rely on other software to function - like routines in *shared libraries*. If a package requires a shared resource, it has a **dependency**. Package management systems provide some form of *dependency resolution* to ensure that when a package is installed, what is needed is also installed.

There are usually two types of tools

 - low-level tools - tasks such as installing and removing package files (dpkg / rpm)
 - high-level tools - metadata searching and dependency resolution (apt, apt-get, aptitude / yum, dnf)

**Common tasks**

**`<cmd: dpkg; apt-get; apt-cache; rpm; yum;>`**

 - finding a package
    - `apt-get update; apt-cache search search_string`
    - `yum search search_string`
 - install a package with full dependency resolution
    - `apt-get update; apt-get install package_name`
    - `yum install package_name`
 - install a package from a package file
    - `dpkg -i package_file`
    - `rpm -i package_file`
 - remove a package
    - `apt-get remove package_name`
    - `yum erase package_name`
 - updating packages from a repository
    - `apt-get update; apt-get upgrade`
    - `yum update`
 - upgrading a package from a package file (replacing the current one)
    - `dpkg -i package_file`
    - `rpm -U package_file`
 - list installed packages
    - `dpkg -l`
    - `rpm -qa`
 - determine whether a package is installed
    - `dpkg -s package_name`
    - `rmp -q package_name`
 - display info about an install packages
    - `apt-cache show package_name`
    - `yum info package_name`
 - find which package installed a file
    - `dpkg -S file_name`
    - `rpm -qf file_name`

*Resources*

 - [debian FAQ package management system](https://www.debian.org/doc/manuals/debian-faq/ch-pkgtools.en.html)

### Storage Media

The process of attaching a storage device to the file system tree is called *mounting*.

File `/etc/fstab` lists the devices that will be mounted at boot time. Fields:

 - 1 - device - name of device, but with hot pluggable devices, better to use a label (added to the storage media when it is formatted). Either a simple text label or a randomly generated UUID.
 - 2 - mount point - directory where it is attached to the file system tree
 - 3 - file system type - native Linux usually Fourth Extended File System (ext4)
 - 4 - mount options - ex: read-only, or non-executable
 - 5 - frequency - if and when a file system is to be backed up with the `dump` command
 - 6 - order - what order file systems should be checks with the `fsck` command

**`<cmd: mount; umount;>`**

View list of the file systems currently mounted - `mount`. Format: *device on mount-point type file-system-type (optons)*.

CD-ROM type: iso9660. Note that audio CDs are not like CD-ROMs, they do not have file systems to be mounted in the usual sense.

A device can be unmounted with `unmount dev_name`. To mount it somewhere else, first you should create that new *mount point*, simply a directory. Note that the directory can be non-empty, in which case its contents won't be visible while the new file system is mounted there.

To mount at the new mount point, the mount command is used again.

 - `su -`
 - `umount /dev/sdc`
 - `mkdir /mnt/cdrom`
 - `mount -t iso9660 /dev/sdc /mnt/cdrom`

If the device is in use by someone, or by some process, like changing the working directory there, `umount` will fail with "device is busy".

Note: unmounting is important. Operating systems store data for or from storage devices in memory for as long as possible before actually interacting with the (slower) device. It does this by using *buffers*. It makes writing to storage devices fast, because the OS will actually pile it up in memory and write it to the device at another time. Unmounting entails writing all remaining data to the device (if vital updates, like directory updates are not transferred to the device, *file system corruption* can occur).

*Device names*

All devices live in `/dev/`. Bellow are some naming patters used by Linux:

 - /dev/fd - floppy disk
 - /dev/hd - IDE (PATA) disks on older systems; hda / hdb - master / slave on first channel; hdc / hdd - second channel; a trailing digit represents the partition on the device
 - /dev/lp - printers
 - /dev/sd - SCSI disks; on modern Linux distros, the kernel treats all disk-like devices (PATA/SATA, flash drives, USB mass storage) as SCSI disks
 - /dev/sr - optical drives

To see the name of a new device use `tail -f` on /var/log/messages or /var/log/syslog. Example: `kernel: [46493.044865] sd 6:0:0:0: [sdb] Attached SCSI removable disk`.

*Partitions and file system utils*

**`<cmd: fdisk; mkfs; fsck;>`**

Manipulating partitions on a disk-like device can be done with `fdisk`. The device has to be unmonted, then: `sudo fdisk /dev/name`. The entire device must be given, not a partition name.

 - `fdisk /dev/name`
    - `m` - help / menu
    - `p` - print partition table of device
    - `l` - list know partition types (eg: `b` - windows 95 fat32)
    - `t` - change partition type (eg: `83` - Linux)
    - `w` - write partition table to disk

To create a new file system on a partition, the `mkfs` tool can be used

 - `sudo mkfs -t ext4 /dev/sdb1` or `-t vfat` for FAT32

*Testing and repairing filesystems*

At startup, Linux checks the filesystems before mounting them; it does this in the order specified in `/etc/fstab`, in the last column. The tool that does this is `fsck`. It can also repair some issues - the recovered portions if files are put in the `lost+found` directory.

 - `sudo fsck /dev/sdb1`

Note: "what the fsck?!" - unix culture, fsck used in place of a popular word

*Moving data directly to and from devices*

**`<cmd: dd;>`**

Disks can be seen not only as directories and files, but as raw data blocks as well. This gives us the ability, for example, to clone a disk.

The `dd` program does this - copies blocks of data from one place to another.

 - `dd if=input_file of=output_file [bs=block_size count=blocks]]`

Example: two identical drives attached to a computer, to copy all from one to another, or to a local file

 - `dd if=/dev/sdb of=/dev/sdc`
 - `dd if=/dev/sdb of=flash_drive.img`

*Creating and writing CD-ROM images*

**`<cmd: genisoimage; wodim;>`**

To make an ISO image on an existing CD-ROM, we can simpy use `dd` like in the above example (from CD-ROM device to local file). (for audio CDs, look at `cdrdao` command)

To create an ISO image with the contents of a directory

 - `genisoimage -o cd-rom.iso -R -J ~/cd-rom-files`
    - R - metadata for Rock Ridge extension (long filenames and POSIX-style file permissions)
    - J - Joliet extensions, long filenames for Windows

NOTE: `wodim` and `genisoimage` are a fork with replacement programs for `cdrecord` and `mkisofs`, which went through a partial change of license.

To mount an ISO image while it is still on our hard disk, we use the *loop* option to mount:

 - `mkdir /mnt/iso_image`
 - `mount -t iso9660 -o loop image.iso /mnt/iso_image`

To write an image on a CD/DVD, the `wodim` program is used

 - `wodim dev=/dev/cdrw blank=fast` - blank a rewritable media
 - `wodim dev=/dev/cdrw image.iso` - write image
    - `-v` - verbose
    - `-dao` - disc-at-once mode; this should be used if preparing a disc for commercial reproduction (default is track-at-once, suitable for recording music tracks)

*Integrity check*

**`<cmd: md5sum;>>`**

To verify the integrity of an ISO image a checksum is used. Usually, it is an md5 sum.

 - `md5sum image.iso`

This can also be used to verify if we have written an image correctly on the media. For CD-R and CD-RW disks written in disk at once we can simply do a sum over the entire device. Many types of media, such as DVDs, require a calculation of how many 2048-byte blocks does the image span (optical media is always written in 2048-byte blocks).

 - `md5sum dvd-image.iso; dd if=/dev/dvd bs=2048 count=$(( $(stat -c "%s" dvd-image.iso) / 2048 )) | md5sum`

*Resources*

 - [why dd doesn't work on audio CDs](https://www.quora.com/Why-doesnt-the-Linux-DD-command-work-with-audio-CDs)

### Networking

*Network monitoring*

**`<cmd: ping; traceroute; ip; netstat;>`**

 - `ping linuxcommand.org` - test network connection
 - `traceroute shashdor.org` - see routers along the way to a host
    - `*` in the output means that the hop does not provide identifying information, due to router configuration, network congestion, firewall etc; may be overcome by using `-T / -I` as parameter (tcp syn / icmp echo)
 - `ip` - multi-purpose network config tool (replaces the deprecated `ifconfig`)
    - `ip a` - show network interfaces, their state and configuration
 - `netstat` - examine various network setting and statistics
    - `netstat -ie` - examine the network interfaces in our system, including stats
    - `netstat -r` - kernel's routing table

*File download*

**`<cmd: ftp; lftp; wget;>`**

One of the first programs of the Internet is `ftp` - gets its name from the *File transfer protocol*. Browser now support URIs of the form `ftp://`, but before browsers there were FTP clients that talked with FTP servers. In its original form it is not secure - it sends username and passwords in clear text. Over the internet, usually *anonymous FTP servers* are used - anyone can log in with the username *anonymous* and a meaningless password.

 - `ftp fileserver` - connect to a FTP server (fileserver.localdomain)
    - *anonymous* username (some servers accept a blank passwords, others expect one in the form of an email)
    - `cd` - change directory (note that `pub` is usually where public info is storec on a FTP server)
    - `ls`
    - `lcd Desktop` - change local directory
    - `get ubuntu-xxx.iso` - download file from FTP dir to local dir
    - `bye` - exit

A better FTP client is `lftp` - it has many convenience features - automatic retry, background processes, tab completion, multiple-protocol support (including HTTP), others.

Another popular program for file downloading is `wget` - download from both web and FTP sites. Single files, multiple files or even entire sites can be downloaded.

 - `wget http://linuxcommand.org/index.php` - see man page for features like recursive download, background download, complete partial download

*Secure communication*

**`<cmd: ssh;>`**

Even before the Internet age, UNIX systems were routinely remotely administered. The programs used were `rlogin` and `telnet`. Both send data in cleartext.

The SSH (Secure Shell) was developed to address this problem. It 1) authenticates that the remote host is who it says and 2) it encrypts all the communication. Port 22 is used on the server side to listen for incoming connections from SSH clients. Most Linux distros ship with the OpenSSH implementation (from the OpenBSD project), some with only the client, some with client and server.

 - `ssh [user@]remote-system`
 - `ssh [user@]remote-system 'ls *' > dirlist.txt` - run ls on a remote system and redirect the output to a local file
    - note the `' '`, single quotes, since we do not want the *pathname expansion* to take place on our local shell, but on the remote

The SSH client stores authentication data for remotes in `~/.ssh/know_hosts`, one per line. If a server changes its id, you should verify with the admin that this is really the case, and then remove the entry from the local storage. The idea is to prevent a man-in-the-middle-attack.

When connection to a remote host, SSH establishes a *tunnel*. By default, commands typed at the local system go into the tunnel, as does the result of those commands on the other side. But this can also be used to send most types of traffic, creating a sort of VPN. One of the most common uses of this is X Windown traffic - on a system running an X server (GUI), it is possible to run an X client (a graphical application) on a remote system and have its output, its display appear on the local system.

 - `ssh -X remote-system; xload`

**`<cmd: scp; sftp;>`**

OpenSSH also includes two programs that can use the encrypted tunnel to copy files.

`scp` works like `cp`, but it can connect to a remote system.

 - `scp remote-sys:document.txt .` - copy document.txt from our home on remote-sys into the current (local) directory

`sftp` is a secure replacement for the `ftp` program. One thing to note is that it does not require an FTP server to run on the remote system, it only requires the SSH servers.

 - `sftp remote-sys`
    - `ls`
    - `lcd Desktop`
    - `get ubuntu-xx.iso`
    - `bye`

SFTP is usually supported by many file-managers. An URI beginning with `sftp://` will allow you to operate files on a remote system.

The most common SSH client for Windows is PuTTY. Also used is MobaXterm.

### Searching for Files

**`<cmd: locate;>`**

The `locate` program performs a rapid database search of pathnames.

 - `locate bin/zip` - look for a program starting with zip (since it is a program, we assume its path ends with `bin/`)
 - `locate zip | grep bin` - not sure if it stars with zip
 - `sudo updatedb`

The locate database is created by another program - `updatedb`, which is usually run as a *cron job*. To update instantly, run it with superuser privileges, as exemplified above.

**`<cmd: find;>`**

`find` searches a given directory and its subdirecotries for files based on a variety of criteria. In its simplest form, find is given one or more names of directories to search, so it can be used to list files in a directory tree.

 - `find ~` - listing of home directory, usually quite large

To identify files that match a specific criteria, find uses *options*, *tests* and *actions*.

Tests

Searching by file type is a common use-case

 - `find ~ -type d` - the test *-type d* limits the search to directories
 - `find ~ -type f` - only regular files files
 - `b` - block special device file; `c` - character dev file; `l` - symlink; `f`; `d`

File size and name can also be a test

 - `find ~ -type f -name "*.JPG" -size +1M` - JPG pictures larger than 1 megabyte
	- the double quotes are to prevent pathname expansion by the shell
	- units can be specified in: `b` - 512-byte blocks (default); `c` - bytes; `w` - 2-byte words; `k`; `M`; `G`

Other common tests supported by find

 - `cmin n` - content or attributes last modified exactly n minutes ago (`-n` - less than n minutes ago)
 - `cnewer file` - content or attributes last modified more recently than those of file
 - `ctime n` - content or attributes last modified n x 24 hours ago
 - `empty` - empty files and directories
 - `group name`
 - `iname pattern` - case-insensitive
 - `inum n` - inode number n (can find all hard links to a particular inode)
 - `mmin n` - content last modified n minutes ago
 - `mtime n` - content last modified n x 24 hours ago
 - `name pattern` - wildcard pattern
 - `newer file` - contents modified more recently than file (useful when writing scripts that perform backups, for example, by modifying a log file when a backup is performed and then only backuping the newer files)
 - `nouser` - files and directories that do not belong to a valid user
 - `nogroup` - same, for group
 - `perm mode` - either octal or symbolic, files or directories with permissions set to mode
 - `samefile name` - like inum, same inode number as file name
 - `size n`
 - `type c`
 - `user name` - username or user ID

Operators

With so many tests, find also offers a way to express logical relantionships between the tests. For example, determine if all the files and directories have secure permissions - look for all files that do not have 0600 and all directories that do not have 0700.

 - `find ~ \( -type f -not -perm 0600 \) -or \( -type d -not -perm 0700 \)`

Operators are simple and, or, not and grouping. Logical AND is implied by default. The logical expressions are not evaluated if not needed, for performance.

 - `-and / -a`, `-or / -o`, `-not / !`, `( )`

Predefined actions

We can also act on the items of the list produced by find. Predefined or user defined actions can be applied. Some predefined actions are listed below

 - `-delete` - it does not prompt for confirmatio, so use `-print` first to see the list returned
 - `-ls` - like performing `ls -dils`
 - `-print` - output full pathname of matching file to standard output (default if no other action specified)
 - `-quit` - quit once a match was made

Note that actions and tests are treated the same from the logical operators' point of view. For example:

 - `find ~ -type f -name "*.bak" -print` - find all back files
 - `find ~ -print -type f -name "*.bak"` - will print out all files since it is the first test and it evaluates to true, then it will continue to test the other criteria

User-defined Actions

We can also invoke arbitrary commands. The traditional way is used `-exec` action: `-exec command {} ;` - the `{}` is a representation of the current pathname and the semicolon is required to indicate the end of the command.

 - `exec rm '{}' ';'`
	- brace and semicolon have special meaning to the shell, so quote or escape

To execute an action interactively, `-ok` action should be used.

 - `find ~ -type f -name 'foo*' -ok ls -l '{}' ';'`

To not call a new instance of the command each time a match is found, we can combine all the search results and launch a single command with them as arguments. This can be done the tradition way, using the external command `xargs`, or using a new feature in `find` itself. The latter is achieved by simply the semicolong character to a plus sign.

 - `find ~ -type f -name 'foo*' -ok ls -l '{}' +` - ls command is executed on an argument list comprised of the search results

**`<cmd: xargs;>`**

The xargs commands accepts input for standard input and converts it into an argument list for a specified command.

 - `find ~ -type f -name 'foo*' -print | xargs ls -l`

Note: the number of command line arguments is large, but not unlimited. xargs gets around this by executing the command with the maximum number possible and then repeats the process until standard input is exhausted. To see the maximum size of the command line: `xargs --show-limits`

Names can contain embedded spaces, or even newlines, in their names. This is a problem for programs like xargs that construct argument lists for others. To overcome, find can be told to delimit the files with *null character*, 0, and xargs can be told to accept null-separated input.

 - `find ~ -name '*.jpg' -print0 | xargs --null ls -l`

---

**`<cmd: touch;>`**

Usually used to set or update the access, change and modify times of files, it is also used to create an empty file.

 - `mkdir -p p/dir-{001..100}`
 - `touch p/dir-{001..100}/file-{A-Z}`

The above creates 100 subdirectories with 26 empty files each.

 - `touch p/timestamp` - creates and empy file and sets its modification time to current time
 - `stat p/timestamp - reveals all that the system understands about a file and its attributes

Example

 - `touch p/timestamp`
 - `find p -type f -name 'file-B' -exec touch '{}' ';'` - update file-B files
 - `find p -type f -newer p/timestamp' - find files newer than a file

To find the files and directories with not ok permission, and then to change these permissions

 - `find p \( -type f -not -perm 0600 -exec chmod '{}' ';' \) or \( -type d -not -perm 0700 -exec chmod 0700 '{}' ';' \)`

---

Options

The options are used to control the scope of a *find* serach. They may be included with other tests and actions when constructing expressions.

 - `-depth` - process a directory's files before the directory itself
 - `maxdepth levels` - how much find will descen
 - `mindepth levels` - minimum number of level *find* will descend before applying tests and actions
 - `mount` - not to traverse directories that are mounted on other file systems
 - `noleaf` - not optimze search based on the assumption that it is searching a Unix-like filesystem (for dos/windows, cd-rom filesystems)

*Resources*

 - [GNU findutils](https://www.gnu.org/software/findutils/)

### Archiving and Backup

*Compressing files*

Data compression is removing redundancy from data. For example, instead of having a 100 x 100 pixels black picture, we could specify it as 1 black pixel, 10 000 times. This is one of the most rudimentary compression schemes, called *run-length encoding*.

Compression algorithms can be *lossless* (preserves all data, a restored version is exactly the same) or *lossy* (removes some of the data; examples are JPEG and MP3). For compressing data files we talk only about *lossless*.

**`<cmd: gzip; gunzip; zcat; zless;>`**

`gzip` replaces the original file with a compressed version. `gunzip` restores it.

 - `gzip foo.txt` - replaces foo.txt with foo.txt.gz, smaller in size, same persmissions, same timestamp
	- `-c / --stdout / --to-stdout` - keep original files, write output to standard output
	- `-d / --decompress / --uncompress` - act like gunzip
	- `-f / --force` - force compression even if a compressed version of the file exists
	- `-l / --list` - list compression stats for each existing compressed file
	- `-r / --recursive` - if arguments are directories, recursevily compress files withing them
	- `-v` - verbose
	- `-t` - test the integrity of an existing compressed file
	- `-number` - set amount of compression from 1 (`--fast`) to 9 (`--best`); default is 6
 - `gunzip foo.txt[.gz]` - restores the original version, same permissions and timestamp
	- `gunzip -c foo.txt | less` - only view the contents of a compressed file (or use `zcat` supplied with gzip, does the same thing; or even `zless`)

**`<cmd: bzip2; bunzip2; bzcat; bzip2recover;>`**

The `bzip2`program is similar but achieves hight compression and the cost of speed. The usual extension is `.bz2`. The program can be used in the same way as gzip. Note that the *-number* option has a different meaning and it does not support *-r*. It also comes with a `bzcat` program, as well as a program that will try to recover damaged bz2 files, `bzip2recover`.

*Note*: compressing an already compressed file, like a jpg, will most likely give you a large size. There is no redundant data left that could be saved so that it outweighs the overhead added by the compression scheme.

*Archiving files*

Archiving means gathering up many files and budling them together into a single large file. It is usually done in backup, when moving data from one system to another.

**`<cmd: tar;>`**

The `tar` program is the classic tool for archiving in Unix world (*tape archive*).

 - `tar mode[options] pathname...`
	- `c` (mode) - create an archive from a list of files and/or directories
	- `x` (mode) - extract an archive
	- `r` (mode) - append specified pathnames to the end of an archive
	- `t` (mode) - list the contents of an archive
 - `tar cf playground.tar playground-dir` - the `f` option specified the name of the tar archive to be created out of the directory hierarchy; it will also include the top playground-dir directory
 - `tar tvf playground.tar` - list the contents, with more details - `v` option
 - `tar xf pathto/playround.tar` - extract the content of the archive in the current directory
	- unless done as superuser, files and directories extracted take on the ownership of the user doing the operation rather than the original owner

Another interesting behaviour is how tar handles pathnames. The default is relative; tar removes any leading slash from the pathname when creating the archive.

 - `[host1]$ sudo tar cf /media/bigdisk/home.tar /home` - archive home folder on external media
 - `[host2]$ cd / && sudo tar xf /media/bigdisk/home.tar` - change firs to root dir since the pathnames inside are relative

You can also specify pathnames to be extracted from inside the archive, instead of extracting the entire contents. The GNU version even allows pattern matching with the `--wildcards` option.

 - `cd foo && tar xf ../playground.tar --wildcards 'playground/dir-08*'`

Often tar is used in conjunction with find to produce archives. For incremental backups, find can use a "timestamp" file to find newly modified files and add the pathnames to an archive.

 - `find playground -name 'file-A' -exec tar rf playground.tar '{]' '+'`

Tar can also make use of standard input and standard output. In the example below, tar accepts the standard `-` to indicate either standard input or output, as needed; the `--files-from / -T` option causes tar to read its list of pathnames from a file rather than the command line. The output of tar is piped into gzip to produce a compressed archive (conventional extension `.tgz`, or `tar.gz`).

 - `find playground -name 'file-A' | tar -cf - --files-from=- | gzip > playground.tgz`
 - `find playground -name 'file-A' | tar -czf playground.tgz -T -`
	- the compression can also be done directly by modern version of tar (both gzip and bzip) with `z`, `j` options. For bzip compression, the extension usually is `.tbz`

Using stdin and stdout we can also transfer files between networked systems.

 - `mkdir remote-stuff`
 - `cd remote-stuff`
 - `ssh remote-sys 'tar cf - Documents' | tar xf -` - this will give uss the Documents folder in current dir

**`<cmd: zip; unzip;>`**

The `zip` program is both a compression tool and an archiver. The file format is familiar to Windows users.

 - `zip options zipfile file...`
 - `zip -r playground.zip playground` - create a zip archive out of the playground directory and its contents
 - `cd foo; unzip ../playground.zip` - extract the contents of a zip file in the current directory
 - `unzip -l playground.zip playground/dir-087/file-Z` - the l option only tells unzip to list the contents; `-v` can be added to increase verbosity
 - `cd food; unzip ../playground.zip playground/dir-087/file-Z` - extract specific file from archive; if the file already exists, the user is prompted

The messages shown by zip when creating an archive represent if the file or directory is *stored* as is or is *deflated* (compressed). A difference from tar is that if the archive already exists, zip will update it rather than replace it. It will add new files and matching files will be replaced.

zip can also use standard input and output, but not quite like tar

 - `find playground -name "file-A" | zip -@ file-A.zip` - the `@` options allows us to pipe a list of filenames

zip can also write its output to stdout but few programs can make use of the output. Unfortunately, the unzip program does not accept standard input, so zip and unzip can't be used together to perform network file copying like tar. zip does accept standard input so it can be used to compress the output of other programs while unzip can redirect its output to standard output

 - `ls -l /etc/ | zip ls-etc.zip -`
 - `unzip -p ls-etc.zip | less`

*Synchronizing Files and Directories*

**`<cmd: rsync;>`**

A common backup strategy is to keep on or more directories synchronized with another directory or directories located on either the local system (usually a removable storage device of some kind) or a remote system. In the Unix world, the preferred tool for this is `rsync` that uses the *rsync remote-update protocol*, which allows rsync to quickly detect differences between two directories and perform the minimum copying required.

 - `rsync options source destination` - source and destination can be
	- local file or directory
	- remote file or directory `[user@]host:path`
	- a remote rsync server specified with a URI of `rsync://[user@]host[:port]/path`
		- either the source or destination must be local
 - `rsync -av playground foo` - sync the playground directory with a local copy in foo
	- `-a` - archiving - causes recursion and preservation of file attributes
	- `-v` - verbose

rsync will detect if there are no changes and a susequent mirroring of playground to foo/playground will copy no files. If we `touch` a file in the source, it will update it.

Note there is a subtle feature when specifying a rsync source

 - `rsync source-dir destination-dir` - source-dir will be copied into destination-dir (destination-dir/source-dir/)
 - `rsync source-dir/ destination-dri` - the contents of source-dir will be copied

A simple example backup of important stuff to a "big" external disk

 - `mkdir /media/bigdisk/backup`
 - `sudo rsync -av --delete /etc /home /usr/local /media/bigdisk/backup`
	- the `--delete` option will remove files that may have existed in the backup folder, but don't exist anymore in the source

A simple alias could be

 - `alias backup='sudo rsync -av --delete /etc /home /usr/local /media/bigdisk/backup'`

One of the important features of rsync is that it can be used to *copy files over a network*. The first way to do this is with another system that has rsync installed, along with a remote shell program such as ssh.

 - `sudo rsync -av --delete --rsh=ssh /etc /home /usr/local remote-sys:/backup`
	- `--rsh=` - which remote shell rsyhnc will use to connect and transfer data

The second way to do this is by using a *rsync server*. rsync can be configured to run as a daemon; usually this is done to allow mirrorin of a remote system

 - `mkdir fedora-devel` 
 - `rsync -av --delete rsync://archive.linux.duke.edu/fedora/linux/development/rawhide/Everything/x86_64/os/ fedora-devel`

### Regular Expressions





 







