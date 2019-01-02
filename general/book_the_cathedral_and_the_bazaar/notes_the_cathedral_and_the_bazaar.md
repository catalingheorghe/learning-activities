# The Cathedral & the Bazaar

Eric S. Raymond

## Table of Contents

0. About
1. Notes
2. Links
3. Review

## 0. About

Safaribooks online [link](https://learning.oreilly.com/library/view/the-cathedral/0596001088/).

## 1. Notes

**First essay** goes through a brief history of hacker culture and hacker world ("hackerdom").

It emphasizes the initial meaning of the word *hacker*, as somebody who wants to solve problems and to know how things work, as opposed to the negative connotation that it gained.

Ken Thompson was the one who started writing Unix (based on Multics, a time-sharing OS similar with the well-known, at that time, OS ITS from MIT). Together with the C language, by Dennis Ritchie, this gave rise to portability between different machines.

When Bell Labs started commercialization of its technology products, including Unix, Richard. M. Stallman (RMS) was the most vehement against this, and the one who founded FSF (Free Software Foundation). The foundation started GNU (GNU's not Unix) OS, a port of all tools and parts that composed the OS, so that they can remain free. *Free as in free speech, not free beer.*

RMS was also the inventor of Emacs, an editor that dominated the hacker world for decades, and that is still widely used. FSF put in place gcc and all tools that now are omnipresent in the "linux world".

There was a time that different vendors had proprietary versions of Unix, like AT&T. They did not manage to come together with common ideas and approach. Berkeley Unix, the BSD flavor, started at that time as well (around 1984). X windows system also started then, at MIT. It won over Sun's proprietary window management system. Unix was tied with networking culture as well, first ARPANET, then the Internet.

The DOS and Mac worlds did not develop a culture around them because there were no sources shared and no networked community, like with ITS, Unix, BSD.

The proprietary Unix vendors did not manage to produce a cross-platform OS for the new PC market. Microsoft was able to grab the market with an OS considered inferior. The FSF was not able to deliver the HURD kernel for the GNU OS, even after 10 years.

Fortunately, in 1991 Linus Torvalds began developing a free Unix kernel (starting from Minix, based on Unix), using the GNU toolkit, for 386 machines. The new approach was the huge number of hackers involved once it was put forward by Linus and the pace of releases. It worked out, Linux was a worthy, free, Unix kernel that could be used for building a free OS - GNU/Linux. The development model was also something new and relied heavily on the growing Internet and World Wide Web.

Note: at the time, the BSD ports looked much more promising, technically, than the new OS from Linus. However, the community that formed around Linux, the development approach, compensated easily and made Linux more successful. BSD is still very alive, in at least three variants (open, net, free); the communities are still regarded as more rigid.

Note: the languages at that time were LISP, C and, later, Perl.

**The second essay** describes the new development model of open-source software (like a bazaar), opposed to the more restrictive, closed, traditional way, like building a cathedral carefully.

It presents the story of fetchmail, a project that the author maintained, starting from how he got in that position, how he involved users and how the project grew. There are several mention to Linux as a model that the author tried to emulate, in order to see if it can be replicated. The result was clear, it can.

Other examples of open-source and involving communities are given. Like the gcc, which at some point had a traditional, closed community and an open one. The open one grew faster and better; gcc was actually "moved" to that one. Emacs was also a good of example of how a complex software can flourish and retain architectural integrity in a seemingly chaotic model. 

Some basic principles of open source development are identified: recognize a good idea, involve your users, the benefit of bug reports with source code information, be ready to throw one away, or more, put out a stable product at the beginning.

There are many references to *The mythical man month*. Technical ones are usually enforced; the main one that the open-source model contradicts is the one about the complexity raising with the number of developers. However, traditional vs open-source is not as simple as counting developers.

General good practices for any software:

 - the importance of data structures, as opposed to flowcharts. Smart data structures are much better than smart code
 - a design that is perfect - there is nothing more to take away (not adding more things)
 - gateway software, in general, should leave the data as untouched as possible and should throw away as little as possible in the process
 - no to security by obscurity
 - be ready to throw one design/system away

Coding is indeed a solitary activity, but the huge power of the bazaar model is the attention and brainpower provided by large communities.

The two models also display totally different leadership styles, the conventional software management vs the open-source leadership. The open-source model relies on interested, passionate developers, that find motivation in the work that they are doing. It is argued that traditional management relies on mobilizing the vast majority of developers (~95%) that are doing this only for a leaving, and need motivation and structure in order to produce. (Do we really need this many programmers? Wouldn't it be better to rely on fewer, competent, interested ones?).

Opening the sources of Netscape is mentioned. This started the Mozilla group and was very important in the fight with Microsoft to not allow the browser market to become a monopoly.

**The third essay** looks into the customs of the open-source world. It is a very interesting analogy with John Locke's property law. Also with a general behaviour of a gift-culture.

In the world of free software, the FSF is perceived as puritan. Similar, the GPL license is more "hard-core" than a BSD license that a pragmatist may prefer. The FSF is not anti-commercial.

All open licenses (GPL, BSD, Perl's Artistic license) do not restrict forking, anyone can hack anything. Still, there is a clear sense of ownership. There is one recognized source of a software - the one who distributes the modifications. Out of tree work and rogue patches are not under that ownership.

What is the return, the yield of open-source work? Answer: reputation. The culture is identified as a gift-culture, one in which the status is proportional to what you give away. This is different from the general world, which is an exchange-culture (free market in which status is in line with what you can trade) with some command hierarchies (governments, institutions). So the status in this hacker culture is based on peer repute.

(the text touches on ego satisfaction vs self-esteem - is the western way of looking at ego satisfaction as a bad thing really the proper one?) - this also leads to bugs against the code, not against any individual; avoid boasting, competence criticism, etc.

Interesting look-back and prediction of focus: 80s - development tools, internet tools, 90s - OS, 2000 - applications, next? (cloud?)

There are different ways large open-source project are led: a benevolent dictator and sub-system maintainers, a voting committee, rotating benevolent dictator. In any, case, the most work is done by the co-developers in the community. It is interesting to consider how they find motivation in this.

*"The more complext the activity, the more it's hurt by extrinsic motivation."* Interesting paragraphs about motivation in creative work (including programming), with good references for the opinions presented. Bonuses are good for routine work; for creative work it is better to decouple salary from performance. Let people choose their own projects, reward when he is so motivated that he would have done the work without the reward as well.

The open-source model provides intrinsic motivation.

**The fourth essay** dives into the economic view of the model.

Most programming work is done for in-house code that has no sale value, only use value. Code that is used to run other systems for the business. The most money comes from services, not from the selling of the software.

Maintenance work is more than development work.

Following an open-source approach would lower the waste. But can it work? It is compared to the "tragedy of the commons", in which a village shares the green field near it for grazing cattle. Without central allocation, the only outcome is that the field will be overrun (overuse); all will want to take advantage (free-rider). At the end, the field will degrade into a sea of mud.

For open-source, the use does not deteriorate, it actually increases the valued, so it is an inverse commons. The free-rider effect is present, clearly, but sitting on the patch you need gains nothing (it actually incurs the future cost of maintenance; why not give it back to the community?). The less hoops you need to jump thought to contribute to a project, the more success it will have; people will give changes back.

All 4 types of licenses (GPL, BSD, MIT, Artistic) have the effect of making direct sale value hard to capture. Mainly because they prohibit restrictions on distribution and usage. The Open Source Definition puts together the criteria for all open-source license.

Interesting examples of things like DOOM game, who at some point release the sources in order to gain in the seconday market created (extensions from users). RedHat who put effort into standardizing RPMs in order to extend the pool, make the OS more usable. Cygnus which invented "configure" in order to make GNU tools more portable. They are also the ones behind Cygwin.

Why don't hardware vendors open-source their driver code? In the author's opinion, it makes no sense; especially since it can be reversed engineered. They could also gain from long-running maintenance from the community.

Open source: peer reviews, standards, reliability and scalability, future-proof, no vendor lock-down, networked community.

**The last essay** goes back to continuing the first one, presenting some key points in how open-source became known outside of the hackerdom.

In 1993, the HURD kernel was still not ready. Then, Linux came along.

In 1998, Netscape (trying to fight Microsoft), goes open in the client. This was done to prevent monopoly on the client side, which could lead to controlling the standards for communication, leading to only Microsoft servers. Netscape cared about the servers.

In 1998, the term "open source" was coined; the term "free software" introduced and still introduces a lot of undesired confusion. FSF was associated with some negative terms (anti-IP, communism ...). Open Source Initiative, opensource.org [link](https://opensource.org/), was created. Marketing, this time, was also a part of the strategy. Open Source Definition was created, based on Debian guidelines.

Even in 1998, "free" software was already leading important areas (bind, sendmail, perl, apache).

Tim O'Reilly from the publishing with the same name was a supporter of this community.

Oracle announced porting internal systems to Linux.

The halloween documents, Microsoft internal documents about derailing open-source initiatives, were released. 
Windows refund day

Linux and open-source software - servers, IPS, data centers etc. 2018, still not year of the Linux desktop, but the "world dominance" goal was not far-fetched.

## 2. Links

 - http://www.catb.org/esr/faqs/hacker-howto.html
 - http://www.catb.org/esr/faqs/hacking-howto.html
 - http://norvig.com/21-days.html
 - https://opensource.org/osd.html
 - https://www.cnet.com/news/10-years-gone-the-va-linux-systems-ipo/
 - http://www.catb.org/esr/

## 3. Review

A book that offers a glimpse into the history of hacker culture, unix and open-source world, with the main focus being the open-source development model. An easy and pleasurable read, containing impressive analogies with models from the economical and social domains.

Where necessary, the author provides references to studies and other works that go way beyond the technical domain of software development. To me, it was clear that the opinions exposed were properly researched and had a solid basis, and this is something I appreciate.

Even more than the thought provocative modelling of the open-source culture, what I enjoyed most about the work was the fact that it gave me an overview of a truly exciting period in computer science (beginning of time-sharing OS-es, Unix, GNU, open-source). It gave me a lot of opportunities to search for various other resources related to the people, institutions and technologies. All in all, reading the book made me feel that I could relate more strongly to today's "Linux and FOSS world", which I already am apart of, because I now better understand where it comes from and what where the initial events and values that led here. And it made me want even more to grasp the internals of Unix/Linux, different programming paradigms and proper software design.

