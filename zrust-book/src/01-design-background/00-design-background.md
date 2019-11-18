# Design background

The goal of ZRust is to make writing safe zero-knowledge programs and ZKP-based
smart contracts easy. It has been designed with the following principles in mind:

- **Security**. It should be easy to write deterministic and secure programs.
Conversely, it should be hard to write code to exploit some possible
vulnerabilities found in other programming languages.
- **Safety**. The language must enforce the most strict semantics available,
such as a strong static explicit type system.
- **Efficiency**. The code should compile to the most efficient circuit possible.
- **Cost-exposition**. Performance costs that cannot be optimized efficiently
must be made explicit to the developers. An example is the requirement to
explicitly specify the loop range with constants.
- **Simplicity**. Anyone familiar with C-like languages (Javascript, Java,
Golang, C++, Rust, Solidity, Move) should be able to learn ZRust quickly and
with minimum effort.
- **Readability**. The code in ZRust should be easily readable to anybody
familiar with the C++ language family. There should be no counter-intuitive concepts.
- **Minimalism**. Less code is better. There should ideally be only one way to
do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building
complex programs easy.
- **Turing incompleteness**. Unbounded looping and recursion are not permitted
in ZRust. This not only allows more efficient R1CS circuit construction but
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
