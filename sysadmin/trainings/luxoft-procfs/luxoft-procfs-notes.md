Daemon usually creates a pidfile. You can use that to see if there is a
directory in /proc to see if it is running
```
    [ -d /proc/$(</tmp/httpd.pid) ] && echo running
```

How a program is running: its env and the command line
 - /proc/environ
 - /proc/cmdline
Hint: use `strings` to read the files to put a newline instead of NUL character
File descriptors under `fd` subdirectory. To see what sockets are pointing to
you can use `ss` tool.

The image of the running process is stored in /proc/[pid]/exe, as a symlink.
Also the `fd` directory has links.

`fdinfo` contains info about each file handler, including the `pos` in the file
You could see where you are when transferring a file, for example.

The 'status' file will show both the voluntary and the non-voluntary context
switches.

Which libraries are loading - `/proc/pid/maps`.
Note that it can show more than `ldd` as there can be runtime plugins.

Each process has an OOM score. You could go through the processes and see which
one have a high score. `/proc/pid/oom_score`.

Every terminal goes to a pseudo-terminal device (/dev/pts/xx). If you write
stuff in there, it will show up in another terminal.

You can mount a /proc remotely: `sshfs raspberrypi:/proc /mnt/procpi`
TODO - some user permission problems? see the files, but not able to read the
contents
UPDATE: I think it is a problem caused by the fact that files have 0 size in
proc as it is a virtual fs. Maybe the option mentioned here [1] helsp:
`-odirect_io`.

[1] https://sourceforge.net/p/fuse/mailman/message/19880265/

SYSRQ key availble via /proc -> /proc/sysrq-trigger. Echo " " into it, it will
show into `dmesg`. Example: `m` - show memory usage. But also reboot, shutdown,
others.

