# Overview

The goal of ZRust is to make writing zero-knowledge programs and smart
contracts easy. It is being designed with the following principles in mind:

- **Ease of learning**. Anyone familiar with C-like languages (Javascript, Java, Golang,
 C++, Rust, Solidity) should be able to learn ZRust quickly and with
 minimum effort.
- **Readability**. The code in ZRust should be easy to read intuitively
 comprehensible for anybody familiar with the C++ language familiy. There should
 be no counter-intuitive concepts.
- **Security**. It should be easy to write deterministic and secure programs.
 It should be possible to write safe code without need to understand language
 subtleties. Conversely, it should be difficult to write code which does not do
 what it intuitively appears to be doing.
- **Minimalism and simplicity**. Less code is better. There should ideally be
 only one way to do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building
 complex programs easy.
- **Efficiency**. Code should compile to the most efficient circuit possible.
- **Expose non-optimizable costs**. Costs that cannot be optimized efficiently
 must be made explicit to the developers. An example is the requirement to
 explicitly specify the loop range with constants.

These goals led to the following decisions:

- **Functional programming**. The langauge should have first-class support for
 functional programming principles, such as immutability and minimizing
 side-effects.
- **Rustiness**. The language shall follow rust syntax and philosophy as closely
 as possible. It should be a subset of rust whenever possible. 
- **Divergence from Rust**.
  - **Types**. Obviously we need to adapt the type system to be efficiently
   representable in finite fields, which are the basic building block of R1CS.
  - **References and ownership**. Memory management is very different in R1CS
   circuits compared to the von Neumann architecture. The decision is to pass
   everything "by value" by default without moving ownership (see the developer
   guide for explanation).
  - **`for-while` loops**. Combining `for` and `while` loops allows nicer syntax
   without hiding the fact that the `for-while` loop has a fixed number
   of iterations.
