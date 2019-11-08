# zrust-vm

## Contents

- [Overview](#Overview)
- Appendix
    - [Bytecode Specification](doc/bytecode.md)
    - [R1CS Notes](doc/r1cs.md)

## Overview

**zrust-vm** is a virtual machine that serves three purposes:
execute arbitrary computations,
generate zero-knowledge proof of performed computations,
or verify provided proof without knowing all the inputs for the computations.

**zrust-vm** is a stack-based virtual machine which is similar to many existing ones (e.g. python's vm).
Even though **zrust-vm** is designed considering specifics and limitations of zero-knowledge computations,
bytecode instructions only manipulate data on the stack while
all zero-knowledge constraints are automatically applied by virtual machine. 

There are however some limitations specific to zero-knowledge computations.
For example conditional execution is implemented differently.
Instead of conditional jumps, there is conditional assign instruction (you can think of it as of ternary operator).


## Usage

Show bytecode as human-readable assembly code:

    zrust-disassembly <file.zrsb>

Execute bytecode, log instructions and stack state:

    RUST_LOG=info zrust-vm <file.zrsb>

## Roadmap

- Instructions:
    - [x] Arithmetic
    - [ ] Boolean & Comparison
    - [ ] Conditional assignment
    - [ ] Loops
    - [ ] Function calls
