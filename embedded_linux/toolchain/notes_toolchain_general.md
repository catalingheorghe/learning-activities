# Toolchains - General Notes

## Table of Contents

TODO

## General

When building for embedded, the tools are being run on one platform while building the application for another platform. This set of tools are collectively called *cross-platform development tools*. Note that even if the architecture of the target system is the same as the development station, it is still highly recommended to use a dedicated set of compilation tools (toolchain), instead of using  the native one.

You can either get pre-built binaries from various providers or vendors (windriver, linaro, mentor graphics etc or distributions like ubuntu, debian), or build the tools yourself.

For building the tools yourself, the de-facto toolset is the one from GNU.

**Notes on a practical project workspace**

Setting up the toolchain, either building it or taking it pre-built, should be the first part of starting a new project. In this initial starting phase, another aspect that can help with future efficiency and flow of work is to lay out a practical project workspace. Some notes regarding this point follow

 - plan your layout from the beginning
 - separate your own code from third-party code (even in separate source-control modules)  
   this might also make compliance with different licenses more easy
 - have separate directories for bootloader, kernel and the actual application
 - a `rootfs` directory usually denotes the input for the target rootfs (what the target will see as the root filesystem at runtime)
 - an `images` directory usually contains the binaries produced by the build; think also about where the rest of the output of the build will be stored (e.g. object files, dependency files, generated code)
 - `doc` directory for documentation
 - `debug` directory for debugging tools and other related things
 - `build-tools` directory for what is needed to build the cross-platform development toolchain
 - `tools` directory for the complete cross-toolchain  
   note that another common option is to install the toolchain system-wide; but that does not necessarily exclude this
 - `project` directory for different configuration files, settings, scripts for the project its-self
 - others?

You can also have a simple script that exports various environment variables, like subsystem locations, build env variables. This script needs to be sourced in order to be able to do development. This can be left to the user to do in a terminal session, or can be added to files that are sourced by the shell when started (like `bashrc`).

### GNU toolchain

A toolchain is a set of software tools needed to build computer software.

Usual components

 - linker
 - assembler
 - archiver
 - C compiler
 - C library
 - headers
 
Other components usually include debuggers, profilers and other utility tools. In the GNU ecosystem, this translates into the packages: binutils, gcc and then the C library (e.g. glibc), plus the Linux kernel headers. For debugging, gdb.

The following triplet denotes the systems involved

 - build - where you build your toolchain
 - host - where you use your toolchain
 - target - where the software built by the toolchain will run

Usually, the build and host are the same.

In GNU configure and build system, the parts of the triplet are specified in a standardized format (*tuple*). Full form is `arch-vendor-os-libc/abi`

 - arch - the system's chip architecture. Little endian usually denoted by an `el` suffix. 
 - vendor - a specific maker or family of boards; no effect on the toolchain, can be unknown or omitted as it is ignored by autoconf
 - os - used mainly for GNU/Linux and even then, sometimes, left out; only denotes the kernel type, not specific version. Example: `none`, for bare-metal, or `linux`.
 - libc/abi - a string denoting the combination of C library and ABI in use; can be all kind of strings or even directly the format (like ELF) if it is bare-metal

Some examples

 - `arm-foo-none-eabi` - bare-metal toolchain for ARM, using the ABI eabi, from vendor foo
 - `arm-unknown-linux-gnueabifh` - Linux toolchain this time, using the glibC library (gnu) and EABIhf ABI; vendor is unknown
 - `armeb-linux-uclibcgnueabi` - ARM big endian, vendor left out, uclibc and EABI ABI
 - `i386-pc-linux-gnu, powerpc-8540-linux-gnu, mips-unknown-linux, mipsel-unknown-linux` 

The tools (like gcc, ld, objdump) are usually prefixed with this naming.

Note that a different ABI or different word size (32 vs 64) is also cross-compilation. A native toolchain means *build == host == target*.

The build, host, target translate to `---build`, `--host`, `--target` in autoconf `configure`. If not specified, a native configuration and build is presumed.

#### Components of a Linux cross-toolchain

Four core components

 - binutils
 - gcc
 - Linux kernel headers
 - C library

**Binutils**

Contains utilities to manipulate binary object files: assembler (as), linker (ld) and others, like objdump, readelf, strip etc.

Needs to be configured for each CPU architecture. Example: `./configure --target=arm-buildroot-linux-gnueabihf --with-sysroot=PATH` (autoconf will guess the build and host from the local system configuration).

This usually causes the least problems, as it usually builds ok.

**GCC**

GNU Compiler Collection project. C, C++, ADA, FORTRAN and backends from many CPU architectures.

Source -> intermediate representation -> assembly source for CPU architecture. The `as` tool from binutils produces the machine code from that.

It provides compilers (`cc1` for C), but you will use the *compiler driver*, `gcc`, which is a wrapper that calls the compiler, linker and assembler.
It also provides runtime libraries if you want to run these programs on your target (`libgcc`) or even if you want to run a cpp or fortran program (`libstdc++`, `libfortran`). Plus the header files for the standard C++ library.

The build of gcc is done in two steps.

There are also some dependencies for the building with gcc: some arcane math libraries (`mpfr`, `gmp`, `mpc`) used at build time for floating point calculations. They are not needed on the target, they are needed on the host machine to build something with gcc for the target (so they are part of the toolchain as well, in a way).

**Linux kernel headers**

They are needed to compile the C library, which is the one that interfaces between the kernel API and a more user-friendly, portable, standard-compliant userspace API. The kernel ABI should not change, so building the toolchain with a set of headers and running a different version of the kernel on the target should not be a problem (i.e. if you update the kernel on the target, that does not mean you need to rebuild the toolchain).

To produce the headers, the kernel build has a couple of targets that produce sanitized versions of the headers

    make ARCH=ppc headers_check # obsolete
	make ARCH=ppc INSTALL_HDR_PATH=headers/ headers_install

In the kernel sources the `uapi` directories are the user-space visible headers.

Linux 4.8 installs over 700 header files via the `headers_install` target (kernel system calls, data structures etc.).

Note: the kernel to userspace ABI is backward compatible ("do not break userspace"). The header used for the build must be at least as old as the kernel that will run on your system.

To determine the version of the kernel headers in use in your toolchain, there is a file in the linux kernel that will get to your toolchain install dir: ` $ cat arm-none-linux-gnueabi/libc/usr/include/linux/version.h`, see `LINUX_VERSION_CODE` and `KERNEL_VERSION`.

**C library**

Implementation of POSIX standard functions and other standards and extensions.

The standard one most often used is the GNU C library (glibc). It is full featured and standard compliant, but can be heavy on resources for small embedded systems (under 16 MB of flash and small RAM). As its primary target is not embedded systems, the code is usually optimized for speed, not space.

Alternatives can be 

 - uClibc-ng (former uClibc)
 - musl
 - bionic (for Android)
 - dietlibc (tiny)
 - klibc (tiny)
 - newlib (mostly for bare-metal, can be used to build bootloaders and the even the Linux kernel but not userspace code)

What glibc/uclibc-ng/musl provide is the dynamic linker ('ld.so'), the C library itself (`libc.so`) and other additional libraries (`libm`, `librt`, `libpthread`, `libutil`, `libnsl`, `libresolv`, `libcrypt`), the standard C library headers.

Some points about glibc:

 - no support for noMMU platforms
 - no official support for static linking
 - it is ABI backward compatible - no need to dynamically relink the applications if you upgrade the C library
 - almost no configurability (all of it or none of it) - embedded systems got bigger and bigger and now this is not such a problem anymore
 - LGPL v2.1 or later

In contrast, uclibc was started and is intended for producing smaller footprint, supporting a configurable build. It also supports noMMU platforms. It supports static linking. It does not guarantee ABI backward compatibility. Note that the uClibc project is dead, the fork uClibc-ng is very active.

muls C library is MIT licensed. Very active development. Some noMMU support. There is no configurability but it is still small, especially in static linking. Strict conformance to standards, which may cause some build issues. Everything is inside `libc.so`, even the dynamic linker `ld` is in that. uClibc is moving to the same model.

**Component versions**

GCC, glibc, binutils versions need to be selected. They are independent software projects, so not all combinations are guaranteed to work for a target. Either stick to a tested combination or start with the latest stable for all three. In some cases, it might be better to pick a version with a bugfix release, not a fresh "minor/major" version (eg: 2.2.5 instead of 2.3, until 2.3.1 is released). It is also common to need to apply some patches in order to make them work ok for your target.

Some locations to check for patches and version compatibility are: debian source packages, cross-compile linux from scratch, crosstool build matrix.

### Building a toolchain

**Build overview**

Prerequisites: functional native toolchain as well.

 1. Build binary utilities
 2. Build dependencies of gcc: mpfr, gmp, mpc (built only for the host)
 3. Install the Linux headers
 4. Build the bootstrap compiler / first stage gcc: no support for a C library, support only for static linking
 5. Build the C library using the first stage gcc
 6. Build the full gcc (needs the C library already be built because it needs to know what dynamic linker to use, what pthread code etc for the binaries of gcc its-self)

**Notes for a production project**

Examples of additions to the "project script", which describe the triplet (with build equals host) for building the toolchain 

    TARGET=powerpc-unknown-linux
    HOST=i686-cross-linux-gnu
    PREFIX=${PRJROOT}/tools
    TARGET_PREFIX=${PREFIX}/${TARGET}
    PATH=${PREFIX}/bin:${PATH}

The `TARGET_PREFIX` is used for installation of target specific header files and libraries.

If you want to share the toolchain with all users of a system, preferably use `/opt` to install it in.

Manually building a toolchain should only be attempted for learning experience, as it is an effort that requires time, patience and possibly deep Unix system knowledge. Best resource would be cross-lfs (cross linux from scratch).

Note that for a production system a toolchain build must be

 - reproducible
 - documented
 - shareable

All *automated* cross toolchain build systems have these traits.

**Concept of sysroot**

The sysroot is the logical root directory for headers and libraries. It is where gcc looks for headers when processing source code, and where ld looks for libraries when linking.

Gcc and binutils are built with the same location for `--with-sysroot`. And there we also install the kernel headers and the C library. If the toolchain has been moved, gcc will still find it, if it is a subdirectory of the `--prefix`.

Can be printed with `--print-sysroot` and can be overridden at runtime with `--sysroot` (this is leveraged by, for example, by buildroot to build their own sysroot when building for an embedded linux target).

Multilib toolchains

Most toolchains have only one sysroot (one copy of the C library, kernel headers, C library headers). The runtime libraries for GCC and the C library are built and optimized for the target. So you need one toolchain for each architecture variant or ABI (e.g.: armv5, armv7, armv4 ...), as the toolchain consists also of runtime libraries for your target.

Multilib toolchains = single compiler + different sysroots. Each sysroot built with different CPU specific flags.

Multi sysroot support can be printed with `tuple-gcc -print-multi-lib`. Compiler selects the sysroot depending on the compiler flags passed.

**Contents of a toolchain**

Example is a toolchain built with buildroot.

 - `arm-buildroot-linux-uclibcgnueabihf` (toolchain tuple)
    - `bin` - small set of binutils programs (no prefix), with hardlinks to the ones with prefix in the upper `bin` folder
	- set of headers for the C++ header library
	- `lib` - gcc runtime libraries, built for the target (`libatomic`, `libgcc`, std C++ library)
	- `sysroot` - C library and gcc runtime libraries, Linux kernel and C library headers
 - `bin` - the tools with the tuple prefix
 - `include`- headers of the host libraries (gmp, mpft, mpc) - not needed at all to run the toolchain
 - `lib` - host version of gmp, mpfr, mpc needed for gcc; things created by gcc (constructor, destructor object files `crtbegin, crtend` - linked in all executables; headers provided by the compiler - `stdarg.h, stdint.h, sigatomic.h etc`; static variants of the gcc runtime libraries; linker scripts - `ldscripts`, which tell the linker how to arrange the sections of the binaries for each architecture)
 - `libexec` - the actual compilers (`cc1, cc1plus1 ...`)
 - `share` - documentation (man pages, info pages)

**Architecture tuning**

Configure time options for building gcc that will be the default when gcc will be used (`--with-arch, --with-cpu, --with-abi, --with-fpu ...`).

GCC documentation -> *Machine-dependent options*.

**What is an ABI?**

From the point of view of the toolchain, ABI is:

 - the calling convention (how function calls are made - argument passing, return value passing)
 - size of basic data types
 - alignment of members in structures
 - how system calls are made

For a given CPU architecture, there can be multiple ABIs, basically multiple specifications on how to use the CPU registers, instructions etc. Object files built for different ABIs cannot be linked together.

For example, on ARM, which is an architecture that has a lot of ABIs

 - OABI - obsolete ABI (no longer supported anywhere)
 - EABI - allows mixing hard-float code (that uses CPU floating point instructions) with soft-float code (emulates floating point operation in a userspace library from gcc)
 - EABIhf - hard-float - avoid the performance slowdown of EABI where you do have floating-point unit

**Toolchain vs SDK**

Toolchain is only the strict set of compiler, C library, binutils.

An SDK also offers other libraries that can be linked with and that can be run on the target (networking, graphics, etc) and other native tools that help in building the software.

Yocto, for example, can use an existing toolchain as input, or build its own. A build system like yocto, besides producing a root filesystem, can also produce and SDK to allow application developers to build applications for the target.

**Bootstrapping gcc**

When you build a toolchain manually, you mind end up using different versions of the native compiler to build the cross-compiler. That means that the toolchains might behave differently if they are built on the different machines.

One way to get around that is to build a version of the compiler first, that will then be used for the building the toolchain - bootstraping gcc.

Or isolate the toolchain build in a system that does not change.

#### Crosstool-ng

TODO

Specialized in building cross-compilation toolchains.

 - http://www.ebadf.net/2012/03/09/crosstool-ng/
 - https://github.com/bootlin/toolchains-builder/issues/2

#### Embedded Linux build systems

Yocto, Buildroot, OpenWRT can also build toolchains.


## Resources

 - "Building Embedded Linux Systems, 2nd edition", O'Reilly
 - "Anatomy of Cross-Compilation Toolchains" - [youtube](https://www.youtube.com/watch?v=Pbt330zuNPc&t=550s)
 - [reddit question about gcc, glibc, binutils, libstdc++](https://www.reddit.com/r/linuxquestions/comments/1tghjd/what_is_the_relationship_between_gcc_libstdc/)

Specific

 - [gcc docs](https://gcc.gnu.org/onlinedocs/)
 - [binutils docs](https://www.gnu.org/software/binutils/) -> Documentation
 - [crosstool-ng](https://crosstool-ng.github.io/docs/)

