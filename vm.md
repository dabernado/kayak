# Kayak Virtual Machine

## Requirements
- operates directly on capnp types
    - must encode kayak type system in a capnp protocol
- orthogonal persistence
    - need only to persist data at significant events, such as input/output of a program
        - as long as kayak is deterministic, any previous state can be recovered
    - also should persist IO data
- lightweight concurrency
    - product combinators must spawn concurrent subcomputations
        - product subcomputations that are just `id` can be optimized away
    - just use threads
        - much simpler and more performant
        - spawn new thread for second value, eval first value in the current thread
        - easier integration with cap-std
        - linux scheduler can handle tens of thousands of threads
        - not available on all platforms, embedded devices may lack multithreading
- needs to be a first class implementation
    - kayak small-step semantics should be observed
        - we don't need a representation of the small-step machine at the abstract level
        - as long as vm nodes are observable, we can convert to one whenever necessary
    - kayak abstract machine data structure will sit on top of the vm
    - vm as a "metaprogram"
    - what properties will it have?
        - observability
            - absolutely required
            - will be the main thing that distinguishes this platform from others
            - as long as reversibility is preserved between abstract and concrete nodes, we should be good
        - completeness; the conjunction of
            - totality
                - all nodes in the abstract are implementable in the concrete
                - this should be pretty easy
            - fullness
                - all transitions in the abstract are implementable in the concrete
                - also should be pretty easy
        - liveness
            - might be tough since reversible computations aren't considered as "advancing"
            - arrow computations are though, so we could define it in those terms
        - co-liveness
            - all terminal objects in the abstract are also terminal in the concrete
            - should be pretty easy to implement

- ast vs bytecode
    - ast
        - pro: easy to manipulate as data in Kayak
        - pro: concrete nodes are directly observable
        - con: VM is basically an interpreter at this point
    - bytecode
        - pro: good way to standardize a reversible ISA
        - pro: more efficient
        - pro: concrete and abstract computations should be separate anyways
        - con: cannot manipulate as data in a kayak program
            - is this even necessary for full reflection?
- process state machine needs to be a first-class type in some form
    - necessary for the following use cases:
        - first-class implementations
        - hot code migration
        - migrating processes to a meta-circular interpreter
        - going step-by-step through a process and inspecting its state
        - virtualization
    - dont need to have a value of this type at all times, as long as VM state machine is observable

## Processes
- Each process object contains:
	- The data structure it operates on
	- A program containing the code it executes
	- A state machine for executing the AST, including a context stack and AST index

### Data Strucure
- Implemented as a Capnp data type for easy serialization/deserialization
- All capabilities are contained within the process' local data structure as first-class objects

### Program
- A Kayak data type itself which contains all the functions used in the program's codebase

### State Machine
- The state machine constitutes the Rust data structures which the VM uses to execute the code

## Kayak Image Format
- A Kayak image is a serialized form of an entire process, including the program, data structure and state machine
- When building source code into an image, the state machine will always start at index 0 with an empty context stack; the starting data structure can be one of two options:
    - A previously created Kayak type value distributed with the program
    - Nothing, which will prompt the user to supply a value before running the program
- Kayak images can be written to a file or passed over a network connection
    - entire processes can be serialized while they are running and migrated to a different platform via these interactions

## MVP
- The Minimum Viable Product virtual machine should be able to:
    - parse Kayak source code into an AST data structure
    - implement all functions and combinators specified in the **Core** sections in ./std.md
        - no need to worry about arrows for now
    - operate directly on capnp types
    - load Kayak image files, execute them, and output the result as an image file
    - execute product combinators sequentially (we can add concurrency later)
- Once this is done, the next steps are to:
    - implement arrows + capabilities
    - write the standard library
    - make product combinators execute concurrently
    - implement first-class implementations
        - Kayak state machine needs to be a first-class value