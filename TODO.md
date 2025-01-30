## High Priority
- refactor sum types
    - no more packed tags, just cellptrs like everything else
        - replace int tag with bool
        - refactor sum type funcs to handle that
        - remove lc/rc from sum instructions
        - refactor (or remove?) sum instruction masking functions