# Root File System

## 

The root file is the first file system mounted at the top of the file system hierarchy (`/`). Mounting a root file system is not mandatory, but brings great benefits to implementing applications and services (the alternative would be doing everything inside a kernel thread or device driver).

Linux expects from the root file system to contain programs and utilities that help in booting the system (initialize system console and networking, mount other file systems, load device drivers or kernel modules in general).

The **layout** of the root file system on Linux has been standardized to some degree in the FHS (File System Hierarhy Standard). Most Linux distributions keep pretty close to these guidelines.

When space is a concern, a very small root file system is created on a bootable device, and later, a larger file system is mounted from another device or from the network (from an NFS server). A larget file system can also be mounted on top of the original small one. A simple layout for top level directories contains:

```
    bin/
    dev/
    etc/
	home/
	lib/
	sbin/
	usr/
	var/
	tmp/
```

A truly minimal root file system can be something along the lines of

```
   bin/busybox
   bin/sh -> busybox
   dev/console
   etc/init.d/rcS
   lib/ld-2.23.so
   lib/ld-linux.so.2 -> ld.23.so
   lib/libc-2.23.so
   lib/libc.so.6 -> libc-2.23.so
```

Building a root file system for an embedded device needs to take into consideration the size of the storage. It employs elements of "distribution engineering", what companies like Red Hat and Canonical, or Mentor Graphics (embedded) do in order to populate the different directories on the file system that different packages need. Automated tools to build file systems exist: buildroot, bitbake (from Yocto). It can also be done manually by choosing controlling what files are put in the rootfs; feasible when the number of packages is not large.

In the last steps of the **kernel booting up**, it tries to exec a program that is expected to be there in the root file system (the init program). If it does not find it, the kernel will `panic()` and, implicitly, halt. One of the last expected locations is `/bin/sh`, hence the link in the minimal root file system above. The usual first location is `/sbin/init`.

If the init process is dynamically linked, then the root file system must satisfy its dependencies by containing the required libraries. A tool exists that can tell you what these dependencies are: `${CROSS_COMPILE}-ldd`. Or using readelf in the form: `${CROSS_COMPILE}-readelf program | grep NEEDED`. What ldd does is to invoke the dynamic linker/loader in trace mode along the lines: `LD_TRACE_LOADED_OBJECTS=1 LD_WARN= LD_BIND_NOW= LD_LIBRARY_VERSION= LD_VERBOSE= arm-linux-gnueabihf-ld --verify target/brs2s/rootfs/sbin/init`.

The second kind of dependencies that init, or any other program, can expect is composed of condiguration and data files. This must be determined by knowing that program works.

The initial process can be customized by the kernel command-line parameter "init" (e.g.: `init=/sbin/myinit`).

Specifying a custom init process can be considered unusual, as the standard init programs are very flexible. The old-school init, used still in busybox, together with init script form the **System V Init** schema.

Additional child processes are spawned by init according to the configuration file `/etc/inittab`. This file has the concept of "runlevel", which can be thought of as a system state. Each runlevel is defined by the services that are enabled when entering that runlevel. `init` can be in a single runlevel at any given time.

Runlevels "run" from 0 to 6 plus a special one call S. Runlevel 0 means system halt, while runlevel 6 means system reboot. Each runlevel can have a series of startup and shutdown scripts. The other levels commonly have the following usage: 1 - single-user system configuration for maintenance; 2 - user-defined; 3 - general-puropose multiuser configuration; 4 - user-defined; 5 - multiuser with GUI on startup.

Scripts that enable or disable services are found under `/etc/rc.d/init.d` (or similar). The scripts can also be run manually with arguments like `start`, `stop`, `restart`. A runlevel is defined by a directory structure of symbolic links to these scripts. This structure of directories is usually under `/etc/rc.d` in the form `rcN.d/`. The directories an scripts to run are known to init because inittab mentions a script that goes in the directories and runs them.

The first thing to be run is marked with "sysinit" in inittab. More info about the syntax can be found in the man page of inittab.

The Linux kernel contains two mechanisms to **mount an early root file system** for system init and configuration. The legacy one is `initrd` (initial RAM disk), the newer one is `initramfs`.

The **initial RAM disk** is a small root file system that is usually used to load specific device drivers before the completion of the boot cycle. For example, in Red Hat and Ubuntu, and initrd is used to load device drivers for EXT3 before mounting the real root file system.

The boot loader is usually involved to boot using and initrd. It loads a compressed kernel image into memory and then loads and initrd into another section of memory. The bootloader passes the address of the initrd to the kernel before passing control to it. The kernel when it starts, copies the initrd from raw memory to a proper ram disk (`/dev/ram0`) and frees up the initial memory. The magic of initrd is in a script `linuxrc`, which, if present, is executed by the kernel (for example to load drivers) and then continues booting. If the real root file system has a directory `/initrd`, the kernel mounts the initrd image there. If the kernel command line specfies the root file system to be a ramdisk (`root=/dev/ram0`, for example), then there will be no attempt to mount a second file system as root, so initrd can be the only root file system.

Example for kernel command line and config:

 - cmdline
   - `console=ttyS0,115200 root=/dev/ram0 rw rdinit=/sbin/init quiet`
 - config
   - `CONFIG_BLK_DEV_INITRD=y`
   - `CONFIG_INITRAMFS_SOURCE=""`
   - `CONFIG_RD_LZO=y`

An initrd image must be a proper root file system, and in addition, it must be small. For example, if it only has busybox as program, that can be statically linked, so the C library and dynamic loader are not needed. The C library is usually quite large, so omitting it can save a lot of space.

The init RAM disk mechanism is also described in man initrd(4).

**initramfs** is similar to initrd; it also has the similar purpose of enabling the loading of drivers that might be required for mounting the real root file system. It is enabled with the same configuration options. There are some differences that make it the option that preferred over initrd.

It is called earlier in the kernel boot up sequence (see `filesystems/ramfs-rootfs-initramfs.txt` kernel doc). It is a cpio archive, not a gzipped file system image (usually what initrd is); you can create it without being root. It is integrated into the kernel source tree and a small default empty image is built automatically (`usr/` directory in kernel build location - files are selected by `gen_initramfs_list.sh`).

There are basically two ways to customize the contents: either create a cpio archive with required files, or specify a list of directories and files that will be merged with the default one (using `CONFIG_INITRAMFS_SOURCE`). By default, the kernel looks for a `/init` to execute on the initramfs; if it does not find it, it skips the initramfs. The kernel parameter `rdinit=` can be used to change that path. When using the approach to extend the cpio archive used by the kernel, there is no need to load a separate image. If you create the cpio archive separately, the bootloader still needs to provide the location.

**Shutdown** is often overlooked, but it can affect startup times or it can even corrupt certain file systems. The scale of a shutdown strategy can range from a simple script, or use of halt or reboot to a full System V scheme. If init is used, issuing the `init 0` command halts the system. The steps, in general, should be: send SIGTERM to all processes, wait a short time, send SIGKILL, unmount file systems, call architecture specific halt or reboot routines.

Good documentation about initramfs is in the kernel doc: `filesystems/ramfs-rootfs-initramfs.txt`.

## Resources

 - "Embedded Linux Primer" book, Chapter 6

File system layout

 - http://www.pathname.com/fhs/
 - https://wiki.debian.org/FilesystemHierarchyStandard
 - [wikipedia](https://en.wikipedia.org/wiki/Unix_filesystem#Conventional_directory_layout)

