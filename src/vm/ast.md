# Kayak Abstract Syntax Tree (AST) Data Type

```
AST :: [(String + AST) + ((AST + AST) * AST)]
Program :: [String * (AST * [String])]
```
## Program
- The program type is just a map from function names (`String`) to their code (`AST`, with `[String]`)
    - The `[String]` holds the names of functions supplied to combinators in their codebase
        - For example, `if` has an arity of 2, so the length of the `[String]` will be 2
        - Assuming the supplied functions to `if` are referenced as `f` and `g` in its code, then the `[String]` would have a value of `["f", "g"]`
        - For normal functions, it will be an empty list

## AST
### String: Functions and Combinators
- Functions and combinators are introduced in the AST as strings of their names, which are then executed via lookup in the `Program`'s map
    - Core functions are also defined in the AST this way, but are not present in the function map as they are executed directly by the VM

### AST: Expressions
- Concatenated functions inside parenthesis which constitute a single expression

### (AST + AST) * AST: sum/product combinator
- The sum type on the left contains an AST
    - if it is a `left AST`, the combinator is a sum combinator
    - if it is a `right AST`, the combinator is a product combinator
- The AST in the second value of the product is the second piece of code contained in the combinator
    - If a sum combinator, the VM will choose which of the two ASTs to execute
    - If a product combinator, both ASTs will be executed concurrently

### Char: separator
- Equivalent to an `id` op, but need to be included in the AST for fully reversible compilation

### String: comments
- Equivalent to an `id` op, but need to be included in the AST for fully reversible compilation
    

### ([Nat * String] * AST): Combinator
- the `[Nat * String]` is a map from the combinator's arity to what the supplied functions are named in its code
    - e.g., in the combinator `if f g`, 0 would map to `f` and 1 would map to `g`
- the `AST` is the combinator's code
- this will require a special combinator execution context inside the VM to hold the arity map

## Metadata
- All information in the original code needs to be preserved in order to enable fully reversible compilation between code-as-text and the AST
- How to make code-as-text reversible between the AST?
    - Rethink the "code editor"; programmer must be editing the AST itself
        - no text, perhaps circuitry instead?

### Type Information
- Likely will be represented by a `[String * Type]`, `String` being the name of the type/function/arrow and `Type` representing its actual type
- How to implement `Type`?
    ```
    Type :: (
        (String + (1 + 1))                  // (User-defined type + (1 + 0))
        + ((Type * Type) + (Type * Type))   // (Product + Sum)
        + ((Type * Type) + (Type * Type))   // (Iso + Arrow)
        + (Type + Type)                     // (-a + 1/a)
        + Type                              // Recursive
    )
    ````

### Separators
```
Separator :: [(Nat * Nat) * String]
```
- `Nat * Nat` represents line and column where separator begins
- `String` is the separator which could be a space, tab, newline, comment, or combination of any/all four