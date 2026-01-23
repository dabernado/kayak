## Kayak Standard Library
### Syntax Examples
- expressions
    - `(swapp checkVals)`

- constants
    - constants are defined as functions with type `1 <-> a`
    - the combinator `const` takes an AST with the constant defined in it, and creates the value using `just`
        - therefore, constants are just a DSL embedded in Kayak
    - a sum type constant's function uses the `just` function at its core to create it, via other functions which are based on `just`
        - example: a constant of `false` is just a `just`, while `true` is a `just swaps`; hence, they are just functions and can be used as such
        - example: the `injectRight` function has a type of `a <-> a + a` and is implemented as follows:
        ```
        injectRight :: a <-> (a + a)
        injectRight = uniti (just, id) distrib (unite | unite)
        ```
    - isorecursive type constants use the combinator `const`
        - example: `const "test"` has a type of `1 <-> String`
    - adding the sym character before a constant implements its inverse and is used for "deallocating" constants
    - constant delimiters
        - strings: ""
        - nats/integers: (-)n
        - chars: '
        - lists: []
        - quotes: \`\`

    - nats:
    ```
    ID :: 1 <-> Nat
    ID = const `8`
    ```
        - chars:
        ```
        CHAR :: 1 <-> Char
        CHAR = const `'h`
        ```

    - sums:
    ```
    SUM_RIGHT :: 1 <-> (Nat + 1)
    SUM_RIGHT = const `right ()`
    ```
        - booleans:
        ```
        FLAG :: 1 <-> Bool
        FLAG = false
        ```
        - integers:
        ```
        KEY :: 1 <-> Int
        KEY = const `-24`
        ```

    - products:
    ```
    NUM_PAIR :: 1 <-> (Nat * Nat)
    NUM_PAIR = const `(4, 6)`
    ```

    - lists:
    ```
    NUM_LIST :: 1 <-> [Nat]
    NUM_LIST = const `[0 1 2 3 4]`
    ```
        - strings: 
        ```
        USER :: 1 <-> String
        USER = const `"username"`
        ```
    
    - quotations:
    ```
    MY_CODE :: 1 <-> AST
    MY_CODE = `swaps (id | verify)`
    ```
        - for quotations, the interpreter will make a recursive call to itself to transform the text into an AST data type
        - antiquotations are applied to the function's argument and splice the result into the AST
            - antiquoted expressions must return an AST
        - antiquotations:
        ```
        splice :: String <-> AST
        splice = `uniti swapp (^modify, just swaps)`
        ```
        - ASTs can be added together simply by using the `concat` function for two lists
        ```
        makeCode :: (AST * AST) <-> AST
        makeCode =
            (
              uniti
              swapp
              (id, `some code here`)
              concat
            , id
            )
            concat
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
    deleteVal :: (a * a) ~> 1
    deleteVal = first (arr myFunc) >> second myArrow >> delete
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
    sendTCP :: ((NetWrite * NetRead) * a) ~> Result
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
- `-a` (negative types)
- `1/a` (fractional types)
- `?a` (unobserved values)
- `[a]` (lists)
- `Nat` (natural numbers)
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
- `eval`, `reval`
    - Evaluate an AST forwards or in reverse

### Core Combinators (defined in compiler)
- `( | )` (sum combinator)
- `( , )` (product combinator)
- `~f`
    - sym combinator, inverts the following function
    - compiles to the function's AST inverse or the inverse of `f` if `f` is a core function

### Core Arrows (dynamically defined in compiler)
- `create`, `erase`
    - creates or deletes a value of an arbitrary type
    - all constants are based on the use of the `create` arrow
    - depending on the hypocomputation, `create` and `erase` may be implemented as a `recv` or `send` either from/to another subprogram or from/to create/erase functions implemented by the VM
    - `Create` and `Erase` capabilities needed for these arrows?

### Core Arrow Combinators (defined in compiler)
- `>>`
    - composes two arrows together
    - example: `create >> sendTCP`
- `arr f`
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
    - important: `~just` on a value of `left ()` is non-terminating
- `add`, `sub`
    - takes a product of two nats and adds/subtracts the first nat to/from the second one
- `true`
    - takes unit value and turns it into boolean value of true
    - defined as `just swaps`
- `false`
    - takes unit value and turns it into boolean value of false
    - defined as `just`
- `eval`, `reval`
    - applies quotation to a given argument
    - has type `(AST * a) <-> (AST * b)`
- `evalArr`
    - applies quoted arrow to a given argument
    - has type `(AST * a) ~> (AST * b)`
- `send`, `recv`
    - abstraction over message passing via a communication channel established by a fraction/unobserved value pair
    - defined as ???
- comparison operators
    - how to implement this?
    - should only need one for recursive types, will subsume nats and lists
        - at most we would only need one for each
    - also need one for bools, but this should be pretty straightforward to write

### Combinators
- `trace f`
    - additive trace, implements looping
    - defined as `zeroi (expn | id) assocrs (id | f) assocls (coln | id) zeroe`
- `search f`
    - multiplicative trace, implements constraint search
    - defined as `uniti (expf, id) assocrp (id, f) assoclp (colf, id) unite`
- `neg f`
    - transforms the type of `f` from `a <-> b` to `-a <-> -b`
    - defined as:
    ```
    zeroi
    (expn | id)
    assocrs
    (
      id
    | swaps
      coln
      (id | f)
      expn swaps
    )
    assocls
    (coln | id)
    zeroe
    ```
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
- `const \`v\``
    - creates an isorecursive type of value `v` out of a unit value; `v` is not a function and must be a quotation
    - equivalent to `make` in the James paper
    - defined as ???
    - example: injecting a `String` into a `(String + AST) + ((AST + AST) * AST)` when compiling text into an `AST` type:
    ```
    stringToOp :: String <-> (String + AST) + ((AST + AST) * AST)
    stringToOp =
        just swaps
        (id | const `[]`)
        just swaps
        (id | const `[]`)
        (id | uniti)
        (id | (just, id))
        (
          id
        | (
            (const `[]` | const `[]`)
          , id
          )
        )

    ```
        - `just swaps` on the string to get `String + 1`
        - `const []` on right hand of sum combinator to get `String + AST`
        - `just swaps` to get `(String + AST) + 1`
        - `const []` on right hand of sum combinator to get `(String + AST) + AST`
        - `uniti` on right hand of sum combinator to get `(String + AST) + (1 * AST)`
        - `just` on first part of product combinator to get `(String + AST) + ((1 + 1) * AST)`
        - `const []` on both hands of the inner sum combinator for `(String + AST) + ((AST + AST) * AST)`
        - note: not important what constant is provided to `const` since that arm of the computation will not be executed because we already have a value of `String`, soon to be `left (String)`
- `shift k e`, `reset`
    - `shift` captures `e` as a delimited continuation and binds it to function name `k`
    - `reset` delimits the continuation
    - defined as ???
- `spawn f g`
    - splices `f` and `g` into a quotation that establishes the necessary amount of typed communication channels between the functions listed before running them as subcomputations
        - searches through each subprogram to find `send` and `recv` calls to do this
    - subsumes use case of fore/back computations, as dividing up code between core logic and I/O handling is best practice
        - also subsumes sandboxing arrows, since the programmer will want to write their own I/O handlers for that too
    - how to handle message passing between more than two subcomputations?
        - build it like a tree; the parent computation of two communicating subcomputations can also communicate with another computation
- `spawnArr a b`
    - same as above, but with arrows instead of pure functions
- `sandbox a`
    - takes arrow which utilizes object capabilities and produces a pure function quotation with all I/O arrow calls replaced with `send` and `recv`, and that doesn't require any capabilities
    - resulting quotation is meant to be spawned with an I/O handler subprogram that is granted capabilities and handles the I/O requests however it sees fit

### Arrows
- `clone`
    - clones a value of type `a`, resulting in a value of type `(a * a)`

### Arrow Combinators
- `second a`
    - defined as `(arr swapp) >> first a >> (arr swapp)`
- `right a`
    - defined as `(arr swaps) >> left a >> (arr swaps)`

### DSLs
- How can we make it easy to build macros and DSLs in kayak?
- `@std/os`
    - DSL for interacting with an operating system runtime written in kayak
- `@std/rdp`
    - reactive demand programming DSL
    - `@std/rdp/build`: kayak build system DSL implemented on top of `@std/rdp`
- `@std/optics`
    - optical programming DSL

### Modules
- How to implement a module system?
    - Reread the houyhnhnm computing chapter on this

### Object Capabilities
- Object capabilities are a reference to some kind of resource represented as `(a * Nat)`
    - `a` is a reference to the resource represented by some type value, usually an `Int`
    - `Nat` is a UUID in number form which represents the capability itself and what permissions are granted to it
        - Modifying this value in any way will make it invalid
- Capabilities are only used in arrows, especially I/O arrows
- Capability values are first-class and are passed through a program's data flow with the rest of the data structures
- Does the programmer need to be able to define its own object capability types?
    - This will definitely be necessary for FFI
- How to grant/revoke capabilities?
    - Need to be able to grant more than one capability for a resource at a time
- Should we keep programs from using `clone` arrow on capability values?

### Modules
- How should we go about implementing a module system?
    - very important to consider: https://ngnghm.github.io/blog/2016/04/26/chapter-9-build-systems-and-modularity/
- The `Program` data type is basically already a module; perhaps it should be renamed
    - the only difference between a "program" and "module" is that the program has an `init` function

## High-level Language Constructs
### Reactive Demand Programming
- this seems so sick and perfect for what we're trying to do

### Computational Reflection
- Three dimensions of reflection
    - ante/post computation (generation/quotation)
        - "ante" being some program that modifies/generates code
        - "post" being the modified/generated code that is ran, represented as data or actual code
        - this can be implemented via a macro system + `quote/unquote` iso
    - hypo/hyper computation (implementation/interpretation)
        - "hypo" being some program that implements lower-level constructs in some "hypercomputation"
            - e.g., a program that decides what kind of memory allocation scheme a program uses
            - e.g., a program that decides which effect handlers are used by the program it implements
        - "hyper" being the program that a "hypocomputation" implements
        - this can be implemented via processing ASTs before an `unquote` on different platforms
            - e.g., migrating from linux to sel4, or to a distributed cluster
            - arrow calls can be matched against and replaced, as the VM essentially represents a hypocomputation already
    - fore/back computation (manifestation/control)
        - "fore" being the base program that a user wants to run and interact with
        - "back" being the metaprogram that controls the "foreprogram"
            - a 'degenerate, one-shot' example would be a config file that is loaded at runtime
        - this can be implemented via message passing between subcomputions
            - one subcomputation represents the forecomputation and receives all its inputs from the other
            - the other is the backcomputation and receives live inputs via arrow functions
- Hot code migration
    - Do we need some kind of versioning system for live code updates?

### Coroutines
- implemented via negative types

### Unit Tests/Proofs
- Is there a way to implement a proof system for Kayak?
    - easy and automatic formal verification would be huge