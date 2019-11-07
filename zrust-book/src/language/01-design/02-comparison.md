# Comparison to Rust

ZRust is designed specifically for ZKP-based smart contract development, so some
differences from Rust are inevitable.

## Types

Obviously we need to adapt the type system to be efficiently representable in
finite fields, which are the basic building block of R1CS. The current type
system mostly follows Rust, but some aspects are borrowed from smart contract
languages. For example, ZRust provides integer types with 1-byte step sizes,
like those in Solidity.

## Ownership and borrowing

Memory management is very different in R1CS circuits compared to the von Neumann
architecture. Also, since R1CS does not imply parallel programming patterns,
a lot of elements of the Rust design would be unnecessary and redundant.
The decision is to copy data by default without moving ownership. The borrowing
mechanism is still being designed, but probably, only immutable references will
be allowed in the near future.
