# Design

The goal of ZRust is to make writing zero-knowledge programs and smart
contracts easy. It is being designed with the following principles in mind:

- **Security**. It should be easy to write deterministic and secure programs.
 Conversely, it should be hard to write code in order to exploit some possible
 vulnerabilities found in other programming languages.
- **Safety**. The language must enforce the most strict semantics available,
 such as strong static explicit type system.
- **Efficiency**. Code should compile to the most efficient circuit possible.
- **Cost-exposition**. Costs that cannot be optimized efficiently
 must be made explicit to the developers. An example is the requirement to
 explicitly specify the loop range with constants.
- **Simplicity**. Anyone familiar with C-like languages (Javascript, Java,
 Golang, C++, Rust, Solidity) should be able to learn ZRust quickly and with
 minimum effort.
- **Readability**. The code in ZRust should be easy to read for anybody
 familiar with the C++ language family. There should be no counter-intuitive
 concepts.
- **Minimalism**. Less code is better. There should ideally be
 only one way to do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building
 complex programs easy.
