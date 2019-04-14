# The Linux Command Line

William Shots

## Table of Contents

0. About
1. Shell notes and Linux intro
2. Configuration and the environment
3. Common tasks and essential tools
4. Writing shell scripts

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
    - `-n` - number of lines to print (`- / +` can be used to exclude last / first lines)
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
    - `\033[s` - store the cursor position (not recognized by some terminal emulators)
    - `\033[0;0H` - move to line 0, column 0
    - `\033[0;41m` - set background color to red
    - `\033[K` - clear till the end of the line (it will be cleared to red); cursor position not changed
    - `\033[1;33` - text color yellow
    - `\t` - current time
    - `\033[0m` - turn off color (both text and background)
    - `\033[u` - restore cursor position saved earlier

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

To make an ISO image of an existing CD-ROM, we can simpy use `dd` like in the above example (from CD-ROM device to local file). (for audio CDs, look at `cdrdao` command)

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

The options are used to control the scope of a *find* search. They may be included with other tests and actions when constructing expressions.

 - `-depth` - process a directory's files before the directory itself
 - `-maxdepth levels` - how much find will descen
 - `-mindepth levels` - minimum number of level *find* will descend before applying tests and actions
 - `-mount` - not to traverse directories that are mounted on other file systems
 - `-noleaf` - not optimze search based on the assumption that it is searching a Unix-like filesystem (for dos/windows, cd-rom filesystems)

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

Text manipulation play a big role in Unix-like systems, and regular expressions are a very powerful tool for this.

Regular expressions are symbolic notations used to identify patterns in text. In a way similar to shell wildcard, but much more capable. Note that there are different versions of regular expressions, they vary from tool to tool and programmin language to language. This will present regular expressions as in the **POSIX variant** (which covers most of the command line tools). Most programming languages (most notably Perl) uses a slightly larger and richer set of nations.

**`<cmd: grep;>`**

"global regular expression print" - search text file for occurences of text matching the regular expression and prints any line with a match in it.

Common grep options:

 - `-i / --ignore-case`
 - `-v / --invert-match` - print lines that do not contain a match
 - `-c / --count` - print the number of matches instead of the lines themselves
 - `-l / --files-with-matches` - print name of each file that contains a match instead of lines themselves
 - `-L / --files-without-match`
 - `-n / --line-number` - prefix with line number
 - `-h / --no-filename` - for multi-file searche, suppress the output of filenames

Examples of usage, considering we have multiple files that contain listing of various directories from the system.

 - `grep bzip dirlist*.txt` - pathname expansion will give all files as arguments to grep

*Literals and metacharacters*

"bzip" is a very simple regular expression used in the previous example: a match will occurt if the line in the file contains at least four characters and that somewhere in the line the characters in "bzip" are in exactly this order. The characters in "bzip" are *literal characters* - they match the letters.

Regular expressions use *metacharacters* to specify more complex matches: `^ $ . [ ] { } - ? * + ( ) | \`. The backslash is used to escape metacharactes and in a few cases to create *meta sequences*.

Note that many regular expression metacharacters are also special for the shell. To not be expanded, the regular expression needs to be quoted.

 - any character `.` - matches any character, exactly one character
	- `.zip` - four characters; including also the literal ".zip"
 - anchors - the caret `^` and dollar `$` - match the beginning or end of the line respectively
	- `grep -h '^zip' dirlist*.txt` - zip, zipcloack, zipinfo ...
	- `grep -h '^zip$ dirlist*.txt` - zip
	- `^$` - this will match blank lines
	- `grep -i '^..j.r$' /usr/share/dict/words` - crossword solver using the dictionary that comes with linux distributions
 - bracket expressions and character classes - match a character from a set of characters
	- bracket expressions - match a character from a set of characters; the set can include metacharacters
		-`grep -h '[bg]zip' dirlist*.txt`
	- two metacharacters still have special meaning inside: `^` - negation, `-` - range
		- only if `^` is the first character in a bracket expression, then the set is an inverted match
			- `grep -h '[^bg]zip' dirlist*.txt` - not b or g, still requires one character
		- traditonal character ranges
			- `grep -h '^[A-Z]' dirlist*.txt`
			- `^[A-Za-z0-9]` - first character is a letter or number
			- `[-AZ]` - here the dash will not be treated specially; this will match either dash, A or Z
		- POSIX character classes
			- `[:alnum:]`
			- `[:word:]` - alnum plus `_` underscore
			- `[:alpha:]`
			- `[:blank:]` - space and tab
			- `[:cntrl:]` - ASCII control code - chars 0-31 and 127
			- `[:digit:]`
			- `[:graph:]` - visible characters; in ASCII, 33 - 126
			- `[:lower:]`
			- `[:punct:]` - punctuation characters
			- `[:upper:]`
			- `[:print:]` - printable charactes, graph puls the space character
			- `[:space:]` - whitespace characters including space, table, carriage return, newline, vertical tab, form feed.
			- `[:upper:]`
			- `[:xdigit:]` - characters for hexadecimal numbers

*Note about wildcards and character ranges*

**`<cmd: locale;>`**

In the beginning of UNIX, it used only ASCII characters. ASCII *collation order* is first uppercase, then lowercase, not like the dictionary order aAbB and so on. Support for characters not found in the English language was added by expanding the ASCII table to use characters 128-255. POSIX introduced a concept called *locale*, which could be adjusted to select the character set. `echo $LANG` will show the language setting in our system. With `en_US.UTF-8` POSIX compliant applications will use the dictionary order for ranges; that is why `[A-Z]` will not be only uppercase letters. POSIX then introduces character classes to address this. Note that there is still no convient way to express partial ranges, such as `[A-M]` for shell pathname expansion.  

The LANG variable contains the name of the language and the character set used in your locale. This is determined when a language was selected during installation. See `locale` command. To revert to traditional UNIX behaviours, to use U.S. English - ASCII: `export LANG=POSIX`. Can be made permanent by adding it to `.bashrc`.

*POSIX Basic vs Extended regular expressions (BRE, ERE)*

**`<cmd: egrep;>`**

The difference lies in the metacharacters recognized by the regular expression.

 - BRE: `^ $ . [ ] *`
 - ERE: BRE + `( ) { } ? + |`

However, the (, ), {, and } are treated as metacharacters in BREs if they are escaped with a backslash, while preceding them with backslash in ERE will make it a literal.

`grep` is usually BRE and `egrep` is ERE, but GNU version of `grep` supports extended regular expressions with the `-E` option. The next parts pertain to ERE.

*Note about POSIX standard*

In 1980s Unix was a popular commercial os. Vendors licenses the source code and attempted differentiation by customizing it, which lead to compatibility issues. This perios is now know as *the Balkanization*. IEEE began developing a set of standards to define how Unix and Unix-like systems perform - application programming interfaces, shell, utilities. The name POSIX was suggested by Richard Stallman - *Portable Operating System Interface* and an X for snappiness.

*Alternation*

Alternation allows a match to occur from among a set of expressions. To combine alternation with other regular expression elements, the () can be used.

 - `echo "AAA" | grep -E 'AAA|BBB'` - match either the string AAA or the sring BBB; notice the quotes of the extended regular expression in order to prevent the shell from interpreting the vertical bar as a pipe operator
 - `echo "CCC" | grep -E 'AAA|BBB|DDD'`
 - `grep -Eh '^(bz|gz|zip)' dirlist*.txt` - without the parantheses it would mean begins with bz or contains gz or contains zip, not begins with one of them

*Quantifiers*

ERE support specifying a number of times an element is matched.

 - `?` - match the preceding element zero or one time - basically, optional
	- `^\(?[0-9][0-9][0-9]\)? [0-9][0-9][0-9]-[0-9][0-9][0-9][0-9]$` - a regular expression for a phone number that can have the prefix in parantheses or not; the parantheses are escaped because they must be treated as literals, not metacharacters (ERE)
 - `*` - match the preceding element zero or more time
	- `[[:upper:]][[:upper:][:lower:] ]*\.` - a crude definition of a sentence - starts with any capital letter and is followed by any number of lowercase, uppercase or space until the final period
 - `+` - match an element one or more times
	- `^([[:alpha:]]+ ?)+$` - groups of one or more alphabetic characters separated by single spaces
 - `{}` - match an element a specific number of times
	- `{n}` - exactly n times
	- `{n,m}` - between n and m times
	- `{n,}` - at least n times
	- `{,m}` - at most m times
		- `^\(?[0-9]{3}\)? [0-9]{3}-[0-9]{4}$` - the regular expression from before, shorter

*Putting regular expressions to work*

Validate a phone list with grep: we have a text file with phone numbers one per line. We can scan for invalid numbers by using `-v` option to grep and the regular expression from before.

 - `grep -Ev '^\([0-9]{3}\) [0-9]{3}-[0-9]{4}$' phonelist.txt`

Find ungly filenames with find: find has a test that can uses regular expressions; unlike grep, which will print a line that contains a match, for find, the pathname must match exactly. We can use this to find files that have in their pathname characters outside `[-_./0-9a-z-A-Z]`

 - `find . -regex '.*[^-_./0-9a-zA-Z].*'` - we used zero or more instances of any characters at both ends, for a full match, and a negated bracket expression for the unacceptable characters

Searching for files with locate: locate supports basic (`--regexp`) and extended (`--regex`) regular expressions

 - `locate --regex 'bin/(bz|gz|zip)'`

Searching for text in less and vim. less supports extended regular expressions, vim basic - so for vim the extra metacharacters must be escaped with a backslash. In vim, `:hlsearch` command mode command activates text match highlighting in the file.

To find more applications that use regular expressions we can search the man pages

 - `cd /usr/share/man/man1`
 - `zgrep -El 'regex|regular expression' *.gz`

*Note: one feature from basic regular expressions, back references, will be covered in the next chapter.*

### Text Processing

There are many tools used to "slice and dice" text.

Applications of text are vast:

 - configuration files
 - documents - large documents in text format and then embed markup language to describe the formatting
	- web pages - text documents either use *Hypertext Markup Language* or *Extensible Markup Language* for the visual format
	- email - even non-text attachments are converted into text representation for transmission; a mail message seen with less with show a header with metadata and then the body with content
	- printer output - output destined for a printer is sent as plain text, or if the page contains graphics, converted into a text format *page description language* known as PostScript, which is then set to a program that generates the graphic dots to be printed
	- program source code

*A second look at some programs*

**`<cmd: cat; sort; uniq;>`**

*cat*

 - `-A` - display control characters; eg: MS-DOS line endings or lines of text with trailing spaces
 - `cat > foo.txt` - primitive word processor to create and edit a file; Ctlr-d indicates end of file
 - `-n` - disaply also line numbers
 - `-s` - supress the output of multiple blank lines (show only one)

Note about MS-DOS text vs Unix text: Unix and DOS define line endings differently. Unix ends a line with linefeed character (ASCII 10), while MS-DOS uses the sequence carriage return (ASCII 13) and linefeed. `dos2unix` and `unix2dos` are programs that convert lineendings, but this can be done with "basic" programs as well.

*sort*

Sort the contents of standard input or one or more specified files and sends the results to stdout.

 - `sort file1 file2 file3 > final_sorted.txt` - merge multiple files into a sorted one
 - `-b / --ignore-leading-blanks` - sort based on the first non-whitespace characters
 - `-f / --ignore-case` - make sorting case-insensitive
 - `-n / --numeric-sort`
	- see also `-g` which is general numeric value, not string numerical value
 - `-r / --reverse`
 - `-k / --key=field1[,field2]` - sort basedon a key field rather than the entire line
 - `-m / --merge` - treat arguments as pre-sorted and only merge them
 - `-o / --output=file`
 - `-t / --field-separator=char` - define field separator character; by default, spaces or tabs

Sort the output of ls by size (even if can do it as well) by telling sort to use the 5th *field* in the *records* from the *tabular data* produced by ls.

 - `ls -l | sort -nrk 5 | head`

To properly use the k option, we have to understand how sort understands fields. By default the delimiters are whitespace characters (spaces and tabs). Note that the delimiters are included in the field, so they can affect sorting - for example different number of leading spaces. Also, the power comes from the fact that the sort can be done on multiple keys. The k option can be specified multiple times and it permits a range of fields. If no range is specified, the key is the field plus the rest of the line until the end.

 - a file of lines in the format `Fedora	10		11/25/2008`
	- `sort -key=1,1 -k 2n distros.txt` - first key is the distro name; second key starts at field 2 and is numeric (or even better, `g`, to handle decimal numbers properly)
	- dates in computers are usually in the format YYYY-MM-DD, for easier chronological sorting; but the american format is MM/DD/YYYY; the key option allows specification of *offsets* within fields
		- `sort -k 3.7nbr -k 3.1nbr -k3.4nbr distros.txt` - the `b` option is important because it ignores leading whitespaces, which is different between lines so can affect the sort
 - a file like `/etc/passwd/` uses `:` (colons) for delimiters
	- `sort -t ':' -k 7 /etc/passwd`  - sort by default shell

*uniq*

Compared with sort, uniq is lightweight. When given a sorted file (or standard input), it removes any duplicate lines.

Note that the GNU version of sort has a `-u` option which removes duplicates.

 - `-c / --count` - list of duplicate lines preceded by the repetition count
 - `-d / --repeated` - output only repeated lines rather than unique lines
 - `-f n / --skip-fields=n` - ignore n leading fields in each line; fields are separated by whitespace; no option to specify delimiter
 - `-i / --ignore-case`
 - `-u / --unique` - print only unique lines

*Slicing and dicing*

**`<cmd: cut; expand; paste;>`**

*cut*

Extract a section of text from a line and output it to standard output. It can accept multiple files or input from standard input.

 - `-c list / --characters=list` - extract the portion of the line defined by list - one or more comma-separated numerical ranges
 - `-f list / --fields=list` - extract one or more fields
 - `-d delim / --delimeter=delim` - for use with fields, specify the delimiting character; by default, fields must be separated by a single tab character
 - `--complement` - extract the inverse

The input for cut is not very flexible; you can use `cat -A` to see if a text is strict enough.

 - `cut -f 2 distros.txt` - note that if the fields are not separated by exactly one tab, it won't give the expected results.

If the file is spaces delimited, instead of tab delimited, on each line it should have a certain field at the same position as on any other line. This is when character extraction can be used

 - `cut -f 3 distors.txt | cut -c 7-10` - extract the year from a list of MM/DD/YYYY by specifying a range

If we want to extract via characters from a tab separated file, we need to expand the tabs. GNU coreutils offers a pair of programs that handles tab expansion

 - `expand distros.txt | cut -c 23-` - expand tabs and then extract every character from the 23rd position to the end of the line (the year, again)
 - `expand distros.txt | unexpand -a | cat -A`

A different character can be specified as field delimiters to handle other types of files

 - `cut -d ':' -f 1 /etc/passw | head` - first 10 user names

*paste*

It is the opposite of cut: it adds one or more columns of text to a file. For example if we split the distros text file (sorted by release date) in one containing only the date and one containing the name and version, we can then use paste to create a new file in which the date is the first column

 - `paste distros-dates.txt distros-versions.txt > distros-dates-first.txt`

*join*

The join program acts like a join from relational databases, where data from multiple tables with a shared key is combined to form a desired result. In order to function properly the files must be sorted on the key field. By default, join uses whitespace as the input field delimiter and a single spaece as the output field delimiter (can be changed by options).

For example, if we split the distros.txt (after we sort it by release date) into one file with date and version and a second file with date and name, then we could join the two together simply by calling join:

 - `join distros-key-names.txt distros-key-versions.txt`

*Comparing text*

**`<cmd: comm; diff; patch;>`**

*comm*

The comm program compares two text files and produces an output split into three columns: two for the lines unique in each file and the third for the lines that are common.

 - `comm file1.txt file2.txt`
 - `comm -n file1.txt file2.txt` - n is either 1, 2, or 3 - which column(s) to suppress
	- `comm -n12 file1.txt file2.txt` - show only lines shared by both files

*diff*

The diff program is much more complex, oftern used to examine source code changes; can handle directories (*source trees*). One common use is to create *diff* or *patch* files.

 - `diff file1.txt file2.txt`
	- a description of the differences between the two files; each group of changes is preceded by a *change command* in the form of *range operation range*
		- `r1ar2` - append the lines at position r2 in the second file to the position r1 in the first file
		- `r1cr2` - change/replace the lines at position r1 in the first file with the lines at the position r2 in the second file
		- `r1dr2` - delete the lines in the first file at position r1, which would have appeared at range r2 in the second file
		- a range is a comma-separated list of starting line and ending line
	- this format is the most POSIX compliant and backward compatible, but it is not widely used (more popular: *context format* and the *unifiied format*)
 - `diff -c file1.txt file2.txt` (context format)
	- the output format begins with the two file names plus timestamps and markers for each one; these markers are used in the listing that follow to identify the file which the change refers to (change groups)
	- a change groups starts with the markers for one for the files, then the range of lines from that file; then each line begins with one indicator:
		- blank - a line shown for context, does not indicate a difference
		- `-` - a line deleted (will appear in the first file, but not ine second)
		- `+` - a line added
		- `!` - a line changed; the two version of the line will be displayed, each in its respective section of the change group
 - `diff -u file1.txt file2.txt` (unified format)
	- similar to the context format, but more concise
	- the change groups describe lines in both files, so the duplicate context lines are shown only once
	- a change group starts with a header that shows the lines in each files; then there are the lines themselves, with 3 lines of context by default; each line starts with
		- blank - line shared by both files
		- `-` - line was removed from the first file
		- `+` - line was added to the first file

*patch*

The patch program accepts output from diff and is generally used to convert older version of files to newer ones, by applying the differences described in the diff/patch file.

To prepare a diff file for use with patch, the GNU documentation suggests

 - `diff -Naur old_file new_file > diff_file` (treat new files as empty, treat all files as text, unified format, recursive) - works for single files or directories
 - `patch < diff_file` - we do not need to specify a target file as the diff file already contains the filenames.

Patch has a large number of options and there are additional utility programs that can be used to analyze and edit patches.

*Editing on the fly*

**`<cmd: tr; sed;>`**

*tr*

The tr program *tranliterates* characters - changes them from one alphabet to another. Operates on stdin and outputs to stdout. 

 - `echo "lowercase text" | tr a-z A-Z`
	- the character sets arguments can be
		- enumerated list `ABCDEFGHIJ`
		- character range (subjec to same possible issues with ordering depending on locale)
		- POSIX character classes like
	- the sets can have unequal lenghts
		- `echo "lowercase" | tr [:lower:] A` - will transform all chars in A's
		- `echo "hello" | tr [a-e] [A-E]` - will yield "hEllo"

It also has an option to delete characters from the input.

 - `tr -d '\r' < dos_file.txt > unix_file.txt` - remove carriage returns from file

A nice trick is to use tr for ROT13 encoding of text - move all letters 13 letters up in the alphabet, which is halfway. So the same transformation will give give back the original text.

 - `echo "secret text" | tr a-zA-Z n-za-mN-ZA-M | tee secret.txt | tr a-zA-Z n-za-mN-ZA-M`

tr can also "squeeze" repeating characters

 - `echo "aaabbbccc" | tr -s ab` - will produce "abccc"

*sed*

The *stream editor* program is a powerful program for editing text from standard input or from a set of files. It can receive an editing command or the name of a script file with multiple commands, and it then performs the command on each line of the input.

 - `echo "front" | sed 's/front/back/'` - "back" - substitutes front with back
	- the stream "front" is fed into sed which received a single command as an argument
	- a command starts with a single letter - type of command (s - substition), then the arguments for the command, separated by slash (or any other character)
		- `s_front_back_` - same thing
	- most commands can be preceded by an address - what line(s) are edited
		- `1s/front/back/` - only line number 1
		- `$` - the last line
		- `/regexp/` - line matching a POSIX basic regular expression (to delimit the expression with another character c `\cregexpc`)
		- `addr1,addr2` - range of lines, inclusive
		- `first~step` - from line *first*, every *step* line
		- `addr1,+n` - addr1 and the following n lines
		- `addr!` - all excepts addr

To print out only the first five lines of a file, we use the `p` command with a range address. The `-n` (no auto-print) option is required, otherwise sed will print all lines.

 - `sed -n '1,5p' distros.txt`

To print only some matching lines, we can use a regular expression

 - `sed -n '/SUSE/p' distros.txt`
 - `sed -n '/SUSE/!p' distros.txt` - only lines not matching

Basic editing commands supported by sed (not only `s` and `p`)

 - `=` - output line number
 - `a` - append text after current line
 - `d` - delete current line
 - `i` - insert text in front of the line
 - `p` - print current line (by default sed prints every line and only edits matching ones)
 - `q` - exit sed without processing more lines; without -n, outputs the current line
 - `Q` - exit sed
 - `s/regexp/replacemen/` - replacement may include `&`, which is the text matched by regexp. Also it can include \1 to \9, subexpression in regexp. A further flag may be specified at the end to modify the command's behavior (most common, `g` to perform the substituion globally to a line, not only to the first match)
 - `y/set1/set2` - preform transliteration of characters - sets must have same length (character ranges or classes are not supported)

The `s` substitute command is the most used. For example, we can transform dates in format MM/DD/YYYY to YYYY-MM-DD

 - `sed 's/\([0-9]\{2\}\)\/\([0-9]\{2\}\)\/\([0-9]\{4\}\)$/\3-\1-\2/' distros.txt`  
   *(that is why regular expression are referred to as "write-only" - you can write them, but you can't read them)*

In the example above, the *back-reference* feature of BREs which allows identifying subexpressions.

Multiple commands can be specified:

 - `sed -i 's/white/black/; s/up/down/' foo.txt`
	- `-i[SUFFIX] / --in-place[=SUFFIX]` - edit files in place (makes backup if suffix is given) 

To do more complex transformations with sed, a script can be used. For example, the following sed script will produce a report out of the distros file: a title at the top, changed dates formats and all names in uppercase.

```

    # sed script to produce Linux distribution report
    
    1 i\
    \
    Linux Distribution Report\
    
    s/\([0-9]\{2\}\)\/\([0-9]\{2\}\)\/\([0-9]\{4\}\)$/\3-\1-\2/
    y/abcdefghijklmnopqrstuvwxyz/ABCDEFGHIJKLMNOPQRSTUVWXYZ/

```

 - `sed -f distros.sed distros.txt`

Comments and blank lines are for readability and maintainability. Line-continuation characters ('\' + newline) can be used to embed multiple lines.

Note: `sed` is usually used for simple one-line tasks. For larger tasks, the most popular tools are `awk` and `perl`.

*aspell*

**`<cmd: aspell;>`**

The aspell tool is an interactive spelling checker (successor of an earlier program, ispell). It is used usually by other programs that require spell checking, but can also be used stand-alone. It can intelligently check various types of text files, like HTML, C/C++ programs, email messages etc.

 - `aspell check textfile.txt` - it will open an interactive menu for correcting misspelled words
 - `aspell -H check index.html` - the HTML filter mode  makes checking usable for HTML files - processes only the non-markup parts plus the contents of ALT tags

For other filter modes and options, see the documentation.

*Resources*

 - GNU coreutils
 - GNU diffutils
 - [sed one liners](http://sed.sourceforge.net/sed1line.txt)
 - [sed big tutorial](http://www.grymoire.com/Unix/Sed.html)
 - [shelldorado](http://www.shelldorado.com/links/index.html#tutorials)

*Other programs*

 - `split` - split a file into pieces
 - `csplit` - split based on context
 - `sdiff` - side-by-side merge of file differences

### Formatting output

*Simple formatting tools* mostly one single job and are usually used in pipeline or scripts: nl, fold, fmt, pr, printf.

**`<cmd: nl; fold; fmt; pr; printf;>`**

*nl - number lines*

In its simplest use it just numbers lines, like `cat -n`. Like cat, it can accept multiple files or stdin (`-`).

 - `nl distros.txt`

However, nl supports a form of markup and some options that allow more complex numbering - "logical pages" with header, body and footer; all of these can start the numbering at different values and have different format styles.

 - `\:\:\:` - start of logical page header
 - `\:\:` - start of logical page body
 - `\:` - start of logical page footer

A markup element must appear alone on its own line; after nl processes it, it deletes it from the stream.

Options

 - `-b / -f / -h style` - body / footer / head numbering style: `a` number all lines, `t` only non-blank lines, `n` none, `p<regexp>` only lines matching
 - `-i number` - page numbering increment
 - `-n format` - left justified (1n), right justified (rn), right justified with leading zeros (rz)
 - `-p` - do not reset page numbering at the beginning of each logical page
 - `-s string` - add string after each line number; by default a tab
 - `-v number` - first line number of each logical page
 - `-w width` - width of line number field

To further the work with the Linux distributions report we can use a sed script to add header, body and footer markups and then call nl to number the relevant lines.

```

    1 i\
    \\:\\:\\:\
    \
    Linux Distributions Report\
    \
    Name	Ver. Released\
    ----	---- --------\
    \\:\\:
    s/\([0-9]\{2\}\)\/\([0-9]\{2\}\)\/\([0-9]\{4\}\)$/\3-\1-\2/
    $ a\
    \\:\
    \
    End Of Report

```

Now we can produce an enhanced report

 - `sort -k 1,1 -k 2n distros.txt | sed -f distros-nl.sed | nl` - or `nl -w 3 -n rz -s ' - '`

*fold - wrap lines*

Folding is the process of breaking lines of text at a specified width (default 80).

 - `echo "The quick brown fox jumbed over the lazy dog" | fold -w 12` - it does not take into account work boundaries
	- `-s` - preserve word boundaries (break at last available space before the width)

*fmt - simple text formatter*

The fmt program also folds text, but it also does paragraph formatting. Take as an example a part of the fmt man - fmt-info.txt.

 - `format -w 50 fmt-info.txt` - will give out a bit awkaward result, since the indentation of lines are preserved by default, even if broken up
 - `format -cw 50 fmt-info.txt` - will make the result look ok
	- `-c` - *crown margin* mode - preserves the indentation of the first two lines; following lines will have the indentation of the second line
	- `-p string` - format only the lines that start with string and prepend the string to newly formatted lines; useful for formatting source code comments
	- `-s` - split only mode; shorter lines will not be joined (can be useful for formatting code)
	- `-u` - perform uniform spacing: one space between words, two between sentences; can be useful for removing padding from justification
	- `-w width` - defautl 75; fmt actually formats lines slighty shorter to allow for line balancing.
 - `format -w 50 -p '# ' some-code.txt` - will only format lines starging with '# ', comments in many programming languages

*pr - format text for printing*

The pr program is used to paginate text. When printing text is is often desirable to separate the pages of output with several lines of whitespace, to provide a margin or place for a header and footer.

 - `pr -l 15 -w 65 distros.txt`
	- `-l number` - page length (number of lines in a page)
	- `-w number` - page width (number of columns in a page)
	- it will add some extra lines and a default header - all can be adjusted via options

*printf - format and print data*

The printf program does not accept standard input; it is not often used on the command line, mostly in scripts. It comes from C and is implemented in many programming languages, including in the shell its-self as a builtin.

 - `printf "format" arguments...`
 - `printf "string: %s\n" foo` - the sequences starting with `%` are called *conversion specifiers*
	- `d` - format a number as a signed decimal integer
	- `f` - format and output a floating-point number
	- `o` - octal number
	- `s` - string
	- `x` - hex number, lowercase
	- `X` - hex number, uppercase
	- `%` - the literal `%`
 - `printf "%d, %f, %o, %x, %X, %s" 380 380 380 380 380 380`

A complete converstion specifier has the form

 - `%[flags][width][.precision]conversion_specifier`
	- flags:
		- `#` - use the "alternate output" for output - varies by datatype; for hex it means prefixed with 0x or 0X
		- `0` - pad with leading zeros
		- `-` - left-align the output (default is right-align)
		- ` ` - add a leading space for positive numbers
		- `+` - sign positive numbers
	- width - number specifying the minimum field width
	- .precision - for floating-point numbers, the number for decimal; for sting, the number of characters of output

*Document formatting systems*

UNIX was used in scientific and academic centers, so it made sense to offer tools that could be used to produce many types of documents, especially academic.

The developers working on UNIX justified requesting a new PDP-11 by implementing a document formatting system for AT&T patents division. The program was a reimplementation of `roff`. This was year 1971.

Two main families dominate the field: those descended from `roff` (including `nroff` and `troff`) and those base on Donald Knuth's TeX ("tek") typesetting system.

The `nroff` program is used to format documents for output to devices that use monospace fonts (character terminals, typewriter-stype printers). The program `troff` formats documents for output on *typesetters*, devices used to produce camera-ready type for commercial printing. Most computer printers today are like typesetters. The roff family of programs include `eqn` (for equations) and `tbl` (for tables).

The TeX system appeared later, 1989, and displaced troff at the tool of choice. It is complex and not installed by default. (tip: texlive package, LyX graphical content editor).

**`<cmd: groff; ps2pdf; tbl;>`**

`groff` is a suite of programs containing the GNU implementation of troff. It can also emulate the rest of the roff family.

The way roff-like programs work is to format documents using a markup language. The modern analog for this would be the web page, HTML. The details are not worth going into, but groff offers *macro packages* that expose a smaller set of high-level commands. One macro packae still in wide use is the one for man pages documentation.

Man documentation is stored in `/usr/share/man` as gzip compressed text files. The man pages shown by the man program are rendered with groff with the mandoc package.

 - `zcat /usr/share/man/man1/ls.1.gz`
 - `zcat /usr/share/man/man1/ls.1.gz | groff -mandoc -T ascii`
	- the default output for groff is PostScript (-T ascii changes that)
	- `groff -mandoc > ~/Desktop/ls.ps` will produce a file that can be opened by a graphical page viewer
	- `ps2pdf ls.ps ls.pdf` - PostScript can be easily be converted to PDF
		- `ls /usr/bin/*[[:alpha:]]2[[:alpha:]]*` - conversion programs; also "formattoformat"

*PostScript is a page description language that is used to describe the contents of a printed page to a typesetter-like device. ps2pdf is part of the ghostscript package, the most Linux systems that support printing have installed.*

To form a table for typesetting we can use `tbl`. Using sed we can add markup *requests* to distros.txt.

```

    # sed script to produce Linux distributions report
       
    1 i\
    .TS\
    center box;\
    cb s s\
    cb cb cb\
    l n c.\
    Linux Distributions Report\
    =\
    Name	Version	Released\
    _
    s/\([0-9]\{2\}\)\/\([0-9]\{2\}\)\/\([0-9]\{4\}\)$/\3-\1-\2/
    $ a\
    .TE

```

 - `sort -k 1,1 -k 2n distros.txt | sed -f distros-tbl.sed | groff -t -T ascii`
	- `-t` - process the stream with tbl
 - without -T, groff will produce a PostScript document that looks better when opened with a graphical viewer

*Resources*

 - [groff user guide](http://www.gnu.org/software/groff/manual/)
 - [writing papers with nroff using -me](http://docs.freebsd.org/44doc/usd/19.memacros/paper.pdf)
 - [tbl program](http://plan9.bell-labs.com/10thEdMan/tbl.pdf)
 - "drawing with pic"

### Printing

We will look at command line tools to print files and control printer operation. Configuring and seting up a printer is out of scope, as it varies by configuration to configuration.

*A brief history of printing*

As mainframe computers where shared, users connecting to them via terminals, so where printers, which were attached to those large computers. To identify print jobs from a particular user, a *banner* page was oftern printed at the beginning of each print job. Then the computer support staff would deliver the prints to the people.

First printes where *impact printers*, mechanically forming impressions on the paper. Most common types where *daisy-wheel* and *dot-matrix*. The font was monospace; printing was done at fixed positions on the page and the printable area was fixed. Most printers did 10 CPI, charactes per inch, and 6 LPI (lines per inch). That means that the US-letter sheet of paper (format popular in US, a bit different than A4) is 85 characters by 66 lines. Taking into account a small margin on each syde, you get 80 characters. So using a terminal with 80 chars width you get a WYSIWYG editor.

Data is sent to a typewriter-like printer in a stream of bytes - ASCII codes for characters and for control characters. Some limited effects can be achieved: like using backspace (Ctrl-h, ^H) to go back, print the char again, for a more bolded impression.

The development of GUIs led to major changes in printer technology, which moved from character-based to graphical techniques. This was made possible by the laser printer, which could print tiny dots anywher in the printable area (proportional fonts like in typesetters, diagrams, pictures). The amount of information to describe a page for a 300DPI (dots per inch) printer was almost 20 times higher than for a character-based printer. A new way was invented to descibe the information: PDL (Page Description Language). The first major one was PostScript. The interpretation of PostScript program was embeded in the printers, which accepted as input a PostScript program - page descriptions. The printer had a processor and memory for executing the PostScript interpreter that produced the large bit pattern (dots) for a page (*bitmap*) - this process is called *raster image processor*.

With time, the RIP moved from printers to host computers. Many printers still accept character-based streams, but many do not. They except the stream of bits to print as dots. Some printers stil are PostScript printers.

*Printing with Linux*

Two suites are involved

 - CUPS (Common Unix Printing System) - print drivers and print-job management (using print queues)
 - Ghostscript - PostScript interpreter (acts as a RIP)

**`<cmd: pr; lpr; lpstat; lp; a2ps;>`**

pr is used to adjust text to fit on a specific page size, with optional page headers and margins

 - `ls /usr/bin | pr -3 -w 65` - format out on a 65 wide page and organize the content on 3 columns
	- `+first[:last]` - limit output to a range of pages
	- `-columns_number`
	- `-a` - for mutlicolumn, list output horizontally (across)
	- `-d` - double space output
	- `-D "format"` - format of the date in th eheader (man date)
	- `-f` - use form feeds to separate pages
	- `-h "header_text"` - use this text in the central position of the header
	- `-l lenght` - page length (default 66)
	- `-n` - number lines
	- `-o offset` - left margin ofsset, in characters
	- `-t` - skip header and footer
	- `-w width` - page width (default 72)

To send a job to a printer, one method is `lpr` (Berkeley), the other `lp` (SysV). They do roughly the same thing.

 - `ls /usr/bin/ | pr -3 | lpr`
 - `lpr -P printer file` - you can specify a printer
	- `-# number` - no. of copies
	- `-p` - print each page with a shaded header
	- `-P printer`
	- `-r` - delete after printing
 - `lpstat -a` - list of printers known by the system

System V Style is `lp`. It has a bit more sophisticaded option set

 - `ls /usr/bin | pr -4 -w 90 -l 88 | lp -o page-left=36 -o cpi=12 -o lpi=8`
	- `-d printer`
	- `-n num_copies`
	- `-o landscape`
	- `-o fitplot` - scale the file to fit the page
	- `-o scaling=number` - scale file; 100 is full page
	- `-o cpi=number`
	- `-o lpi=number`
	- `-o page-bottom=points | page-left | page-right | page-top` - 72 points in an inch
	- `-P pages` - list of pages, comma separated, ranges allowed

The program `a2ps` is now *anything to PostScript*. It sends its default output to the system's default printer, not to stdout. It improves the appearence of output ("pretty printer").

 - `ls /usr/bin | pr -3 -t | a2ps -o ~/Dekstop/ls2.ps -L 66`
	- `--center-title=text` - set center page title to text
	- `--columns=number` - default is 2; arrange pages into columns
	- `--footer=text` - text in page footer
	- `--guess` - report the types of files given as arguments
	- `--left-footer=text`
	- `--left-title=text`
	- `--line-numbers=interval` - number lines of output every interval lines
	- `--list=defaults` - display default settings
	- `--pages=range` - print pages in range
	- `--right-footer=text`
	- `--right-title=text`
	- `--rows=number` -arrage pages into number rows, defaul is 1
	- ... font size, line number, characters per line, tab width, landscape, portrait, watermark

There is another similar program *enscript* that converts text to PostScript, but accepts only text input.

*Monitoring and Controlling Print Jobs*

CUPS is designe to handle multiple print jobs from multiple users. Each printer is given a *print queue* where jobs are waiting to be *spooled* to the printer.

*lpstat - display print system status*

The use is to determine the names an availability of printers on the system.

 - `lpstat -a [printer]` - printer queues status
	- `-s` - more detailed description, including device for the printer
	- `-r` - status of print server
	- `-d` - name of default printer
	- `-t` - complete status report

*lpq, lprm, cancel - display printer queue status and cancel jobs*

**`<cmd: lpq; lprm; cancel;>`**

The lpq program shows the status and contents of the printer queues (print jobs). To cancel print jobs and remove them from queues the programs to use are `lprm` (Berkeley style) and `cancel` (System V). 

 - `ls *.txt | pr -3 | lp`
 - `lpq`
 - `cancel 603`
 - `lpq`

*Resources*

 - wikipedia: PostScript, CUPS, Berkeley printing system, System V printing system

### Compiling programs

Compiling programs from source is a big part of Unix and Linux way of thinking. However, now distribution maintainers keep huge repositories of precompiler binaries. Nevertheless, compiling from source still has reasons: availability (not all distributions package all programs) and timeliness (most distributions do not compile the laster version).

Compiling means translating *source code* into *machine language* (numeric code - instructions expressed in binary). The next level over machine code is assembly language - character mnemonics for the CPU instructions. The assembler transforms assembly language into machine language. Above this come the *high-level programming languages* - hide the details of what the process is doing: FORTRAN, COBOL, C, C++ etc. The *compiler* turns this higher level source code into machine code. Some compilers translate into assembly language and then use an assembler.

*Linking* is also an important part. Libraries provide support for common tasks. Usual folders: `/lib`, `/usr/lib` - the linker program forms connections between the output of the compiler and the libraries required by that program. The final result is an *executable program file*.

Some programs don't require compiling. They are interpreted (Perl, Python, PHP, Ruby, shell etc). They are usually slower, but are easier to develop.

*Compiling a C program from source*

Download souce code archive for `diction` program - a spell and style checker.

 - `mkdir src; cd src`
 - `ftp ftp.gnu.org`
	- anonymous
	- cd gnu/dictions
	- get diction-1.11-tar-gz
	- bye
 - wget https://ftp.gnu.org/gnu/diction-1.1.tar.gz
 - tar xzf diction-1.1.tar.gz

GNU Project programs contain files like README, INSTALL, NEWS, COPYING. Before building, README and INSTALL should be read.

Header files installed with the compiler are usually put in `/usr/include/` (stdion.h, unistd.h, stdlib.h etc).

Building the program is usually a simple two step process

 - `./configure` - shell script that analyzes the build environment (adjustments to source code for different Unix types; check for external tools and components)
 - `make` - the make program run according to the Makefile created previously by the configure script; formed of targets and their dependencies, and instructions how to build the targets. It keeps the targets up-to-date on every run, but only the ones that are required, that have a changed dependency

Installing the program is equally simple, for packages that provide the de-facto make special target

 - `sudo make install` - the system directory for installation is usually `/usr/local/bin` (not writable but regular users)

*Resources*

 - [GNU Make manual](http://www.gnu.org/software/make/manual/html_node/index.html)

## 4. Writing Shell Scripts

### First script - basics

At its simplest, a shell script is a file consisting of a series of commands. The shell is both an interface to the system and a scripting language interpreter.

Basics (formatting, executable permissions, location)

 - first line of a script starts with the *shebang* - `#!`; it tells the kernel the name of the interpreter to use
 - comments start with `#` and can be also at the end of a line
 - must have executable permissions (`chmod 755 script`)
	- usually `755` for scripts that everyone can execute
	- `700` for scripts that only the owner can execute
 - the location of the script must be in PATH to execute it without giving the path to it; note that `~/bin`, if existing, is part of the PATH (it is added at login, or in a login shell `su - $username`)
	- `~/bin` - good location for personal use scripts
	- `/usr/local/bin` - a script that everyone on the system is allowed to use
	- `/usr/local/sbin` - script intended for use by the system administrator (locally supplied software, scripts or compile program should go under `/usr/local`, not `/bin` or `/usr/bin` - these are specifed by the Linux Filesystem Hierarchy Standard to contain files supplied and maintained by the Linux distributor)

Note: to make the shell read a file and execute the commands in it, like they were given from the keyboad: `source` or `.` (shell builtin). One example would be modified `.bashrc` or similar and wanting the modifications in the current session, you can just source it.

Making a script easy to read and modify is very important for making it useful (maintainability)

 - usage of long options (e.g. `ls --all --directory` in a script) can provide improved readability
 - by using indentation and continuation (backslahs-linefeed sequence), long commands can be made very readable
    - this can also be done on the command line, but it is hard to type. Also, tabs can't be used for indentation on the command line, they are used to activate completion

```  

    find p \( -type f -not -perm 0600 -exec chmod '{}' ';' \) or \( -type d -not -perm 0700 -exec chmod 0700 '{}' ';' \)
    
    find p \
        \( \
            -type f \
            -not -perm 0600 \
            -exec chmod 0600 '{}' ';' \
        \) \
        -or \
        \( \
            -type d \
            -not -perm 0700 \
            -exec chmod 0700 '{}' ';' \
        \)

```

Vim for writing scripts

 - `:syntax on` (full version of vim and a shebang in the script; with no shebang: `set syntax=sh`)
 - `:set hlsearch`
 - `:set tabstop` - number of characters occupied by a tab character; default is 8; this allows longer lines to fit easier on the screen and is a common practice
 - `:set autoindent` (to stop indentation press `Ctrl-d`)

Note: these changes can be made permanent by adding them to your `~/.vimrc` file

### Starting a project

Project purpose: build a *good* program. Program purpose: produce reports in HTML about our system (status and statistics).

First stage is to output a minimal HTML document, that can then be opened with a web browser.

 - `sys_info_page > sys_info_page.html`
 - `firefox sys_info_page.html` (or `file://path_to_html` in browser URL)

Multiline echo makes it easier to read and modify. Quoated strings may include newlines. This works both on scripts and on the CLI. On the CLI, when a quoted string is started, the character on the next lines comes from the shell prompt in the `PS2` variable (`>`).

**Variables**

Variables are created by the shell when they are encountered.

 - `title="System Information Report"`
 - `echo "... <title>$title</title> ..."` 
    - *parameter expansion* lets you use the content of the variable

The lax way in which the shell handles variables can lead to problems

 - `echo $foo1` - "foo1" variable was not encountered before (spelling error: it should have been "foo")
    - the shell will create it on the fly and give it the default value of nothing/empty
    - echo will just print a newline, no warning

Naming conventions

 - alphanumeric characters and underscore
 - first character must be a letter or and underscore
 - no spaces or punctuation marks

additionally, variables that are intended to remain constant (like the value of PI), are often given uppercase names, while varibales whose content is intended to be changed use lowercase. Note that the shell does provide a way to enforce the immutability of constants, by using the builtin `declare`

 - `declare -r TITLE="Page Title for $HOSTNAME`
    - `-r` - read-only; shell would prevent subsequent assignments; rarely used, only in very formal scripts
    - $HOSTNAME is a shell variable with the network name of the machine

All values assigned to variables are treated as strings (`declare -i` can restrict the assignment to integers, but it is rarely done). No space between `variable=value`.

 - `a=z`
 - `b="a string"    # Quotes required for the space`
 - `c= a and $b"    # Can contain other expansions`
 - `d=$(ls -l foo.txt)    # Output of a command`
 - `e=$((5 * 7))    # Arithmetic expansion
 - `f="\t\a string\n"     # Escape sequences
 - `a=3 b="a string"`     $ Multiple assignment on same line

During expansion, variable name may be surrounded by curly braces: `mv "$filename" "${filename}1"`.

A good practice is to enclose variables and command substitutions in double quotes, to limit the effect of word-splitting by the shell.

**Here Documents**

Here documents is another way of writing multiline strings. The usual marker used is `_EOF_`. In here documents, quotes can be used freely. 

```

    command << token
    text
    token

```

In order to ignore a leading tab character (not space) we can usa `<<-`, instead of `<<`; this does not skip spaces, which may be a problem if tabs are expanded.

*Resources*

 - [HTML tutorial](http://html.net/tutorials/html/)
 - man bash page - HERE DOCUMENTS

### Top-Down Design

Identifying the top-leve steps and developing increasingly detailed views of those steps is *top-down design*.

**Shell Functions**

Commands called in a script can be commands that are available in the CLI or can be small mini-scripts in our program - shell functions.

Syntactic forms:

```

    function name {
        commands
        return
    }

    # Or simpler and genrally preferred form

    name () {
        commands
        return
    }

```

For function calls to not be interpreted as external commands, the shell functions must appear before they are called in the script. 

The `return` command is optional at the end; it terminates the function and returns control to the line with the function call. A functions must have at least one command.

Function names follow the same rules as variable names.

**Local variables**

Local variables are only accesible within the shell function. Names can already exist, globally or in other functions - there are no conflicts.

 - `local foo`

Note that shell functions make great replacements for aliases. A shell function can go in `.bashrc` and then it can executed easily from the CLI.

### Flow Control - Branching with if

Some scripts should behave differently, for example in order to adapt to the privileges of the user running the script. We need the proram to *branch*.

```

    x=5
    if [ "$x" -eq 5 ]; then
        echo "x equals 5"
    else
        echo "x does not equal 5"
    fi

    # General form
    if commands; then
        commands
    [elif commands; then
        commands...]
    [else
        commands]
    fi

```

What is evaluated is the *exit status* of a command, that is the condition for the if test. The range is 0 - 255, where only 0 indicates success. The shell provides a parameter to see the exit status of the previous command: `$?`

 - `ls -d "/usr/bin"`
    - `echo $?` - should be 0 in this case
 - `ls -d "/bin/usr"`
    - `echo $?` - should be != 0
 
Two simple shell builtin commands simply terminate with either 0 or 1: `true` and `false`.

If a list of commands follows `if`, then the last command is evaluate

 - `if true; false; then echo "True"; fi` - will output nothing (false)

A script can be terminated at any time with the `exit` command. Can take an optional argument for the exit status value; if no argument is given it uses the exit status of the last command executed.

Similarly, a shell function can return a value via the `return` command.

**test**

The most common command used in the if clause is the `test` command

 - `test expression`
 - `[ expression ]` - the more popular form

The test command returns an exit status of 0 if the expression is true, or 1 if the expression is false.

*Note: both `test` and `[` are actually commands. In bash they are shell builtins, but they also exist as programs in `/usr/bin` for use with other shells. For `[`, the expression is actually just its arguments, and the `]` needs to be the last argument.*

Supported expressions and tests:

 - file expressions
    - `file1 -ef file2` - same inode number, they are hardlinks
    - `file1 -nt file2` - newer than
    - `file1 -ot file2` - older than
    - `-b file` - exists and is a block device file
    - `-c file` - character device fie
    - `-d file` - directory
    - `-e file` - file exists
    - `-f file` - regular file
    - `-g file` - set-group-ID
    - `-G file` - is owned by the effective group ID
    - `-k file` - sticky bit
    - `-L file` - symlink
    - `-O file` - is owned by the effective user ID
    - `-p file` - named pipe
    - `-r file` - read permissions of the effective user
    - `-s file` - length greater than zero
    - `-S file` - socket
    - `-t fd`   - fd iss a file descriptor directed to/from the terminal (can be used to see if stdin/out/err are being redirected)
    - `-u file` - setuid
    - `-w file` - writable for effective user
    - `-x file` - executable for effective user

The proper/safe way to specify a file path is by quoting it: `if [ -f "FILE" ]; then echo "it is a file"; fi`

 - string expressions
    - `string` - string is not null
    - `-n string` - len greater than zero
    - `-z string` - len is zero
    - `string1 = string2`, `string1 == string2` - strings are equal. The == is supported by bash and generally preffered, but not POSIX compliant
    - `string1 != string2` - different strings
    - `string1 > string2` - string1 sorts after string2
    - `string1 < string2` 

WARNING: the `<, >` operators must quoted or escaped with backslash, in order to not interpret them. Also, until 4.1, bash sorted in ASCII order, not conforming to the collation of the current locale.

 - integer expressions (to compare values as integers, not as strings)
    - `int1 -eq int2`
    - `int1 -ne int2`
    - `int1 -le int2`
    - `int1 -lt int2`
    - `int1 -ge int2`
    - `int1 -gt int2`

Example: `if [ $((INT % 2)) -eq 0 ]; then echo "INT is even"; fi`

**A more modern version of test**

Modern versions of bash add a compound command that does everything that `test` does

 - `[[ expression ]]`
 - `type [[` - "is a shell keyword"

It adds an important new string expression:

 - `string1 =~ regex` - true if string1 is matched by the extended regular expression

This opens up a lot of possibilities for data validation. For example, to check if a variable contains an integer before using it in an integer expression.

 - `if [[ "$INT" =~ ^-?[0-9]+$ ]]; then if [ "$INT" -eq 0 ]; then echo "INT is zero"; fi; fi`

Another feature of `[[ ]]` is that the `==` operator supports pattern matching like pathname expansion does, which makes it useful for evaluating file and pathnames.

 - `FILE=foo.bar; if [[ "$FILE" == foo.* ]]; then echo "matches 'foo.*'"; fi`

**(( )) for integers**

Bash provides the `(( ))` compound command for operating on integers. It performs an *arithmetic truth test* - true if the evaluation is non-zero.

 - `if ((INT == 0)); then echo "INT is zero"; fi`
 - `if ((INT % 2 == 0)); then "INT is even"; fi`

Because it is part of the shell syntax, it is able to recognize variables by name, it does not require parameter expansion to be performed.

**Combining expressions**

Similar to tests for the `find` commands, expressions can be combined with AND, OR, NOT. `test` and `[[ ]] / (( ))` use different operators:

 - AND, OR, NOT: `-a -o !` / `&& || !`

Example:

 - `if [[ "$INT" -ge 10 && "$INT" -lt 20 ]]; then echo "in range"; fi`
 - `if [ ! \( "$INT" -ge 10 -a "$INT" -lt 20 \) ]; then echo "out of range"; fi`

Note: `test` is traditional and part of the POSIX specification for standard shells, which are often used to run system startup scripts. `[[ ]]` is specific to bash and other modern shells - it should be used in writing modern scripts.

Note: "real" UNIX people value portability very much (especially after what happened to UNIX world before POSIX was introduced). That means to stick with the "lowest commond denominator" for shell programming, which for shell programming would mean making everything compatible with `sh`, the original Bourne shell, which  prevents progress. Proprietary software vendors use this pretext to justify their proprietary extensions, which are basically a lock-in. The GNU tools, such as `bash` , don't restrict you; they support standards for portability and are universally available.

**Control operators - another way to branch**

 - `command1 && command2`
    - command1 is executed and, only if it is successful, will command2 be executed
 - `command1 || command2`
    - command2 is executed if and only if command1 is unsuccessful

For example, to exit a script (or a shell session) if a dir does not exist

 - `[ -d temp ] || exit 1`

*Resources*

 - bash man page: "Lists", "Compound Commands", "CONDITIONAL EXPRESSIONS", "SHELL BUILTIN COMMANDS"

### Reading Keyboard Input

**read - read values from stdin**

The `read` command is a shell builtin that read a single line of standard input. It can read data from standard input, or if redirected, from a file.

 - `read [-options] [variable...]`
    - it can handle one or more variables to hold the input value (it assigns fields from stdin to the specified variables)
    - if no variable is given, the shell variable REPLY will be used

Handling multiple values

```

   echo -n "Enter one or more values > "
   read var1 var2 var3 var4 var5

   echo "var1 = '$var1'"
   echo "var2 = '$var2'"
   echo "var3 = '$var3'"
   echo "var4 = '$var4'"
   echo "var5 = '$var5'"

```

If "a b c d e" is given, all variables will have a value. If "a" is given, all variables except var1 will be empty. If "a b c d e f g" is given, var5 will be 'e f g'.

Options

 - `-a array` - assign input to an array, starting with index 0
 - `-d delimiter` - first char is used to indicate the end of input, not a newline character
 - `-e` - use readline to handle input; this permit input editing like on the command line
 - `-i string` - string will be a default reply if user simply presses Enter (requires -e)
 - `-n num` - read num characters, not the entire line
 - `-p prompt` - display an input prompt
 - `-r` - raw mode - do not interpret backslash as escape character
 - `-s` - silent mode - do not echo characters to the display (confidential input)
 - `-t seconds` - timeout; after this, read will return a non-zero exist status
 - `-u fd` - use input from file descriptor fd, rather than stdin

Example to read a "secret", with timeout even

```

    if read -t 10 -sp "Enter secret passhprase > " secret_pass; then
        echo -e "\nSecret passphrase = '$secret_pass'"
    else
        echo -e "\nInput timed out" > &2
        exit 1
    fi

```

**IFS**

The shell performs word splitting on the input given to `read`. This is configured by the shell variable IFS (Internal Field Separator). The default value contains a space, a tab, and a newline character.

To separate fields from a file like `/etc/passwd`, IFS can be changed to a single `:`, then `read` can be used.

```

    read -p "Enter a username > " user_name

    file_info="$(grep "^$user_name:" $FILE)"

    if [ -n "$file_info" ]; then
        IFS=":" read user pw uid gid name home shell <<< "$file_info"
        echo $user ...
    else
        echo "No such user" >&2
        exit 1
    fi

```

The shell allows for one or more variable assignments right before a command. The environemnt will be temporary changed, but only for the execution of that command.

The `<<<` indicates a **here string** - like a here document, but consisting of a single line. The string will be fed into standard input of the command. Why don't use the standard `echo string | read ...`? Because the shell handles pipelines by creating subshells; a subshell has its own copy of the environment, but can not alter the environemnt of the parent. The assignments to variables done by `read` will be lost when it finishes and the subshell terminates.

**Validating input**

Guarding against bad input is very important in scripts shared by multiple users, or even script that are only for us but that do dangerous operations.

Example program

```

    #!/bin/bash

    # read-validate: validate input

    invalid_input () {
        echo "Invalid input '$REPLY'" >&2
        exit 1
    }
    read -p "Enter a single item > "

    # input is empty (invalid)
    [[ -z "$REPLY" ]] && invalid_input

    # input is multiple items (invalid)
    (( "$(echo "$REPLY" | wc -w)" > 1 )) && invalid_input

    # is input a valid filename?
    if [[ "$REPLY" =~ ^[-[:alnum:]\._]+$ ]]; then
        echo "'$REPLY' is a valid filename."
        if [[ -e "$REPLY" ]]; then
            echo "And file '$REPLY' exists."
        else
            echo "However, file '$REPLY' does not exist."
        fi

        # is input a floating point number?
        if [[ "$REPLY" =~ ^-?[[:digit:]]*\.[[:digit:]]+$ ]]; then
            echo "'$REPLY' is a floating point number."
        else
            echo "'$REPLY' is not a floating point number."
        fi

        # is input an integer?
        if [[ "$REPLY" =~ ^-?[[:digit:]]+$ ]]; then
            echo "'$REPLY' is an integer."
        else
            echo "'$REPLY' is not an integer."
        fi
    else
        echo "The string '$REPLY' is not a valid filename."
    fi
    
```

**Menus**

A common type of interactivity is *menu-driven*. The user is presented a list of numbered choices and is asked to pick one. Presenting the menu can be a simple print, while validating the input can help in making sure the choice is valid.

*Resources*

 - [Bash Reference Manual - builtins](http://www.gnu.org/software/bash/manual/bashref.html#Bash-Builtins)

### Flow Control: while / until

**while loop**

 - `count=1; while [[ "$count" -le 5 ]]; do echo "$count"; count=$((count+1)); done`

Similar to if, while evaluates the exist status of a list of commands, and if the status is 0 (true), it executes the command between `do` and `done`. Keywords `continue` and `break` can be used to control program flow.

An *endless loop* can be created using the `true` command as a test, and a `break` if a condition happens to end the endless loop.

**until loop**

 - `count=1; until [[ "$count" -gt 5 ]]; do echo "$count$; count=$((count+1)); done`

The until loop is like the while loop, only that it executes the commands until the condition's exit status is 0. The decision between while and until is usually such that the test is easier to write.

**Reading files with loops**

while and until can process standard input. With redirection, they can be used to read files. The redirection operator is placed after the `done` statement. The `read` command will return an exit status different from 0 when there are no more lines to be read.

```

    while read distro version release; do
        printf "Distro: %s\tVersion: %s\tReleased: %s\n" \
            "$distro" "$version" "$release"
    done < ../distros.txt

```

It is also possible to pipe standard input into a loop. Note that the pipe will execute the while in a subshell, so any variable created or assigned will be lost when the loop terminates

 - `sort -k 1,1 distros.txt | while read distro version release; do echo "..."; done`

*Resources*

 - [Bash Guide for Beginners - tldp](http://tldp.org/LDP/Bash-Beginners-Guide/html/sect_09_02.html)

### Troubleshooting

Common errors and tricks to track down porblems are good to know once scripts grow complicated.

**Syntatic Errors**

 - missing quotes 
    - forgetting to put the end quotes on a string will make bash search for the first quotes, which may be farther down and so the string created may encompass keywords, commands etc
    - best way to avoid this is using syntex highlighting in the editor (e.g.: for vim `:syntax on`)
 - missing or unexpected tokens
    - fogetting to complete a compound command (like if ... fi, or while ... do ... done)
    - for example, forgetting the semi-colon after a `test / [ ]` command will make the shell add the following words as arguments to the test command, which will in turn make the then part of the if now part of the list of commands for if; so when `else` is seen, it is out of place
 - unanticipated expansion
    - if a script sometimes runs ok, but sometimes it doesn't, it may mean that an expansion is performed that is not as expected
    - for example, expansion of an empty variable, which is used unquoted, will lead to an empty argument to the command or shell builtin, which is surely not what you want
    - similar, expansion into multiword string can be same dangerous

**Logical Errors**

They do not prevent a script from running, but it will not produce the desired result, which is even worse. Some of the most common are

 - incorrect conditional expressions
 - "off by one" errors - loop conters
 - unanticipated situations (like unanticipated input, errors)

*Defensive programming* means validating assumptions

 - `cd "$dir_name" && rm *` - do not remove files if the change directory fails
    - still a problem: if `$dir_name` is empty or undefined, you will remove the files from the user's home directory
    - `[[ -d "$dir_name" ]] && cd "$dir_name" && rm *`

Even better, including code to warn about the error and terminat the program can be very useful

```

    if [[ ! -d "$dir_name" ]]; then
        echo "no such dir '$dir_name'" >&2
        exit 1
    fi

    if ! cd "$dir_name"; then
        echo "cannot cd" >&2
        exit 1
    fi

    if ! rm *; then
        echo "file deletion failed" >&2
        exit 1
    fi

```

*Watching out for filenames* is important, since in Unix world, only two characters are not allowed in a file name: '/' and null character (a zero byte). A name could be "-rf ~", which would reck havo when expanded for a command like `rm *`. 

The solution is to always precede wildcards with a path: 

 - `rm ./*`

Portable filenames (POSIX) include letters, numbers, underscore, hyphen, period. It also suggests not to start with a hyphen.

*Verifying input* is a general programming rule. Best way is to make sure the input is from an allowed range.

**Testing**

In open-source world, the saying *release early, release often* is common. The earlier your program is exposed to usage and testing, the better; goal is to find problems early in the development cycle.

For example, placing an `echo` before a command, you can see what it will look like.

Useful testing requires good test cases (input data, operating conditions that reflect normal and edge/corner cases).

**Debugging**

If a problem is revealed, you need to see what the script is doing and why. A well-written script, written defensively and with feedback will help, but sometimes it is not enough.

 - commenting out sections of the script can lead to discovering the problem area
 - informative messages can be placed in the script for a sort of tracing
    - `bash` provides a method of tracing by the `-x` option to the shebang or `set -x` in the script (`set +x` will turn it off)
        - messages will be displayed with a leading `+` sign (the default value of PS4, which is the prompt for tracing)
        - to include the line number also: `export PS4='$LINENO + '`
 - extra messages to display the content of variables

For really heavy-duty debugger, opensource projects like "Bash Debugger" exist.

*Note: design (and programming) is a funciton of time. Given little time, you design (code) something quick and dirty. Given abundent time, you design (code) something complicated.*

*Resources*

 - [tldp gotchas](http://tldp.org/LDP/abs/html/gotchas.html)
 - [bash pitfalls](http://mywiki.wooledge.org/BashPitfalls)
 - [bash debugger](http://bashdb.sourceforge.net/)

### Flow Control: branching with case

Multiple-choice compound command is `case`

```

    case word in
        [pattern [| pattern]...) commands;;]...
    esac

```

The patterns used are like the ones for pathname expansion. They are terminated with a ")" character. Example of patterns

 - `a)` - word "a"
 - `[[:alpha:]])` - a single alphabetic character
 - `???)` - three characters long
 - `*.txt)` - word ending with ".txt"
 - `*)` - any value of word; as last pattern, in will catch all that has not been dealt with

Example

```

    read -p "enter word > "

    case "$REPLY" in
        [[:alpha:]])    echo "...." ;;
        [ABC][0-9])     echo "...." ;; 
        ???)            echo "...." ;;
        *.txt)          echo "...." ;;
        *)              echo "...." ;;
    esac

```

Patterns can be combined

 - `a|A)` - lower or uppercase "a"

In bash prior to 4.0, case executed a single action and then the command terminated. Modern versions allow case to continue to the next test and see if it matches

 - `;;&` - this will make case continue to the next test and check if the pattern for it matches

*Resources*

 - [Bash Reference Manual](http://tiswww.case.edu/php/chet/bash/bashref.html#SEC21)
 - [tldp bash](http://tldp.org/LDP/abs/html/testbranch.html)


### Positional parameters

The shell provides a set of variables ($0 - $9) that contain the words on the command line.

 - `echo $0` - will always be non-empty, containing the pathname of the script executed
 - more parameters can be accessed using parameter expansion: `${10}, ${211} ..`

The `$#` shell variable will give you the number of arguments on the command line.

In order to access a great number of arguments, the shell provides a method to move all the parameters down by one (except for $0, which never changes).

 - `count=1; while [[ $# -gt 0 ]]; do echo "Argument $count = $1"; count=$((count + 1)); shift; done`

Note: `basename` command removes the leading portion of the path. With it, the script can be renamed easily and the script will adapt.

Positional parameters can be used exactly the same with *shell functions*. The shell has a variable "$FUNCNAME" that contains the name of the current executing shell function. However, the variable "$0" will always contain the full pathname of the first name of the command line, not the name of the function.

It might be useful to handle the complete list of positional parameters as a group, for example to create a wrapper around another program.

 - `$*` - expands into the list of positional parameters, starting with 1. When in double quotes, it expands into a double quoted string with each parameter separated by the first character of the IFS
 - `$@` - similar, but when in double quotes, it expands each positional parameter into a separate word as if it was surrounded by double quotes

Example

 - "word" "word with spaces"
    - `$*`   - 4 args: word word with spaces
    - `"$*"` - 1 arg : "word word with spaces"
    - `$@`   - 4 args: word word with spaces
    - `"$@"` - 2 args: "word" "word with spaces" -> this is the most common use-case

*Resources*

 - [Bash hackers wiki - positional parameters](https://wiki.bash-hackers.org/scripting/posparams)
 - [Bash reference manual - special parameters](http://www.gnu.org/software/bash/manual/bashref.html#Special-Parameters)
 - [getopts](http://wiki.bash-hackers.org/howto/getopts_tutorial)
 - bash man - SHELL BUILTIN COMMANDS

### Flow Contorl - for

**Traditional shell form**

```

    for variable [in words]; do
        commands
    done

```

 - `for i in A B C D; do echo $i; done`
    - echo will be executed four times, one for each item in the list
 - `for i in {A..D}; do echo $i; done`
    - list created through *brace expansion*
 - `for i in distros*.txt; do echo $i; done`
    - or *pathname expansion*
    - note that if the expansion does not match anything, `distro*.txt` will be the variable
        - `if [[ -e "$i" ]]; then echo "$i"; fi`
 - `for i in $(strings "$1"); do len="$(echo -n "$i" | wc -c)"; echo "$i $len"; done`
    - go through the string literals in a file and print the len
    - generate the for sequence using *command substitution*
        - we do not surround the command substituion with double quotes, as usual, because we want word splitting to occur

If the optional `in words` are omitted, for will process the positional parameters.

Note: the usage of i, j, k comes from Fortran, where I, J, K, L, M variables, if used undeclared are considered integers. The rest are considered decimals.

**C language form**

Recent version of bash include a C-like form. It is useful when a numeric sequence is needed.

```

    for (( expression1; expression2; expression3 )); do
        commands
    done

```

The expressions are arithmetic expressions. The first one initializes conditions of the loop; the second determines when the loop ends; the third is carried one at the end of each iteration.

 - `for (( i=0; i<5; i=i+1 )); do echo "$i"; done`

*Resources*

 - [advanced bash scripting guide](http://tldp.org/LDP/abs/html/loops1.html)
 - [bash reference manual](https://www.gnu.org/software/bash/manual/bashref.html#Looping-Constructs)

### Strings and Numbers

**Parameter Expansion**

Note: parameter expansion should always be enclosed in double quotes to prevent unwanted word splitting (unless, of course, it is actually desired).

A form of parameter expansions was already encountered - shell variables.

*Basic Parameters*

 - `$a`, `${a}` - when epanded, it becomes whatever the variable a contains
    - braces are required if the expansion is adjacent to other text
    - also to access position parameters higher than 9: `${11}`

*Expansions to manager empty variables*

Several parameter expansions are intended to deal with empty and nonexistent variables. Usfeul for handling positional parameters and giving default values to parameters.

 - `${parameter:-word}` - if parameter is unset or empty, the expansion results in word; if not empty, the value of parameter
    - `echo ${foo:-"substitute if unset"}`
 - `${parameter:=word}` - if parameter is unset or empty, the expansion results in word and it is assigned to parameter
    - positional and other special parameters cannot be assigned this way
 - `${parameter:?word}` - if parameter is unset or empty, the script will exit with an error, and the contents of word will be sent to standard error
 - `${parameter:+word}` - if parameter is unset or empty, the expansion results in nothing; if not empty, word is substituted for parameter (the value of parameter is not changed)

*Expansions that return variable names*

The shell can return names of variables. This can be used in some exotic situations

 - `${!prefix*}`
 - `${!prefix@}`

Both forms perform identically - return the names of existing variables with names beginning with *prefix*. E.g.: `echo ${!BASH*}`

*String operations*

 - `${#parameter}` - expands into the length of the string contained by parameter
    - if parameter is `@` or `*`, then it returns the number of positional parameters
 - `${parameter:offset} ${parameter:offset:length}` - extract a portion of the string contained by parameter, starting at offset
    - if offset is negative, it starts from the end of the string (must be preceded by a space to not be confused with substitute if unset expansion
    - if parameter is @, the result is length positional parameters starting at offset
 - `${parameter#pattern} ${parameter##pattern}` - remove a leading portion of the string contained in parameter, defined by pattern (wildcard pattern, like in pathname expansion)
    - the # removes the shortest match, while the ## removes the longest
 - `${parameter%pattern} ${parameter%%pattern}` - same like the previous expansion, but remove text from the end
 - `${parameter/pattern/string} ${parameter//pattern/string} ${parameter/#pattern/string} #{parameter/%pattern/string}`
    - search and replace; first form, only the first occurrence, second all of them; the third required the match to occur at the beginning of the string, the last one, at the end
    - if '/string' is omitted, pattern will just be deleted

These kind of parameter expansions can replace some usage of commands like sed or cut, making script more efficient.

*Case conversion*

`bash` can do case conversion via four parameter expansions and two `declare` command options. *Normalizing* data to a standardized form (all upper or all lower) can be useful for storing it.

`declare` command can be used to force a variable to always contain either upper or lower case, no matter what is assigned to it

 - `declare -u upper`
 - `declare -l lower`

The following parameter expansions can perform case conversions, with some extra capabilities than declar

 - `${parameter,,pattern}` - expand into all lowercase; pattern - optional shell pattern (e.g. [A-F]) that will limit which characters are converted (bash man for full set of patterns)
 - `${parameter,pattern}` - only the first letter to lowercase
 - `${parameter^^pattern}` - all uppercase
 - `${parameter^pattern}` - expand but only change first char to uppercase

**Arithmetic Evaluation and Expansion**

Arithmetic expansion is used to perform operations on integers. Its basic form is `$((expression))`. This is related to the compound command `(( ))` used for arithmetic evaluation (truth tests).

*Number bases*: the shell supports integer constants in any base. The notations are:

 - `number` - decimal; `0number` - octal; `0xnumber` - hex; `base#number` - number in base

*Unary operators* are `+` and `-`, to indicate whether a number is positive or negative.

*Simple arithmetic* operators are `+ - * / ** %`. Shell arithmetics operates only on integers, so results of division will always be a whole number.

*Assignment* is something that can also be done in an arithmetic expression

 - `if (( foo = 5 )); then echo "true"; fi; echo $foo` - foo variable will be assigned 5 and the expression will evaluate to true, because foo is assigned a non-zero value

Other useful assignment notations are: `+=, -=, /=, %=, ++, -- (both post and pre)`

*Bit operators*: `~ << >> & | ^`

 - `for (( i = 0; i < 8; i++ )); do echo $((1 << i)); done` - power of two - 8 bits

*Logic*: the shell supports all the C like comparison operators, including logical. It also supports the ternary operator - only works with arithmetic expressions. Eg: `((a<1?++a:--a))` - a toggle of a between 1 and 0. To do assignments inside the expression, parentheses must be used.

**bc - an arbitrary precision calculator language**

Shell can handle integer arithmetic, but for floating point or higher math, an external program has to be used. Embedding Perl or AWK programs is on possible solution. Another one is to use a specialized calculator program; one that is found on many is `bc`.

The `bc` program reads a file written in its own C-like language and executes it - can read from a file or from standard input. It support variables, loops, functions.

 - `bc -q foo.bc` - where foo.bc is a text file containing the script (-q suppresses the copyright message)
 - `bc -q` - interactive mode; line by line, until "quit" is given
 - `bc < foo.bc` - scripts via standard input
 - `bc <<< "2+2"` - here strings

With bc you can for example create a monthly loan payment calculator, embedding the bc script in a here document. Note: the formula is explained [here](https://en.wikipedia.org/wiki/Amortization_calculator)

*Resources*

 - [bash hackers' wiki](https://wiki.bash-hackers.org/syntax/pe)
 - [bash reference manual](https://www.gnu.org/software/bash/manual/bashref.html#Shell-Parameter-Expansion)

### Arrays

An array is a data structure which holds multiple values, as opposed to a scalar value. Arrays in bash are limited to a single dimension. Support was added in bash version 2. The original sh from Unix did not have it.

Creating an array can be done by assigning a value to an element or by using the `declare` command

 - `a[1]=foo; echo ${a[1]}`
 - `declare -a a`

Assigning values can be done individually or with multiple values

 - `name[subscript]=value` - subscript is an integer, and can be an arithmetic expression greater, greater or equal than zero
 - `name=(value1 value2 ...)` - assignment will start with element zero
    - `days=(sun mon tue wed thu fri sat)`
    - `days=([0]=sun [1]=mon [2]=tue [3]=wed [4]=thu [5]=fri [6]=sat)`

An example of usage is storing data and indexing it by a criteria. For example, a script that examines the modification times of the files in a specified directory, per hour.

*Array operations*

The `*` and `@` can be used to access every element in an array. The same differences applies like for positional parameters, so the most useful form is usually `@`, quoted.

 - `animals=("a dog" "a cat" "a fish")`
 - `for i in "${animals[@]}"; do echo $i; done` - will give the exact word sequences, quoted like the original array data
    - without quotes, both `*` and `@` give the same result - all words will be splitted (a, dog, a, cat ...)

Determining the number of array elements can be done with parameter expansion, like finding the length of a string

 - `a[100]=foo`
 - ` echo ${#a[@]}` - number of array elements
    - this will be 1, not 101!
 - `echo ${#a[100]}` - length of element 100
    - this will be 3, of course

Note that in bash, array elements exist only if they have been assigned a value, no matter the value of the subscript.

This means that an array can have "gaps" in the subscripts. The following parameter expanssions will give the list of subscripts. Same difference betwee `*` and `@` applies, so the quoted `@` form is most useful as it expands into separate words.

 - `${!array[*]}`
 - `${!array[@]}`

To print out all values and all subscripts

 - `foo=([2]=a [4]=b [6]=c)`
 - `for in in "${foo[@]}"; do echo "$i"; done`
 - `for in in "${!foo[@]}"; do echo "$i"; done`

To append element at the end of an array the shell provides an easy solution: `+=` assignment operator

 - `foo=(a b c)`
 - `foo+=(d e f)`
 - `echo ${foo[@]}` - a b c d e f

Sorting an array can be done easily through a little coding - command subsititution plus a secondary array

 - `a=(f e c d b a)`
 - `a_sorter=($(for i in "${a[@]}"; do echo $i; done | sort))`

To delete an array, the `unset` command must be used. It can also be used to detele single array elements.

 - `unset 'foo[2]'` - must be quoted so that the shell does not perform expansion
 - `unset foo`

Note that assigning an empty value to an array variable will not empty its contents. It will only empty element zero.

 - `foo=A` - any reference to an array variable without a subscript refers to element zero of the array

*Associative arrays*

Bash higher than 4.0 supports associative arrays - using strings instead of integers as array indexes.

Associatives arrays must be created with `declare -A`; it is not enough to simply reference them.

 - `declare -A colors`
 - `colors["red"]="#ff0000"`
 - `collors["green"]="#00ff00"`
 - `echo ${colors["blue"]}`

### Exotica

**Group commands and subshells**

`bash` allows grouping of commands together either via a group command or a subshell.

 - `{ command1; command2; [command3; ...] }` - group command
    - the spaces after and before the braces are important; last command must be followed by a semicolon or a newline
 - `(command1; commnad2; [command3; ...])` - subshell

Both are used to manage redirection, including usage of pipes

 - `{ ls -l; echo "Listing of foo.txt"; cat foo.txt; } > output.txt` - redirect output of three commands to a file
 - `(ls -l; echo "..."; cat foo.txt) > output.txt` - similar
 - `{ ls -l; echo "Listing of foo.txt"; cat foo.txt; } | lpr` - the combined output is passed to another program

*Process substitution*

The difference between group commands `{ }` and subshell `( )` is that for the subshell the commands are executed in child copy of the current shell. The environment is copied but if any modifications are done to it, like variable assignment, it will not reflect in the parent shell. This means that if a subshell is not required, group commands will be faster and less expensive, memory-wise.

The environment "problem" is similar to pipe-ing input into `read`. A pipe creates a subshell, so the read command will modify variables in its own environment only. The shell does provide a special form of expansion for this: process substitution.

: - `<(list_of_commands)` - for processes that produce standard output
 - `>(list_of_commands)` - for processes that intake standard input

For the "read problem"

 - `read < <(echo "foo"); echo "$REPLY"`
    - the output of the echo shell is treated as an ordinary file for purposes of redirection
    - `echo <(echo "foo")` - will give the file descriptor of that file

Process substitution is often used with loops containing `read`.

```

    while read ... ; do
        ...
    done < <(list_of_commands)

```

Several substitutions can be used - basically, where a file can be put in a command, a process substitution can be placed instead

 - `comm - 3 <(sort file1 | uniq) <(sort file2 | uniq)`
    - lines unique to each of the two unsorted files

**Traps**

Scripts can respond to signals like any regular process. Large complicate scripts are more likely to be affected by the user logging of, or the system rebooting, or other interruptions. A signal handling routine can come in handy to exit in an orderly fashion (save state, delete temporary files etc).

`bash` provides traps for this

 - `trap argument signal [signal...]`
    - argument is a string that will be treated as a command, triggered by the specified signal(s)
 - `trap "echo 'I am ignoring you.'" SIGINT SIGTERM`

It is common to specify a shell function as argument.

Note that the signal handler output will be directed to stdout (a normal echo in a signal handler will print on stdout, if stdout not redirected to stdin).

*Note - temporary files*: temporary files are usually created in the shared directory `/tmp`. To prevent *temp race attacks*, a non-predictable filename is required. One way is `tempfile=/tmp/$(basenme $0).$$.$RANDOM`. $RANDOM returns a value in 1 - 32767 only. A better way is to use `mktemp` program, which fills a pattern of X characters with random letters and numbers `tempfile=$(mktemp /tmp/foobar.$$.XXXXXXXXXXX`. For scripts executed by regular users, it may be wise to just avoid the `/tmp` directory and create a directory for temporary files in the user's home `[[ -d $HOME/tmp ]] || mkdir $HOME/tmp`.

**Asynchronous execution**

Scripts can be constructed so that multiple tasks are performed at the "same" time (multitasking).

Usually this is done by launching one or more child scripts. However, it can be hard to keep the parent and children coordinated. `bash` has a builtin command to help manage asynchronous execution: `wait` - the parent script pauses until a specified process finishes.

```

    # parent
    async-child &
    pid = $!

    sleep 2

    # parent: pause to wait for child to finish
    wait "$pid"



    # child
    sleep 5
    
```

**Named pipes**

A special type of file, named pipe, can be used to create a connection between two processes. They behave like FIFO buffers, as to the ordinary (unnamed) pipes

 - `mkfifo named_pipe`
    - create a named pipe - first letter in ls: p
 - `process1 > named_pipe; process2 < named_pipe`
    - it will behave like `process1 | process2`
 
The advantage/difference from the regular pipe, is that the communication processes can be unrelated, for example from different terminal windows or login sessions. The first process, the one that sends output into the pipe, will block (*blocked pipe*) until there is somebody reading on the other side.

*Resources*

 - bash man page - "Compound Commands" section
 - bash man page - EXPANSION (contains process substitution)
 - [advanced bash scripting guide](http://tldp.org/LDP/abs/html/process-sub.html)
 - [linux journal - process substitution](https://www.linuxjournal.com/content/shell-process-redirection)
 - [linux journal - named pipes 1](https://www.linuxjournal.com/article/2156)
 - [linux journal - named pipes 2](https://www.linuxjournal.com/content/using-named-pipes-fifos-bash)




