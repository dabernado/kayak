## Kayak Standard Library
### Syntax (defined in compiler)
- expressions
    - `( )`

- list comprehension
    - ???

- constants
    - "constants" are actually functions that compile down to the type `1 <-> a` instead of just `a`
        - example: `"test"` compiles down to creating a list of n-folded units to create each character
    - adding the sym character before a constant implements its inverse
        - example: `5` compiles to 5 folds, while `-5` compiles to 5 unfolds
    - is it impossible to create sum type constants?

    - units:
    ```
    UNIT :: 1
    UNIT = ()
    ```

    - nats:
    ```
    ID :: nat
    ID = 8
    ```
        - chars:
        ```
        CHAR :: char
        char = 'h
        ```

    - sums:
    ```
    SUM_RIGHT :: (nat + 1)
    SUM_RIGHT = right ()
    ```
        - booleans:
        ```
        FLAG :: bool
        FLAG = false
        ```
        - integers:
        ```
        KEY :: int
        KEY = left 24
        ```

    - products:
    ```
    NUM_PAIR :: (nat * nat)
    NUM_PAIR = (4, 6)
    ```

    - lists:
    ```
    NUM_LIST :: [nat]
    NUM_LIST = (0 1 2 3 4)
    ```
        - strings: 
        ```
        USER :: str
        USER = "username"
        ```

- function definitions
    - ```
    myFunc :: a <-> b
    myFunc = swapp changeVal zeroi
    ```

- combinator definitions
    - ```
    trace f :: b <-> c
        where f :: (a + b) <-> (a + c)
    trace f = zeroi (expn | id) assocrs (id | f) assocls (coln | id) zeroe
    ```

- arrow definitions
    - ```
    deleteVal :: a ~> 1
    deleteVal = arr myFunc >> delete
    ```

### Core Types (defined in compiler)
- `0` (zero)
- `1` (unit)
- `a + b` (sums)
- `a * b` (products)
- `[a]` (lists)
- `nat` (natural numbers)
- `int` (nat + nat)
- `char` (nat)
- `bool` (1 + 1)
- `str` ([char])
- `a <-> b` (isomorphisms)
- `a ~> b` (arrows)

### Core Functions (defined in compiler)
- `id`
- `zeroi`, `zeroe`
- `swaps`
- `assocrs`, `assocls`
- `uniti`, `unite`
- `swapp`
- `assocrp`, `assoclp`
- `distrib`, `factor`
- `fold`, `unfold`
- `expn`, `coln`
- `( | )` (sum combinator)
- `( , )` (product combinator)

### Core Combinators (defined in compiler)
- `-f`
    - sym combinator, inverts the following function
    - compiles to `UNCALL` or the inverse of `f` if `f` is a core function

### Core Arrows (dynamically defined in compiler)
- `send`, `recv`
    - takes a product of a processes' address and a value
    - `send` sends the value as a message to the process, replacing it with a unit value
    - `recv` deques a message out of the processes mailbox and replaces a unit value with it

### Core Arrow Combinators (defined in compiler)
- `>>`
    - composes two arrows together
    - example: `create >> delete`
- `arr f`
    - changes type of iso `f` to an arrow so that it can be sequenced with arrows
- `first a`
    - applies arrow `a` to the first value of a product type
- `left a`
    - applies arrow `a` to a left value of a sum type
- `spawn a`, `return a`
    - spawn + return process running arrow `a` with the current value as an argument
    - `spawn` replaces argument with the new process address
    - `return` takes a process address and waits for it to terminate, receiving its result in its place

### Functions
- `add`, `sub`
    - takes a product of two nats and adds/subtracts the first nat to/from the second one

### Combinators
- `trace f`
    - additive trace, implements looping
    - defined as `zeroi (expn | id) assocrs (id | f) assocls (coln | id) zeroe`
- `neg f`
    - transforms the type of `f` from `a <-> b` to `-a <-> -b`
    - defined as `???`
- `if (f) (g)`
    - if-statement which takes `(bool * a)` as an argument, and applies f on a if the bool is `true` and g if the bool is `false`
    - defined as `distrib ((id, f) | (id, g)) factor`
- `addc n`, `subc n`
    - add to/subtract from a number with a constant
    - defined as `uniti (id, n) add (id, ~n) unite`

### Arrow Combinators
- `second a`
    - defined as `(arr swapp) >> first a >> (arr swapp)`
- `right a`
    - defined as `(arr swaps) >> left a >> (arr swaps)`

### High-level Language Constructs
#### Combinators
- Combinators are functions that take a function as an argument and apply them somewhere in their body

#### Computational Reflection
- Three dimensions of reflection
    - ante/post computation (generation/quotation)
        - "ante" being some program that modifies/generates code
        - "post" being the modified/generated code that is ran, represented as data or actual code
        - this can be implemented via a macro system + `eval/reval` iso on a code-as-data structure
            - could these be one and the same via an abstract syntax tree type?
    - hypo/hyper computation (implementation/interpretation)
        - "hypo" being some program that implements lower-level constructs in some "hypercomputation"
            - e.g., a program that decides what kind of memory allocation scheme a program uses
            - e.g., a program that decides which effect handlers are used by the program it implements
        - "hyper" being the program that a "hypocomputation" implements
        - this can be implemented via evaluative reflection + implementation protocols
            - hyperprograms can send arrow arguments as messages to a hypoprogram running on a separate process
                - the hypoprogram can dynamically decide which I/O arrows to apply
                - this corresponds to `simulate`ing a program and its side effects
    - fore/back computation (manifestation/control)
        - "fore" being the base program that a user wants to run and interact with
        - "back" being the metaprogram that controls the "foreprogram"
            - a 'degenerate, one-shot' example would be a config file that is loaded at runtime
        - this can be implemented via a more flexible arrow system
            - controller process can send live control inputs to another running process
- Migration
    - updating code in a process while it is still running
    - moving a process from one machine to another while it is still running
- What does "code-as-data" look like in Kayak?
    - need an AST data type, likely an inductive of a sum type
    - should we rewrite the VM to operate on this type?
- How will code be able to modify itself while its running?
    - reify itself into AST, make changes and then reflect into the AST?
- How can implement the three dimensions of reflection in a reversible manner?
- Should reflection be restricted to equivalences between programs?

#### Reversible Concurrency + Distributed Computing
- Implement with with actor model send/receive arrows
    - this is more robust and extensible than the signal model
    - each message needs to have its sender id associated with it to make this reversible
    - each process has a list of process ids with which it can use to communicate
        - process ids can in effect act as capabilities (syndicated actor model)

#### Arrows
- Arrows generally can be compiled down from a higher-level language into the VM bytecode, given that the language runtime abstracts these information effects
    - example: deleting information sends the data to an "information bank"
- The exception would be I/O operations, such as reading/writing from a filesystem or sending/receiving data over a network; these arrows are compiled down to calls to machine-defined functions

#### Optical Programming
- Optics (lenses, prisms, etc.) are the way to go for high-level reversible programming
- "Optics and Type Equivalences" explores them in relation to James' calculus
- Also describes 'equivalences between equivalences'
    - Is not true reflection because it is only between functions that are structurally equivalent, not just functions that have the same type
    - Would be useful for optimizing though, explored more thoroughly in the "Embracing the Laws of Physics" paper

#### Delimited Continuations
- yield/run

#### Coroutines
- implemented via negative types