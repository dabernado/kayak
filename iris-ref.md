# Language Spec
### Types
```
0	:= empty type
1	:= unit type

a <-> b := isomorphism type
(a * b) := product type
(a + b) := sum type
μx.[a]  := induction type
-a      := negative type
1/a		:= fractional type
?a		:= unobserved value

Nat  := μx.[1 + x]
Int  := (nat + nat)
Bool := (1 + 1)
List := μx.[1 + (a * x)]
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

ABSORB <-> FACTZ  : (0 * a) <-> 0
 * Eliminate/introduce arbitrary type in product with zero type; operationally equivalent to ID

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

EXPF <-> COLF     	  		: 1 <-> (1/a * ?a)
 * Create an unobserved value and its constraint

START <-> END		  		: a <-> a
 * Denotes start/end of function; operationally equivalent to ID
```

### Type Restrictions
- Functions cannot be defined with negative or fraction types in input/output

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
