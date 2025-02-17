# Kayak Abstract Syntax Tree (AST) Data Type

```
AST :: μx.[1 + (OP * x)]
OP  :: (nat + (AST * AST) + (AST * AST) + AST + [nat] + (nat * Com_AST))
Com_AST :: μx.[1 + ((nat + OP) * x)]
```

## Op_AST
### nat: Core Function
- Isomorphisms in the core instruction set can simply be identified by a nat value

### (AST * AST): sum combinator
- Sum combinators contain two subprograms in which one or the other may be executed, so together they form a product type

### (AST * AST): product combinator
- Same as above, except both subprograms are executed concurrently

### AST: User-defined Function
- Called functions are compiled down to their AST representations and inlined in the program

### (nat + nat): spawn/return
- the `[nat]` value represents the list of indices into the capability store, representing the object-capabilites to be granted to the spawned process
- this is the only reason why `spawn/return` can't be included in the core calculus

### (nat * Com_AST): User-defined Combinator
- the `nat` represents the arity of the combinator
- `Com_AST :: μx.[1 + ((nat + OP) * x)]`
    - the `nat` in the `(nat + OP)` adds the possibility to substitute one of the functions applied to the combinator into its codebase
    - this will require a special combinator execution context inside the VM