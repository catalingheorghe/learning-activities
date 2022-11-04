# Integrated build environments

## General

A lot of pieces need to fit together to build an embedded Linux distribution (toolchain, kernel, editor, application software etc). Build environments exist that already piece together things that work with one another.

One of the differentiator between different integrated build environments is the capability to add package manager on the target. Yocto, for example, can do that; basically, it can create a full blown Linux distribution. Buildroot, on the other hand cannot do that, as it is mainly intended to build root file systems, even though it can also build bootloader and kernel images, plus toolchains.

Another feature that may be useful, especially for production endeavours, is the ability of a build system to keep track of the licenses (and I presume the versions) of the upstream packages that you are incorporating in your firmware.

Some build systems are:

 - Buildroot (one of the first)
 - OpenWRT
 - OpenEmbedded (part of yocto)
 - Yocto (most popular, one could argue)

## Buildroot

### Overview

Buildroot is a set of Makefile, scripts and patches put together in a system that is designed to build a complete embedded Linux distribution (toolchain, kernel, bootloader, root file system, including support for various packages - user space applications and libraries).

 - Linux kernel menuconfig like configuration
 - glibc, uclibc and others
 - simple structure based on makefile language

Stable releases are produced four times per year. Some of them are also markes as LTS (long term support; in this case, 13 months).

Installing is a matter of either using a git checkout and the corresponding tags, or using tar archives. Reading the *System Requirements* from the user manual must be a prerequisite.

After downloading a buildroot snapshot, a simple configuration can be done via `make menuconfig/xconfig/gconfig` . You can select architecture, build options, toolchain options, user-space packages etc. Then the build is started via `make`. At the end the results with be placed in `output/`. The archives of upstream projects downloaded will be placed in `dl/`.

To see a list of default configurations: `make list-defconfigs`. All supported configs are in the `configs` directory.

Adding your own programs can be done via "overlay" or by creating a buildroot package integrated into the configuration of the build system. The overlay option simply means that you build your executable or library outside of buildroot and copy over the buildroot root file system. Note that it is important to build the program with the toolchain that is use by buildroot - simply adding `output/hosr/usr/bin` to PATH and using the proper target tuple should do it. Then you copy it to a directory and set a menuconfig option for rootfile system overlays to point there.

Buildroot can help with the legal requirements (`make legal-info`).

### Online resources

 - [buildroot.org](https://buildroot.org/)
 - [buildroot official documentation](https://buildroot.org/docs.html)

## Yocto


 - https://developer.ibm.com/tutorials/l-yocto-linux/
 - https://events.linuxfoundation.org/wp-content/uploads/2017/12/Buildroot-vs-Yocto-Differences-for-Your-Daily-Job-Luca-Ceresoli-AIM-Sportline.pdf

## Resources

 - "Embedded Linux Primer"
 - "Mastering Embedded Linux Programming - Second Edition"
 - 
