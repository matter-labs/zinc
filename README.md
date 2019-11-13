# zrust-vm

## Contents

- [Overview](#Overview)
- [Usage](#Usage)
- [Memory](#Memory)
- [Conditional Execution](#Conditional-Execution)
- Appendix
    - [Instructions Reference](doc/instructions.md)
    - [R1CS Notes](doc/r1cs.md)

## Overview

**zrust-vm** is a virtual machine that serves three purposes:
execute arbitrary computations,
generate zero-knowledge proof of performed computations,
or verify provided proof without knowing all the inputs for the computations.

*zrust-vm is a stack-based virtual machine which is similar to many existing ones (e.g. python's vm).
Even though *zrust-vm* is designed considering specifics and limitations of zero-knowledge computations,
bytecode instructions only manipulate data on the stack while
all zero-knowledge constraints are automatically applied by virtual machine. 

There are however some limitations specific to zero-knowledge computations.
For example conditional execution is implemented differently.
Instead of conditional jumps, there is conditional assign instruction (you can think of it as of ternary operator).


## Usage

Currently vm doesn't have any kind of output, but you can see execution logs:

    RUST_LOG=info zrust-vm <file.zrsb>
    
There is a disassembler to show contents of bytecode files, e.g.:

    $ zrust-disassembly test.zrsb
    push 25
    push 49
    add
    push 60
    sub

## Memory

Since *zrust-vm* is a stack-based virtual machine, all data is stored on the stack.
The is no data mutability in *zrust-vm* therefore values on stack never change.
However values can be removed from the stack and new values placed instead.

## Conditional Execution

There is no conditional execution in *zrust-vm*, which means for a given program
the virtual machine will always execute same instructions regardless of data input.
That also means that for all loops the number of iterations must be constant. 
