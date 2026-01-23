# Racecar: Visual Kayak AST IDE

## Overview
- GPLv3 licensed
- Can export code in AST image format or text
- Live integration with Kayak VM
    - UI shows what parts of the program are currently executing
    - text widget shows CLI output
- Data structure design interface for initial data objects that programs execute on
- Debugging interface
    - allows programmer to step forwards and backwards through programs

## Racecar Build System
- https://ngnghm.github.io/blog/2016/04/26/chapter-9-build-systems-and-modularity/
- build system not a separate ad-hoc process to the language, but "a library for meta-level build activities, written in an appropriate deterministic reactive style, in the same general purpose programming language as the rest of the system"
- build from "source control", not source code
- modules (packages are just supermodules)
- will need some kind of DSL embedded in Kayak to specify builds
    - what kind of standard library support should we need for creating/interpreting DSLs?
- "virtualization is branching"
    - if you want to test a change to the system, "branch" it by creating a new virtualization and running the changed system inside
    - if it fails, just drop it
    - if it succeeds and you want to merge, execute in reverse until the branch point, apply the changes and then continue forwards

### Kayak VCS
- WE NEED our own VCS system for a sane build system
- Build from "source control" instead of source code
    - see https://ngnghm.github.io/blog/2016/04/26/chapter-9-build-systems-and-modularity/
- "Semantics as Source"
    - Is this just AST as source or something different
- AST data structures for each function indexed in database
    - can we use postgresql for this?
- different versions are managed as "branches", updates are pushed as "commits"
- whole programs are managed in separate "repositories"
- live Kayak programs can query for new function versions and migrate automatically
- special public instance of VCS can be used as a package repository
    - read-only, cannot push new versions without permission
- administration system
    - merge requests, pruning branches, permissions, etc.
    - need some way for developers to communicate in comment threads

## Paid Services ("Racecar Cloud Suite")
- Application hosting
    - IDE can automatically deploy programs to a server running Kayak VM
    - branching + versioning of programs
    - need to develop an HTTP library for this
    - possible tiers
        - personal tier
        - enterprise tier
        - critical tier
            - hosted on physical servers with verified boot and read-only root
            - bsd jails would be ideal for this
- Possible hardware
    - libreboot vs coreboot
    - nix + checksum verification
        - immutable os means less attack surface
        - verifying os checksum on boot to ensure nothing was modified
    - nixos vs nix-freebsd vs nix-dragonflybsd
        - nixos is stable, used in production and has lots of documentation
        - nix-freebsd still in development and not used in production AFAIK
        - nix-dragonflybsd would have to be forked from nix-freebsd and developed by us
            - how much work would be required for this?
            - is this even worth developing?
- Possible stacks
    - DARP stack?
        - DragonflyBSD: Optimized for multithreading and SSD storage, lighter footprint than Linux
            - Rust support possibly incomplete?
        - Apache: classic HTTP server, but could possibly be replaced with Kayak HTTP library?
        - Rust: code for managing VCS/hosted application instances
        - PostgreSQL: stores client information and AST data structures for VCS
    - Bespoke PostgreSQL hosting service for VCS
        - neon.tech provides branching
    - Ideal future Kayak stack
        - Minim: sel4 + Kayak VM
        - HTTP library for serving resources
        - Database program written in Kayak?
            - How should this be done?

## Future Features
- custom theme engine
- plugin engine
- window manager for Kayak programs that have a GUI
    - similar to Plan 9 rio wm