# Language Spec
### Types
```
0	:= empty type
1	:= unit type

a <-> b := isomorphism type
(a * b) := product type
(a + b) := sum type
μx.[a]   := induction type
-a       := negative type

nat  := μx.[1 + x]
int  := (nat + nat)
bool := (1 + 1)
list := μx.[1 + (a * x)]
```

### Functions
```
ID <-> ID         : a <-> a
 * Identity; does nothing

ZEROI <-> ZEROE   : a <-> (0 + a)
 * Introduce/eliminate sum variant of type 0

SWAPS <-> SWAPS   : (a + b) <-> (b + a)
 * Swap the two variant types' sides

ASSRS <-> ASSLS   : ((a + b) + c) <-> (a + (b + c))
 * Associate inner sum with types on the right or left

UNITI <-> UNITE   : a <-> (1 * a)
 * Introduce/eliminate product with unit type

SWAPP <-> SWAPP   : (a * b) <-> (b * a)
 * Swap the first and second values

ASSRP <-> ASSLP   : ((a * b) * c) <-> (a * (b * c))
 * Associate inner product with types on the right or left

DIST <-> FACT     : ((a + b) * c) <-> ((a * c) + (b * c))
 * Distribute inner sum over both product values/Factor inner
 * sum into first value

FOLD <-> UFOLD    : μx.[a/b]b <-> μx.[a]
 * Fold/unfold value into/out of an induction type
 * n = size of inductive type elements
```

### Combinators
```
+(
 - Sum combinator start

+
 - Delimits the two halves of a sum combinator
 - Not an actual instruction

*(
 - Product combinator start

*
 - Delimits the two halves of a product combinator 
 - Not an actual instruction

)
 - Sum/Product combinator end
```

### Control
```
EXPN <-> COLN     	  		: 0 <-> (-a + a)
 * Reverse type sign and direction of execution
 *
 * n = number of types in each side of the sum

RLSE x <-> BIND x	  		: a <-> 1
 * Releases/receives a signal x of type a and removes it from/introduces it to the program's data
 *
 * x = name of signal

START <-> END		  		: a <-> a
 * Denotes start/end of function; operationally equivalent to ID

CALL f <-> UNCALL f	  		: a <-> b
	where f: a <-> b
 * Invoke function forwards/backwards on datatype
<<<<<<< HEAD
 * f = name of invoked function, translated to start + end indices in bytecode

SPAWN f x <-> RETURN f x	: a <-> a
 * Spawns a new process as running a function f with a signal as its argument; as inverse, waits for some process to finish and bind its result to a signal
 *
 * f = function for the new process to run
 * x = name of signal to be used as argument to f
```

### Type Restrictions
- Functions cannot be defined with negative types as input/output types

## Instruction Encoding
```
 * I-Type
 *
 * 31                                     0
 * [            imm            ] [ opcode ]
 *	            27b		               5b
 *
 * Instructions that do not contain additional information or
 * contain a constant value are represented by the I-Type
 * encoding. All IRIS instructions are I-Type encoded.
```
