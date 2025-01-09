## Kayak Standard Library
- `trace f`
    - implements looping
    - compiles to `ZEROI +{ID + EXPN}+ +{f + ID}+ +{ID + COLN}+ ZEROE`, `f` being the function used in the loop 
- `sym f`
    - inverts the following function
    - compiles to `UNCALL` or the inverse of `f` if `if` is a core operator