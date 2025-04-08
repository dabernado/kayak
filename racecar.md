# Racecar: Visual Kayak AST IDE

## Overview
- GPLv3 licensed
- Can export code in AST image format or text
- Live integration with Kayak VM
    - UI shows what parts of the program are currently executing
    - text widget shows CLI output

## Paid Features ("Cloud Suite")
- VCS hosting for code in AST format
    - AST data structures for each function indexed in database
        - can we use postgresql for this?
    - new versions are pushed as "commits"
    - whole programs are managed in separate "repositories"
    - live Kayak programs can query for new function versions and migrate automatically
- Application hosting
    - IDE can automatically deploy programs to a server running Kayak
    - can also deploy code directly from VCS service

## Future Features
- custom theme engine
- plugin engine
- window manager for Kayak programs that have a GUI
    - similar to Plan 9 rio wm