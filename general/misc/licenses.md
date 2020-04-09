# Open Source Licenses

## History

Richard Stallman started the non-profit organization Free Software Foundation. He also put the basis for the GPL license.

The GPL allows you to make changes to software, as long as you publish those changes (so that others may use them or incorporate them back).

*Free Software* must be regarded as in freedom/liberty, not *free* as in monetarily free. Companies can make money with GPL products, for example by offering support. From the gnu.org website: "Specifically, it means that a user is free to run the program, study and change the program, and redistribute the program with or without changes".

To reduce the confusion, the term *Open Source* was coined in the 90s. It does not specifically refer to GPL-licensed software, but to the fact that software comes with the source code, even if under a more restrictive or permisive license than the GPL.

### GPL

The **GPL** allows freedom but also mandates to respect the **license stipulations**:

 - you can copy the code/program, as long as the license and copyright are intact
 - no warranty is provided
 - you can distribute binary copies as long as you accompany them with the source code used to create the binaries ("original" source code)
 - you cannot place further restrictions on your recipients
 - you can modify and distribute the program, as long as you provide the same rights you received 

Notes

 - "modifying" the software creates "derived work", which is subject to the original license terms
 - no difference is made between static and dynamic linking
 - running the code is different than modifying the code
 - a derived work combining GPL software and non-GPL software through any kind of linking must be distributed under GPL

What is **free software** and **copyleft**?

For a program to be free software, it must offer its users the following four permisssions:

 1. the freedom to use the software for any purpose
 2. the freedom to share the software with anybody
 3. the freedom to change the software as our see fit
 4. the freedom to share the changes that you have made

The GPL license ensures those freedoms. It ensures that the software will remain free, no matter who changes it. This is called **copyleft**: the software is copyrighted, but instead of restricting users, the license terms makes sure the users have these freedoms.

**LGPL** - you can link against unmodified version (like the C library); you must distribute modifications under the same LGPL.

**More unclear situations**

 - apps that run using the Linux kernel? aren't they linked in a way to the kernel?
 - binary kernel modules?
 - GPL software in embedded system?

Embedded systems: you are allowed to distribute binary copies of any GPL software as long as your recipients receive the original source code (and modifications, if modified), or the possibility for them to request the software for a fee limited to the cost of the media and transportation.

Preamble to GPL in the Linux source: you can run any application on the LInux kernel without "GPL contamination".

The most unclear situation is with binary kernel modules (ex: for graphics cards). In embedded, this can range from NAND flash drivers, to codec support modules to anything.

**TODO: look into tainted kernel - this info might be a little outdated/incomplete**

This has not been settled in court, yet, but the general consensus is that they are illegal.

GPLv3 introduces **anti-tivoization**. Tivo is a set-top box running a modified Linux kernel that uses cryptographic hashes in order to prevent users from replacing the software with their own customized versions. To Richard Stallman, the use of GPL is undermined whenever an embedded developer introduces cryptographic or DRM measures to prevent users from changing the system, even if the source code is available. This can be regarded as loophole in GPLv2, which has been addressed in GPLv3, which requires the distributor to provide you with whatever information or data is necessary to install modified software on the device (this may range from a set of instructions to a set of keys etc).

**License compability**: in GPLv2 it was stated that you could add code to the project as long as the license on the other code did not have any restrictions that were not already in GPLv2. GPLv3 is more clear and is also compatible to Apache License 2.0. A detailed list of licenses compatiable with GPL is here: https://www.gnu.org/licenses/license-list.html.

Interestingly. OpenSSL is on the incompatible list of the previous link. There is an exception in the GPL for libraries provided with the OS, but things are still not very clear (https://www.openssl.org/docs/faq.html#LEGAL2, https://people.gnome.org/~markmc/openssl-and-the-gpl.html). For this reason, OpenSSL has relicensed since 3.0.00 to Apache License v2 (https://www.openssl.org/source/license.html).

**Linux kernel - GPLv2? why not v3?**

"First, Linux does not require contributors to assign copyright to some central person. Copyright on it is spread out among many, many people (including some who are dead, in which case much effort would be required to figure out who inherited the copyright). Any of them could stop a license switch, unless someone goes through and removes all parts of their work from the kernel. Because GPL v2 and v3 are incompatible, it is illegal to release Linux under v3 with any contributions licensed under v2 only.

Second, Torvalds personally does not like GPL v3. He particularly dislikes certain provisions (like anti-tivoization), which are not restrictions he wants to impose on users of his software. As he won't release his stuff under v3, the whole kernel can't be released under v3 by anyone without prohibitive effort. But he is not the only person who could singlehandedly make it impractical to release the kernel under v3."

 - https://opensource.stackexchange.com/a/1776
 - https://www.kernel.org/doc/html/v4.18/process/license-rules.html

RTLinux patent is special. 

**TODO read about it (chapter 12 in building embedded systems; other sources)**

**What about "or later"?**

"The way developers state their choice is in the *license notice* that goes at the start of each source file. That's where the GPL says the decision is stated.

The Free Software Foundation urged developers to choose or any later version, since that meant users would be free to use that program under GNU GPL version 1, or under version 2 (once there was a version 2), or under version 3 (once there was a version 3). And they will be free to use it under version 4, if we ever have to make a version 4.

Two different copyleft licenses are almost inevitably incompatible in the absence of some explicit compatibility mechanism. That is because each one necessarily requires modified versions of a program to be released under that very same license. As a consequence, code released under GPL version 2 only can't be merged with code released under GPL version 3 only."

 - https://www.gnu.org/licenses/identify-licenses-clearly.html

Note that other licenses give users the explicit permission to use the work under later version of the same license (Mozilla Public License, Eclipse Public License, all Creative Commons licenses). This might be included in a future version of the GPL as well.

**Machine parsable license identifiers** in each file is needed. Because text boilerplate is hard to parse, a machine parseable identifier has been introduced by https://spdx.org/.

**GCC Runtime excpetion notes**

some info regarding gcc lincense(s)

1.
update, after some more digging:

> What libraries does the GCC Runtime Library Exception cover?
>
>    The GCC Runtime Library Exception covers any file that has a notice in its license headers stating that the exception applies. This includes libgcc, libstdc++, libfortran, libgomp, libdecnumber, >    libgcov, and other libraries distributed with GCC.

the entire info is here: https://www.gnu.org/licenses/gcc-exception-3.1-faq.html

what we care about is libgcc - this copied at runtime in the rootfs (besides the actual idea of compiling with gcc - various headers that may get copied into the software etc)

2.
And the runtime exception itself is clear:

> You have permission to propagate a work of Target Code formed by combining the Runtime Library with Independent Modules, even if such propagation would otherwise violate the terms of GPLv3,

https://www.gnu.org/licenses/gcc-exception-3.1.html

3.
I have looked in the gcc source-code. There is a folder `libgcc`. In that folder I did some grepping and looked at some files.

 - there are files under "lesser gnu public license". All of these file seem to be under version 2.1
 - there are files under "GPL version 3 with GCC Runtime Exception 3.1"
 - there are files in "Public domain"

so, my conclusions,

 - the runtime parts that we care about (libgcc) are a mix of "GPLv3+GCC Runtime Exception", LGPLv2.1 or later+also an exception text, public domain
 - **I believe that this makes the "gcc-linaro" and "gcc-oselas" licenses that we show in the table actually be "GPLv3, GCC Runtime Exception 3.1"**
 - the end, all is good

## (Other) Resources

 - https://www.gnu.org/licenses/licenses.html
 - https://www.osadl.org/





