# Kayak - Reversible Computing System

!! THIS PROJECT IS A WIP, IT IS NOT YET IN A USEABLE STATE !!

Kayak is a reversible programming language implemented on top of an efficient, highly concurrent virtual machine. A "reversible programming language" is a Turing-complete language that can execute code both forwards and backwards, while traditional languages can only execute in one direction. This provides a myriad of benefits for computing such as greatly increased energy efficiency, simplified software design, and more.

Kayak is a statically-typed, pure functional language that is also highly concurrent by use of a simple message passing system between live functions. Kayak handles information effects through the use of elegant, composable functions called "arrows". Information effects are necessary in a reversible language for any action that potentially may be non-reversible, such as deleting data or communicating with other processes across a network.

Kayak is designed to be a 'programming language as distributed operating system', meaning that programs are able to modify themselves while they are running via computational reflection primitives. Kayak provides a visual interface and language for interactive programming, and its capability-based security system provides fine-grained and composable security for distributed applications.

## Features

### Pure Functional Programming
Kayak primitives consist of pure, statically-typed reversible functions, meaning they cannot create or destroy data. Side effects are provided via the language's arrow metalanguage; think monads, but easier to understand and work with.

### Efficient
Logical reversibility means no automatic garbage collection and greatly increased energy efficiency. Theoretically, reversible computing can be so efficient that it doesn't consume any additional energy at all. We are currently doing research into designing a Kayak-based ISA for bare-metal processors which can approach this limit.

### Highly Concurrent and Distributed
All data structures in Kayak can be written to disk or passed over a network with no serialization/deserialization required. Distributed computing is made easy with Kayak's capability-based security and arrow system. Moreover, the Kayak VM can also automatically parallelize programs into separate processes that communicate with each other via statically-typed channels.

### Reflective
Kayak programs are represented as a data type in Kayak itself, allowing code to inspect and modify itself at runtime. This allows for powerful abstractions such as hot code migration, DSLs, automatic sandboxing of untrusted code, and more.

### Visual
**Racecar** is our home-grown IDE which provides an intuitive visual language that allows even inexperienced programmers to build complex Kayak programs. For those who prefer to use their own programming environments, Kayak also provides a simple and elegent text-based syntax.

### Free and Open Source
Kayak's virtual machine and standard library are licensed under the GPLv3. Open-source software is not only a public good, but necessary for reversibility between source code and executable images. Contact us for commercial EULA licensing options.