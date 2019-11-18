# Comparison to Rust

ZRust is designed specifically for ZK-circuits and ZKP-based smart contract
development, so some differences from Rust are inevitable.

## Type system

We need to adapt the type system to be efficiently representable in
finite fields, which are the basic building block of R1CS. The current type
system mostly follows Rust, but some aspects are borrowed from smart contract
languages. For example, ZRust provides integer types with 1-byte step sizes,
like those in Solidity.

## Ownership and borrowing

Memory management is very different in R1CS
circuits compared to the von Neumann architecture. Also, since R1CS does not
imply parallel programming patterns, a lot of elements of the Rust design would
be unnecessary and redundant. ZRust has no ownership mechanism found in Rust
because all variables will be passed by value. The borrowing mechanism is still
being designed, but probably, only immutable references will be allowed shortly.

## Loops and recursion

ZRust is a Turing-incomplete language, as it does not allow recursion and
variable loop indexes. Each loop range must be bounded with constant literals
or expressions.
