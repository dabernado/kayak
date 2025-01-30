# Kayak - Reversible Computing System

!! THIS PROJECT IS A WIP, IT IS NOT YET IN A USEABLE STATE !!

Kayak is a reversible programming language implemented on top of an efficient, highly concurrent virtual machine. A "reversible programming language" is a Turing-complete language that can execute code both forwards and backwards, while traditional languages can only execute in one direction. This provides a myriad of benefits for computing such as greatly increased energy efficiency, simplified software design, and more.

Kayak is a statically-typed, pure functional language that is also highly concurrent by use of a simple message passing system between live functions. Kayak handles information effects through the use of elegant, composable functions called "arrows". Information effects are necessary in a reversible language for any action that potentially may be non-reversible, such as deleting data or communicating with other processes across a network.

Kayak is designed to be a 'programming language as operating system' much like Smalltalk, meaning that programs are able to modify themselves while they are running via computational reflection. Also similar to Smalltalk, Kayak provides an interface for interactive programming. When used in a strongly-typed live programming system, arrows take on a character similar to object-capabilities in an operating system in which processes aren't able to run functions with information effects that they haven't already been granted.