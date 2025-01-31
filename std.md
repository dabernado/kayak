## Kayak Standard Library
### Syntax (defined in compiler)
- expressions
    - `( )`

- list comprehension

- signals
    - `^x`

- constants
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
        KEY = -24
        ```

    - products:
    ```
    NUM_PAIR :: (nat * nat)
    NUM_PAIR = (4, 6)
    ```

    - lists:
    ```
    NUM_LIST :: [nat]
    NUM_LIST = [0, 1, 2, 3, 4]
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
- `release ^x`, `bind ^x`
    - `^` symbol denotes a name
    - if called without a symbol name and current value in scope is a string, the string is used as the signal name
- `spawn f ^x`, `return f ^x`
- `( | )` (sum combinator)
- `( , )` (product combinator)

### Core Combinators (defined in compiler)
- `sym f`
    - inverts the following function
    - compiles to `UNCALL` or the inverse of `f` if `f` is a core function

### Core Arrows (dynamically defined in compiler)
- `spawnArr a ^x`, `returnArr a ^x`
    - spawn + return processes which run arrows

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

### Functions

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

### Arrow Combinators
- `second a`
    - defined as `(arr swapp) >> first a >> (arr swapp)`
- `right a`
    - defined as `(arr swaps) >> left a >> (arr swaps)`

### High-level Language Constructs
#### Combinators
- Combinators are functions that take a function as an argument and apply them somewhere in their body

#### Computational Reflection
- What does "code-as-data" look like in Kayak?
    - `eval` function?
- How will code be able to modify itself while its running?
- How can we generate new code in a reversible manner?
- Should reflection be restricted to equivalences between programs?

#### Reversible Concurrency + Distributed Computing
- Implement with signals + machine-defined arrows
    - add a `RLSE/BIND` instruction which can send/receive a value of some type
        - `RLSE` will release a signal of type `?a` with a name of `x`, removing it from the program's data structure
        - `BIND` will receive a signal of name `x`, remove its name so that it can be re-used, and introduce its value into the program's data structure
        - how to make this asynchronous?
            - will make `BIND` wait for its signal to be released before continuing execution at first
            - but we can probably make it async in the future
    - can add a `spawn c` function which will compile down to a `RLSE x` on its argument and a `SPAWN f x` on the signal
        - when a new process is spawned, it binds its argument signal before passing it to its program, and when the program finishes it releases its result to the same signal name
        - the inverse of `SPAWN f x` is `RETURN f x`, which waits for the output signal of the process to be released before proceeding with a `BIND` on the signal
        - `SPAWN` copies the execution direction of the current process to the new one, so when executing in reverse and a `RETURN` is flipped into a `SPAWN`, the new process also runs backwards
    - arrows for communicating with remote processes should be provided by the VM and exposed in the standard library
        - compiles down to `CALL/UNCALL` ops on VM-defined functions
            - VM will need to implement practices for making these as reversible as possible, e.g. copying + storing values that are passed to these functions
        - it is probably a good thing that abstractions for local concurrency and distributed computation are separate, as the first does not introduce effects and the second one does

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