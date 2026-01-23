# Notes on Fare's Paper

## First-Class Implementations

### Abstract Nodes and Arrows (Kayak)
- small-step semantics
- definition of node as a data structure is given in the james paper

### Concrete Nodes and Arrows (VM)
- the only transitions that can be considered "advancing" are ones with information effects
    - fare states in the paper that reversible, local operations are excluded from the advancing subcategory
    - though there are no rules on how its defined, so its up to the programmer at the end of the day

### First-Class Computations
- `=>`
    - denotes first-class computations seen as data recording potential changes and side-effects
    - being data, it’s mostly inert by itself, and its meaning fairly independent of the meta-system where the ante-computations take place
- `~>`
    - denotes actual computations seen as code to be performed with actual side-effects
    - being code with side-effects, its precise semantics may vary a lot depending on the meta-system used for ante-computations
- `->`
    - denotes pure total functions without side effects
- `A :: Node`
    - A `Node` represents the state of whatever computing environment (a lightweight process, the stack registers and heap of an assembly language, a scheme program's continuation, etc.) that is being simulated
- `A => B :: Arrow`
    - An `Arrow` is a transition from one state to another while it is executing, effectful or not
    - In our case, it represents all of the core isomorphisms, combinators and arrows, as well as any function/arrow that can be constructed from them
- `run :: A ~> (A => B)`
    - Takes a `Node` as an argument and executes it, producing producing an `Arrow` as the result
    - Important to note that `run` is typed as an arrow because it could produce side effects; it may also be non-terminating, return immediately without advancing at all, return some intermediate step, get interrupted, or return a final result
    - "a model of what advancing, completing or interrupting means is necessary to improve on that type"

- Notice how `step`, `advance` and `eval` all have the same type; what differentiates them are the logical guarantees they provide for their results
    - `step` produces an single-step `Arrow` that results either in a `Node` that is done, or an `Arrow` that is advancing
    - `advance` does the same, but with side effects
    - `eval` produces an `Arrow` which may contain many transitions, and results in a `Node` that is done

- `advancing? :: (Node => Node) -> Bool`
    - Determines if an `Arrow` is currently advancing, or is stuck
- `done? :: Node -> Bool`
    - Determines if a `Node` is in a finished state, or if there is some `Arrow` which can continue its execution
- `step :: Node -> (Node => Node)`
    - Advances a `Node` one step deterministically, returning an `Arrow`
- `advance :: Node ~> (Node => Node)`
    - Advances a `Node` by some unspecified amount until it halts, potentially producing side effects as well
- `eval :: Node ~> (Node => Node)`
    - Executes a `Node` until it reaches a state where it is done

### Evaluative Reflection
- Perhaps the most crucial function in this formalization is `perform`, which takes a recording of a computation and actually performs it with all its side effects
    - this is basically "lowering" the computation from the simulated realm into the metalanguage realm where it is actually executed
- There is also `simulate`, which does the opposite; it takes an instantiated computation and lifts it into a simulated representation
- `perform-node` and `simulate-state` already constitute an isomorphism, but `perform-arrow` and `simulate-arrow` do not; is there a way that they could?

- `State`
    - Represents the state of the computing environment used to execute the code (an operating system process, programming language thread, etc.)
    - In our case it represents a Kayak process state: the data structure, the program, and the execution state machine
- `perform-node :: Node -> State`
    - Instantiates a first-class representation of a computation node into an actual computation state
- `perform-arrow :: (A => B) -> (State ~> State)`
    - Realizes all the changes encoded in a first-class representation of computation arrow into an actual computation change with all its actual side-effects
- `simulate-state :: State -> Node`
    - Takes the state of a stopped process and freezes it as a first-class object that simulates that state in sufficient detail to perform it in the future, or otherwise reason about it
- `simulate-arrow :: (State -> (State ~> State)) ~> (Node => Node)`
    - Takes an initial state as well as a function to transform that state with side-effects, and returns a record of the internal changes and external side-effects that running this function would have on the state, without actually performing those effects
- `record-state`, `record-arrow`
    - A specialized version of `simulate` that performs all the effects of a computation, but still records what they were and what they did

- Similar to the first-class computations, an advancing protocol also exists for the first-class implementations that "actually" and "directly" run the code
    - `!run :: State ~> State`
    - `!step :: State ~> State`
    - `!advance :: State ~> State`
    - `!eval :: State ~> State`

### First-Class Implementations Protocol
- In this section, "Concrete" is a lower-level computation and "Abstract" is a higher-level computation
- `interpret` is a partial function between a concrete hypo-computation and an abstract hyper-computation
    - "Partial function" (`-/->`) meaning it is a total function of a subset of `Concrete` to `Abstract`, and could be expressed as `X -> Maybe Y` or something of the sort
- What is "totality"?
- What is "completeness"?
- What is "fullness"?
- What is "liveness"?
    - bounded liveness?
    - strong liveness?
    - bounded-strong liveness?
    - strong-step preservation?
- What is "co-liveness"?
- What is "observability"?

- `interpret-node :: Concrete.Node -/-> Abstract.Node`
- `interpret-arrow :: Concrete.Arrow -/-> Abstract.Arrow`

### Implementation Reflection