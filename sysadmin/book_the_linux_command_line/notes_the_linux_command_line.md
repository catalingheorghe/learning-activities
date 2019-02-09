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

### Permisssions

**Users** in Unix are split up into owners, group members and the world.

Each user has a number, *user id - uid*, which is mapped to a human readable name. Also, each user has assigned a *primary group ID - gid* and may belong to several groups.

Different distributions manage things differently regarding permissions. Fedora start with regular user ids at 500, while Ubuntu at 1000. In ubuntu, a user is assigned to more groups in order to manage permissions for different services.

The files used to manage this are: `/etc/passwd` - user (login) name, uid, gid, account name, home dir, login shell; `/etc/groups`; `/etc/shadow` - holds the passwords for users.

Traditional Unix style systems assign regular user to a *users* group, while modern Linux distribution create a unique, single member group with the same name as the user. Makes permission management easier.

**<cmd: id;>**

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

**<cmd: chmod; umask;>**

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

**<cmd: su; sudo;>**

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

**<cmd: chown; chgrp;>**

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

**<cmd: passwd;>**

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

**<cmd: ps; top;>**

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

**<cmd: &; jobs; fg; bg; Ctrl-c; Ctrl-z;>**

 - `cmd &` - execute in background; the output printed contains the job number and the PID of the process
 - `jobs` - see processes launched from our terminal (job control facility of the shell)
 - `fg %1` - %1 is a *jobspec*, the number of the job; optional if only 1
 - `bg %1` - resume a stopped program in the background

A process can be killed by sending it a **signal**. This is what the kill command does. Ctrl-c sends an INT (interrupt) signal, while Ctrl-z send a TSTP (terminal stop) signal. Programs listen to signal and react to them. This allows, for example to save work and terminate when receiving a kill or a Ctrl-c.

**<cmd: kill; killall;>**

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





















