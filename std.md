## Kayak Standard Library
### Syntax Examples
- expressions
    - `(swapp checkVals)`

- numeric literals
    - any number `n` is a function of the type `Nat <-> Nat`, which is composed of `n` folds
        - example: `8` has type of `Nat <-> Nat` and its code is just 8 folds in succession
        - example: `-5` is the function of `5` combined with the sym combinator, meaning it acts as 5 unfolds on a Nat

- constants
    - constants are defined as functions with type `1 <-> a`
    - a sum type constant's function uses the `just` function at its core to create it, via other functions which are based on `just`
        - example: a constant of `false` is just a `just`, while `true` is a `just swaps`
        - example: the `injectRight` function has a type of `a <-> a + a` and is implemented as follows:
        ```
        injectRight :: a <-> (a + a)
        injectRight = uniti (just, id) distrib (unite | unite)
        ```
    - isorecursive type constants compile down to use the combinator `make`
        - example: `make "test"` has a type of `1 <-> String`
    - adding the sym character before a constant implements its inverse
    - constant delimiters
        - strings: ""
        - nats/integers: (-)n
        - chars: '
        - lists: []
        - quotes: \`\`

    - nats:
    ```
    ID :: 1 <-> Nat
    ID = 8
    ```
        - chars:
        ```
        CHAR :: 1 <-> Char
        CHAR = 'h
        ```

    - sums:
    ```
    SUM_RIGHT :: 1 <-> (Nat + 1)
    SUM_RIGHT = right id
    ```
        - booleans:
        ```
        FLAG :: 1 <->> Bool
        FLAG = false
        ```
        - integers:
        ```
        KEY :: 1 <-> Int
        KEY = -24
        ```

    - products:
    ```
    NUM_PAIR :: 1 <-> (Nat * Nat)
    NUM_PAIR = uniti (4, 6)
    ```

    - lists:
    ```
    NUM_LIST :: 1 <-> [Nat]
    NUM_LIST = [0 1 2 3 4]
    ```
        - strings: 
        ```
        USER :: 1 <-> String
        USER = "username"
        ```
    
    - quotations:
    ```
    MY_CODE :: 1 <-> AST
    MY_CODE = `swaps (id | verify)`
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
    trace f = zeroi (expn | id) assocrs (id | f) assocls (coln | id) zeroe
    ```

- arrow definitions
    ```
    deleteVal :: a ~> 1
    deleteVal = ~myFunc >> delete
    ```
- type definitions
    - any type name which stars with a lowercase character is instantiated as a polymorphic type variable
        - type errors come from inside a function where operations are attempted on an unknown type
    ```
    Map :: [Key * Value]
    Result t e :: (t + e)   // with polymorphism
    ```
- object capabilities
    ```
    sendTCP :: (a * (Int.NetWrite * Int.NetRead)) ~> (Result * (Int.NetWrite * Int.NetRead))
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
- macros
    - ???

### Core Types (defined in compiler)
- `0` (zero)
- `1` (unit)
- `a <-> b` (isomorphisms)
- `a ~> b` (arrows)
- `a + b` (sums)
- `a * b` (products)
- `-a` (negative types)
- `1/a` (fractional types)
- `?a` (unobserved values)
- `[a]` (lists)
- `Nat` (natural numbers)

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
- `expn`, `coln`
- `expf`, `colf`
- `fold`, `unfold`
    - fold/unfold examples with values:
    ```
    1 + Nat <-> Nat:
        left () <-> 0

    1 + [Nat] <-> [Nat]:
        left () <-> []
        right (5, []) <-> [5]
    ```
- `quote`, `dequote`
    - converting code from a `Func a b` to an `AST`, and vice versa
    - needs to be a core instruction?
        - can't execute the code without introducing it into a process' AST even with higher-order functions

### Core Combinators (defined in compiler)
- `( | )` (sum combinator)
- `( , )` (product combinator)
- `<-f`
    - sym combinator, inverts the following function
    - compiles to the function's AST inverse or the inverse of `f` if `f` is a core function

### Core Arrows (dynamically defined in compiler)

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

### Types
- `Int :: (Nat + Nat)`
- `Char :: Nat`
- `Bool :: (1 + 1)`
- `String :: [Char]`
- `Func a b :: (1/a * ?b)`
    - higher order functions as represented by a fraction/unobserved value pair

### Functions
- `just`
    - injects a value of type `a` into a type of `1 + a`
    - defined as ???
    - important: `-just` on a value of `left ()` is non-terminating
- `add`, `sub`
    - takes a product of two nats and adds/subtracts the first nat to/from the second one
- `true`
    - takes unit value and turns it into boolean value of true
    - defined as `just swaps`
- `false`
    - takes unit value and turns it into boolean value of false
    - defined as `just`
- `send`, `recv`
    - abstraction over message passing via a communication channel established by a fraction/unobserved value pair
    - defined as ???

### Combinators
- `trace f`
    - additive trace, implements looping
    - defined as `zeroi (expn | id) assocrs (id | f) assocls (coln | id) zeroe`
- `search f`
    - multiplicative trace, implements constraint search
    - defined as `uniti (expf, id) assocrp (id, f) assoclp (colf, id) unite`
- `neg f`
    - transforms the type of `f` from `a <-> b` to `-a <-> -b`
    - defined as `???`
- `inv f`
    - transforms the type of `f` from `a <-> b` to `1/a <-> 1/b`
    - defined as `???`
- `lift f`
    - takes `f` and lifts it into a higher-order function, has type of `1 <-> Func a b`
    - defined as `expf (id, f)`
- `if-else (f) (g)`
    - if-statement which takes `(Bool * a)` as an argument, and applies f on a if the bool is `true` and g if the bool is `false`
    - defined as `distrib ((id, f) | (id, g)) factor`
- `left`, `right`
    - takes the following constant and wraps it in a left-handed or right-handed sum value
    - defined as `f just swaps` and `f just`, respectively
- `make v`
    - creates an isorecursive type of value `v` out of a unit value; `v` is not a function and must be value notation
    - defined as ???
    - example: injecting a `String` into a `(String + AST) + ((AST + AST) * AST)` when compiling text into an `AST` type:
    ```
    stringToOp :: String <-> (String + AST) + ((AST + AST) * AST)
    stringToOp =
        just swaps
        (id | make [])
        just swaps
        (id | make [])
        (id | uniti)
        (id | (just, id))
        (id | ((make [] | make []), id))

    ```
        - `just swaps` on the string to get `String + 1`
        - `make []` on right hand of sum combinator to get `String + AST`
        - `just swaps` to get `(String + AST) + 1`
        - `make []` on right hand of sum combinator to get `(String + AST) + AST`
        - `uniti` on right hand of sum combinator to get `(String + AST) + (1 * AST)`
        - `just` on first part of product combinator to get `(String + AST) + ((1 + 1) * AST)`
        - `make []` on both hands of the inner sum combinator to get `(String + AST) + ((AST + AST) * AST)`
        - note: not important what constant is provided to `make` since that arm of the computation will not be executed because we already have a value of `String`, soon to be `left (String)`
- `shift k e`, `reset`
    - `shift` captures `e` as a delimited continuation and binds it to function name `k`
    - `reset` delimits the continuation
    - defined as ???

### Arrows
- `create`, `erase`
    - creates or deletes a value of an arbitrary type
    - all constants are based on the use of the `create` arrow
    - depending on the hypocomputation, `create` and `erase` may be implemented as a `recv` or `send` either from/to another process or from/to create/erase functions implemented by the VM
    - `Int.Create` and `Int.Erase` capabilities needed for these arrows?

### Arrow Combinators
- `second a`
    - defined as `(arr swapp) >> first a >> (arr swapp)`
- `right a`
    - defined as `(arr swaps) >> left a >> (arr swaps)`

### Object Capabilities
- Object capabilities are a reference to some kind of resource represented as a type value, usually `Int`
    - They are represented in type annotations by `.Cap` format, though this has no real operational semantics and is mainly used for readability
- Capabilities are only used in arrows, especially I/O arrows
- An `Int` value passed as a capability to an arrow can reference either a VM-defined function or the mailbox of another process
    - VM function ids are always negative, while process ids are always positive
    - The programmer can check if a capability is a function id or a process id via the sum combinator
- VM function ids and process ids are interchangable, allowing for processes to be sandbox by other controlling processes
    - example: a running process may utilize `reflect` but wasn't granted a capability which references the VM reflect function, so it instead uses the id of a process which does have that capability and which can handle the request of the sandboxed process however it likes
- List of object capabilities:
    - `Int.Kill`: killing a child process
- Does the programmer need to be able to define its own object capability types?

## High-level Language Constructs
### Computational Reflection
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

### Coroutines
- implemented via negative types