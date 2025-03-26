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
 *
 * n = number of types in each side of the sum

START <-> END		  		: a <-> a
 * Denotes start/end of function; operationally equivalent to ID

SEND <-> RETR				: (int * a) <-> int
 * SEND: Sends a message to another process
 * RETR: Retracts a previously sent message, causing the other process to backtrack to the point of reception
 * The id could be a machine-defined function (negative values), another process, or an IPv6 address in integer form
 * All arrows are implemented via this instruction via operating on a product type, containing the name of the arrow and its input value

RECV <-> RETN				: int <-> (int * a)
 * RECV: Dequeues a message from the process' mailbox that was send by the process with id of int
 * RETN: Returns a received message to the original sender
```

### Type Restrictions
- Functions cannot be defined with negative types as input/output types

### Processes
- Each process object contains:
	- The data structure it operates on
	- An AST value containing the code it executes
	- The process id from which it was spawned
- A process cannot kill another process which it did not spawn
- All capabilities are contained within the process' local data structure

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
