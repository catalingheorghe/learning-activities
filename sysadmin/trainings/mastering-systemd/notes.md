# Mastering Systemd - o'reilly learning training 2020.09.25

Unit types
systemctl -t help

Directories
/usr/lib/systemd/system -> units installed by packages
/etc/systemd/system -> your additions, changes (has higher prio)
/run/systemd/system -> units created on the fly (can be empty)

See a unit
systemctl cat cron
(or vim)

Status of a unit
systemctl status cron.service

Man pages
man -k systemd
man 7 systemd.directives -> an index of all directives
each unit type has a man page systemd.swap
systemd.exec

**Modify a unit**

export SYSTEMD_EDITOR=vim
systemctl edit cron.service

an override file will be created in /etc
systemctl cat will show two files now

what can you modify
systemctl show --all cron.service
(see systemd.directives for details)

**Targets**

### system targets (isolate)

(similar to a runlevel)

systemctl get-default
graphical.target

systemctl cat multi-user.target
 - allowisolate = yes => can be set as a default target, a target state for the
   entire system
grep -i allowisolate=yes * 

systemctl isolate emergency.target
 - moves to that state

systemctl list-units
 - see what units are loaded (in an emergency target, they are much less)

systemctl enable httpd
 - will create a symlink under multi-user.target.wants to the httpd service

systemctl list-dependencies 
### non system targets

bluetooth target is just a group of units for example, not a system target
all the units needed to control bluetooth functionality

a target can be managed with systemctl start/stop

**Journal**

all units output go to journal
(standard commands can be directed there - systemd-cat)

journalctl [-F]

/etc/systemd/journald.conf -> storage - to make it persistent, if there is no
other logging, like syslog

restart systemd daemon
systemctl deamon-reload

journalctl unit=sshd.service
(also shows up in systemctl status sshd.service)

### advanced

Systemd wants to be an entire OS on top of linux kernel. It wants to do anything.
Manages services, hardware init, sessions, mounts, paths, sockets.

systemd-journald
systemd-udevd (hardware initialization)
systemd-logind (session manager)
systemd-networkd (but seems networkmanager has won this)
systemd-nspawn (but docker and podman has won)

loginctl list-sessions | kill-session

**unit dependecies**

systemctl list-dependencies

Tokens in unit files for dependency management 
 - requires
 - requisite
 - wants
 - before
 - after

**own unit files**

anything can run as a systemd service
with or without root

...
[Service]
Type=simple
ExecStart=...
...

man 5 systemd.service

-> /etc/systemd/system
   systemctl daemon-reload

**cgroups**

/sys/bus/devices/cpu0/ -> online file
echo 0 > online => lscpu - will show one less CPU now, top as well

systemd service option: CPUShares

**systemctl --user**

(better than systemctl run)

run things in systemd as user

~/.config/systemd/user/demo.service
(note: requires a [Install] WantedBy=multi-user.target)
systemctl --user enable|start demo.service

this will start the service when the user logs in
if "linger" is set to yes, the demo service will start when the system starts
(option via loginctl)

Note that with this you can start a service in container as a user, at system
startup.

### systemd security

users: in the [Service] section -> User, or even DynamicUser
mount namespaces: protect home, system dirs, own tmp
create temporary directories: config, runtime, logs ...
other (no access to proc or sys, no IPC, no loading of kernel modules, no writes
       to cgroups sys fs, no real-time, can blacklist of specific syscalls, 
	   SE Linux context ...)

**systemd-analyze**

systemd-analize security sshd.service
 - security settings

systemd-analize blame
 - time taken by each unit

### Mount and automounts

standard way: /etc/fstab
from that, generator for systemd info -> systemd units

/run/systemd/generator
 - home.mount
 - (mountpoint).moun

automount also, but autofs might still be better (?)

### Timers

Cron on steroids.

`systemctl status *.timer`

timer comes with a service with the same name

### Sockets

former xinetd

`systemctl status *.socket`

