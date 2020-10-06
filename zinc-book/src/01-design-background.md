# Design background

The goal of Zinc is to make writing safe zero-knowledge circuits and ZKP-based
smart contracts easy. It has been designed with the following principles in mind:

- **Security**. It should be easy to write deterministic and secure applications.
Conversely, it should be hard to write code to exploit some possible
vulnerabilities found in other programming languages.
- **Safety**. The language must enforce the strictest semantics available,
such as a strong static explicit type system.
- **Efficiency**. The code should compile to the most efficient circuit possible.
- **Cost-exposition**. Performance costs that cannot be optimized efficiently
must be made explicit to the developers. An example is the requirement to
explicitly specify the loop range with constants.
- **Simplicity**. Anyone familiar with C-like languages (Javascript, Java,
Golang, C++, Rust, Solidity, Move) should be able to learn Zinc quickly and
with minimum effort.
- **Readability**. The code in Zinc should be easily readable to anybody
familiar with the C++ language family. There should be no counter-intuitive concepts.
- **Minimalism**. Less code is better. There should ideally be only one way to
do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building
complex applications easy.
- **Turing incompleteness**. Unbounded looping and recursion are not permitted
in Zinc. This not only allows more efficient R1CS circuit construction but
also makes formal verifiability about the call and stack safety easier and
eliminates the gas computation problem inherent to Turing-complete smart
contract platforms, such as EVM.

# Key features

- Type safety
- Type inference
- Immutability
- Movable resources as a first-class citizen
- Module definition and import
- Expressive syntax
- Industrial-grade compiler optimizations
- Turing incompleteness: no recursion or unbounded looping
- Flat learning curve for Rust/JS/Solidity/C++ developers

# Comparison to Rust

Zinc is designed specifically for ZK-circuits and ZKP-based smart contract
development, so some differences from Rust are inevitable.

## Type system

We need to adapt the type system to be efficiently representable in
finite fields, which are the basic building block of R1CS. The current type
system mostly follows Rust, but some aspects are borrowed from smart contract
languages. For example, Zinc provides integer types with 1-byte step sizes,
like those in Solidity.

## Ownership and borrowing

Memory management is very different in R1CS circuits compared to the
von Neumann architecture. Also, since R1CS does not imply parallel programming
patterns, a lot of elements of the Rust design would be unnecessary and redundant.
Zinc has no ownership mechanism found in Rust because all variables will be
passed by value. The borrowing mechanism is still being designed, but probably,
only immutable references will be allowed in the future.

## Loops and recursion

Zinc is a Turing-incomplete language, as it does not allow recursion and
variable loop indexes. Every loop range must be bounded with constant literals
or expressions.
