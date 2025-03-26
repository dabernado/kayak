## Kayak Standard Library
### Syntax (defined in compiler)
- numeric literals
    - any number `n` is a function of the type `nat <-> nat`, which is composed of `n` folds
        - example: `8` has type of `nat <-> nat` and its code is just 8 folds in succession
        - example: `-5` is the function of `5` combined with the sym combinator, meaning it acts as 5 unfolds on a nat

- constants
    - constants are defined as functions with type `1 <-> a`
    - a sum type constant's function uses the `just` function at its core to create it, via other functions which are based on `just`
        - example: a constant of `false` is just a `just`, while `true` is a `just swaps`
        - example: the `injectRight` function has a type of `a <-> a + a` and is implemented as follows:
        ```
        injectRight :: a <-> (a + a)
        injectRight = uniti (just, id) distrib {unite, unite}
        ```
    - isorecursive type constants compile down to use the combinator `make`
        - example: `make "test"` has a type of `1 <-> str`
    - adding the sym character before a constant implements its inverse
    - constant delimiters
        - strings: ""
        - nats/integers: (-)n
        - chars: '
        - lists: []
        - everything else: \`\`

    - nats:
    ```
    ID :: 1 <-> nat
    ID = 8
    ```
        - chars:
        ```
        CHAR :: 1 <-> char
        char = 'h
        ```

    - sums:
    ```
    SUM_RIGHT :: 1 <-> (nat + 1)
    SUM_RIGHT = `right ()`
    ```
        - booleans:
        ```
        FLAG :: 1 <->> bool
        FLAG = `false`
        ```
        - integers:
        ```
        KEY :: 1 <-> int
        KEY = -24
        ```

    - products:
    ```
    NUM_PAIR :: 1 <-> (nat * nat)
    NUM_PAIR =  `(4, 6)`
    ```

    - lists:
    ```
    NUM_LIST :: 1 <-> [nat]
    NUM_LIST = [0 1 2 3 4]
    ```
        - strings: 
        ```
        USER :: 1 <-> str
        USER = "username"
        ```

- function definitions
    ```
    myFunc :: a <-> b
    myFunc = swapp changeVal zeroi
    ```

- combinator definitions
    ```
    trace f :: b <-> c
        where f :: (a + b) <-> (a + c)
    trace f = zeroi {expn, id} assocrs {id, f} assocls {coln, id} zeroe
    ```

- arrow definitions
    ```
    deleteVal :: a ~> 1
    deleteVal = ~myFunc >> delete
    ```
- type definitions
    - any type name which cannot be resolved is automatically instantiated as a polymorphic type variable
        - type errors come from inside a function where operations are attempted on an unknown type
    ```
    Map :: [str * nat]
    Result :: (t + e)   { with polymorphism }
    ```
- object capabilities
    ```
    sendTCP :: (a * (int.NetWrite * int.NetRead)) ~> (Result * (int.NetWrite * int.NetRead))
    sendTCP = ...
    ```
- comments
    ```
    // Single-line comment

    /*
     * Multi-line
     * comment
    */
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
- `absorbz`, `factorz`
- `distrib`, `factor`
- `fold`, `unfold`
    - fold/unfold examples with values:
    ```
    1 + nat <-> nat:
        left () <-> 0

    1 + [nat] <-> [nat]:
        left () <-> []
        right (5, []) <-> [5]
    ```
- `expn`, `coln`

### Core Combinators (defined in compiler)
- `{ , }` (sum combinator)
- `( , )` (product combinator)
- `-f`
    - sym combinator, inverts the following function
    - compiles to the function's AST inverse or the inverse of `f` if `f` is a core function
- `reflect f`, `reify f`
    - has type `(1 + Error) <-> (AST + Error)` and `(AST + Error) <-> (1 + Error)`, respectively
    - `reflect` takes `f` and introduces its AST into the data structure as the first value of a product type
        - `f` is removed from the process' AST and cannot be called after reflection until it is reified again
    - `reify` takes an AST and binds it as a function to the name `f`
    - how to implement??
        - likely needs its own instruction

### Core Arrows (dynamically defined in compiler)
- `send`, `recv`
    - takes a product of a processes' address and a value
    - `send` sends the value as a message to the process, replacing it with a unit value
    - `recv` deques a message out of the processes mailbox and replaces a unit value with it
- `create`, `erase`
    - creates or deletes a value of an arbitrary type
    - all constants are based on the use of the `create` arrow
    - depending on the hypocomputation, `create` and `erase` may be implemented as a `recv` or `send` either from/to another process or from/to create/erase functions implemented by the VM
        - use of these VM functions require the process to own `Create` and `Erase` capabilities, respectively

### Core Arrow Combinators (defined in compiler)
- `>>`
    - composes two arrows together
    - example: `create >> delete`
- `~f`
    - changes type of iso `f` to an arrow so that it can be sequenced with arrows
- `first a`
    - applies arrow `a` to the first value of a product type
- `left a`
    - applies arrow `a` to a left value of a sum type

### Functions
- `just`
    - injects a value of type `a` into a type of `1 + a`
    - defined as ???
    - important: `-just` on a value of `left ()` is non-terminating
- `add`, `sub`
    - takes a product of two nats and adds/subtracts the first nat to/from the second one

### Combinators
- `trace f`
    - additive trace, implements looping
    - defined as `zeroi (expn | id) assocrs (id | f) assocls (coln | id) zeroe`
- `neg f`
    - transforms the type of `f` from `a <-> b` to `-a <-> -b`
    - defined as `???`
- `if-else (f) (g)`
    - if-statement which takes `(bool * a)` as an argument, and applies f on a if the bool is `true` and g if the bool is `false`
    - defined as `distrib ((id, f) | (id, g)) factor`
- `make v`
    - creates an isorecursive type of value `v` out of a unit value; `v` is not a function and must be value notation
    - defined as ???
    - example: injecting a `str` into a `str + ((AST + AST) * AST)` when compiling text into an `AST` type can be done via the following steps:
        - `just swaps` on the string to get `str + 1`
        - `make []` on right hand of sum combinator to get `str + AST`
        - `uniti` on right hand of sum combinator to get `str + (1 * AST)`
        - `just` on first part of product combinator to get `str + ((1 + 1) * AST)`
        - `make []` on both hands of the inner sum combinator to get `str + ((AST + AST) * AST)`
        - note: not important what constant is provided to `make` since that arm of the computation will not be executed because we already have a value of `str`, soon to be `left (str)`

### Arrows

### Arrow Combinators
- `second a`
    - defined as `(arr swapp) >> first a >> (arr swapp)`
- `right a`
    - defined as `(arr swaps) >> left a >> (arr swaps)`
- `spawn a`
    - has type `(a * AST) ~> (int * int.Kill)`
        - `(a * AST)` represents the data the new process should operate on and the program it should execute
        - `int` is the ID of the new process
        - `int.Kill` is an int value representing a one-time capability required for killing the process and returning its data
    - defined as:
    ```
    spawn a :: (a * AST) ~> (int * int.Kill)
        where a :: a ~> b
    spawn = ~uniti >> first n >> send >> recv >> first -n >> ~unite
    ```
        - `n` being the id of the VM-defined function for spawning new processes
    - when creating a process, the VM builds a `Program` by copying `a`'s AST and creates the new process with it
    - the VM grants the current process a `int.Kill` capability after spawning the new one
- `kill`
    - has type `int.Kill ~> (a * AST)`

### High-level Language Constructs
#### Combinators
- Combinators are functions that take a function as an argument and apply them somewhere in their body

#### Computational Reflection
- Three dimensions of reflection
    - ante/post computation (generation/quotation)
        - "ante" being some program that modifies/generates code
        - "post" being the modified/generated code that is ran, represented as data or actual code
        - this can be implemented via a macro system + `reflect/reify` iso on a code-as-data structure
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
- Implement with with actor model send/receive arrows...
    - this is more robust and extensible than the signal model
    - each message needs to have its sender id associated with it to make this reversible
    - each process has a list of process ids with which it can use to communicate
        - process ids can in effect act as capabilities (syndicated actor model)
- ...or rstructures-style named signals?

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