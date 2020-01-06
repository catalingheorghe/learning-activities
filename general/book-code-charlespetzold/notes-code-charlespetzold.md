# Code: The Hidden Language of Computer Hardware and Software

Charles Petzold

## Table of Contents

## 0. About

Amazon [link](https://www.amazon.com/Code-Language-Computer-Hardware-Software/dp/0735611319/ref=sr_1_1?crid=3VV2SRGGH0GCR&keywords=code+the+hidden+language+of+computer+hardware+and+software&qid=1575792007&sprefix=code+the+hidd%2Caps%2C283&sr=8-1)

O'Reilly Learning [link](https://learning.oreilly.com/library/view/code-the-hidden/9780735634688/)

Goodreads [link](https://www.goodreads.com/book/show/44882.Code)

## 1-2. Morse Code

Dots and dashes. Good for flashing a light; there are even special flashlights for Morse code, to make it easier. Even better for telegraph systems (see later chapters).

S.O.S. - simplest morse code (`... --- ...`)

Morse to normal - how to organize them to make it easier to decipher:

 - tables for each possible length (from 1 to 6, 7)
 - a binary tree with each node being a possible simbol

Samuel Morse (1791 - 1872) [wikipedia](https://en.wikipedia.org/wiki/Samuel_Morse)

![Morse Letters](02-morse-letters.png)

## 3. Braille

Louis Braille (around 1800 - 1850). 

Started from a writing system used in the military to pass around messages and enhanced it.

Each symbol is a 2 by 3 matrix of possibly raised dots. 64 symbols. Grade 2 braille uses besides the letters, other codes for either common words or groups of letters.

*Shift codes* are when a symbol changes the meaning of the followig symbols. *Escape code* is escaping the following symbol (chaging its meaning).

Braille code is a good example of how symbols are chosen to make them easier to remember. For example, some symbols are represented by a patter in the upper half of the matrix, while associated symbols are represented by the same pattern, but in the lower half.

![Braille Letters](03-braille-letters.png)

## 4. Electricity: Anatomy of a Flashlight

Eletricity - flow of electrons. 

Atom - protons, neutrons, electrons. Number of protons gives the atomic number. Electrons usually match protons. Flow of electricity - electrons moving from one atom to another.

Charge: protons, `+` / electrons, `-`.

Batteries contain chemical reactions that push more electrons towards the `-` (anode), less towards the `+` (catode). These chemical reactions take place if there is a circuit from minus to plus. Batteries in series: increased voltage.

![Flashlight Serial](04-flashlight-ser.png)

In parallel, same voltage, more battery time.

![Flashlight Parallel](04-flashlight-par.png)

Conductors - only one electron in the outer shell. Best: copper, silver, gold.
Opposite: insulators (plastic, rubber) - high resistance (measured in ohms).

*Note*: the thicker the wire, the less resistance (more electrons available to flow).

Voltage - a potential. Current - number of electrons flowing (amps). Like pressure and amount of water, respectively. 

  `I = E / R`

E - voltage, electromotive force.

Short-circuit - connect plus to minus, (virtually) no resistance, so a high current. If battery is large, wire might melt.

A resistor glows if it is thin. This is how the *incadescent bulb* works. Filament made of tungsten, in a vacuum (in open air it would burn up).

Watt is a mesure of power.

  `P = E x I`

A switch allows electricity to flow when it is *on*, or *closed*.

This chapter explained the decomposition of a flashlight (used for face to face morse code) into its basic elctrical circuit elements.

Volta - 1800 fist battery
Edison - 1878 incadescent lightbulb (patent), other inventors were also working on this

## 5. Seeing Around Corners

This chapters shows how you can extend the circuit for a flashlight, or two, to communicate around corners (out of sight), by creating a kind of lightbulb based telegraph system.

Wires can be reduced if the negative sides of the batteries are connected together (*common*). And that common part could even be replaced with an omnipresnet conductor, the Earth (aka ground): stick an 8 pole copper pole into the earth. However, ther Earth has more resistance, so voltage has to be increased (1.5 v batteries to 120 household) and bulbs with more resistance used (normal lightbulbs, not flashlight bulbs). 

Still, this early telegraph system has its limits: the distance of the wires. The longer the wires, the bigger the resistance, no matter how thick (and expensive) the wires.

![Lightbulb Telegraph](05-finalsystem.png)

## 6. Telegraphs and Relays

Morse invented the telegraph in 1836. It was based on electromagnetism - a long wire wrapped many times around a metal bar. Passing current made the metal attract other metals. A proper lightbulb came around in 1878, so that was not an option. Sound was the option (the electromagnet pulls down a flexible piece of metal which was then pulled back - *click clack*).

The distance problem mentioned in the previous chapter was solved by repeatears. The destination end was replaced with a *relay*, basically a switch that was activated by current. The electromagnet pulls a flexible strip of metal that closes the outgoing circuit.

This relay is an important step in the upcoming computers.

![Telegraph Relay](06-tele-relay.png)

## 7. Our Ten Digits

Numbers are the most abstract code that we have. Probably invented to keep track of merchandise. Based on your ten fingers (or digits).

Roman system is probably the only ancient system that survives. It is easy to add and subtract, but not to multiply and divide.

Out decimal system is hindu-arabic. Invented in India and brought to Europe by the Arabs (around 1100). The biggest differences are the fact that it is positional (the position of a digit matters) and it has a dedicated symbol for 0.

The positions, even fractional, represent power of 10. Operations come down to operations on pairs of single digits pairs 

## 8. Alternatives to Ten

A cartoon character has 8 fingers, so what does 9, or even 8, as symbols mean to him? 

In a positional numbering system, 10 (one zero) is the representation of the "number of fingers". The addition and multiplication tables for sIngle digits change, but the rest of the rules remain.

In octal, a nice round number is a power of two because it is a multiplication with a power of 8 (which is a power of two).

![Octal Fingers](08-octal-fingers.png)

Going down, a dolphin can only count with 0 and 1. Binary. The numbers are longer, but the addition and multiplication tables are much smaller, faster.

0 and 1 - current through a wire, switch on off, lightbulb on off

The term *bit* was coined in the 1940s for a shorter form of binaty digit. (Tukey - mathematician - 1948).

## 9. Bit by Bit by Bit

The bit is a special numbering system because it is just a yes or no. But with enough bits, any information can be encoded.

The UPC (universal product code) is a good example of a real, production code

 - a thin bar is a 1, a thin gap is a 0; bars can be 2 3 4 times thicker; gaps the same
 - thin bar 1, thin gap 0 - 95 bits (plus 9 extra gaps, aka 0 bits, on either side)
 - starts and ends with the marker 101 - this allows for the scanner to calibrate and know how thin is the thin bar (and the gap). Also allows it to be read from right to left as well
 - in the center it has a specific center pattern, a *guard pattern*. If not found, malformed or misread (*error detection*)
 - the left side encodes 6 digits, the right side the same
 - 7 bits are used for each digit. On the left side, a digit code always starts with a 0 and ends with a 1 (format guard, error detection); 2 groups of one, so two vertical bars per digit; odd number of 1 bits - odd parity
 - right side is the exact opposite (binary negative); so parity is even; this allows to know whether you are reading from right to left

So, a lot of extra bits (over 100) for a "simple" 12 digit code.

The first digit has a meaning (regular code, has to be weighed, coupons etc). Next 5 digits represent the company id, next 5 product id. Last digit is modulo check character (`n*10 - 3 * (sum(first 5 digits)) - sum(second 5 digits)`). Again, integrity, error check.

![UPC](09-upc.png)

Morse and braille also can be encoded into binary.

## 10. Logic and Switches

Ancient greeks considered analyzing logic in language to help in the search of truth (4th cent bc). Eg: syllogism - drawing a conclusion from two premises.

George Bool, around 1850, transformed logic into mathematic - boolean algebra. The operands are sets, or classes and the operators are union, intersection. Using it you can prove a syllogism, for example. (All men are mortal, Socrates is a man, Socrates is mortal).

The numbers 1 and 0 can be used to represent yes and no as satisfying criterias of being in a class or not. *Or* and *and* operators can help with complex selection criterias.

But what if you transform 1 and 0 into a switch, open or closed, and *or* and *and* into switches in parallel and in series. Then, if the circuit closes (telegraph relay rings for eg), you made the connection between boolean logic and electronics. Like the selection of a cat based on a set of specific criterias (sex, color, neutered).

![Switches](10-logic-switches.png)

Nobody made this connection in the 19th century, not even Babbage, credited to invent the precursor to modern computers using gears and levers.

## 11. Gates

Shannon (1930s) who wrote about the theory of communication made the connection between switches, or even better, relays, and boolean algebra.

A relay has an input and an output. A combination of relays is a logical gate.

 - And
 - Or
 - Inverter
 - Nor
 - Nand
 - Buffer
 - 2 to 4 line decoder (2 inputs, one of 4 outputs trggered)

De Morgan worked on boolean algebra in the same time as Bool. De Morgan's laws: negation of operands is the negation of the result of the other operation.


