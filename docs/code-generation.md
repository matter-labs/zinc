# Code generation and optimizations

## Language design philosophy

The goal of jab is make writing zero-knowledge programs and smart contracts easy. It was designed with the following principles in mind:

- **Ease of learning**. Anyone familiar with C-like languages (Javascript, Java, C++, Rust, Solidity) should be able to learn jab quickly with minimum effort.
- **Readability**. The code in jab should be easy to read und intuitively comprehensible for anybody familiar with the C++ language familiy. There should be no counter-intuitive concepts.
- **Security**. It should be easy to write deterministic and secure programs. It should be possible to write safe code without the need to understand language subtleties. Conversely, it should be difficult to write code which does not do what it intuitively appears to be doing.
- **Minimalism and simplicity**. Less code is better. There should ideally be only one way to do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building complex programs easy.
- **Efficiency**. The code should compile to the most efficient circuit possible.
- **Expose non-optimizable costs**. Costs that can not be optimized efficiently must be made explicit to the developers. An example is the requirement to explicity specify the loop range with constants.

These goals led to the following decision decisions:

- **Functional programming**. The langauge should have first-class support for functional programming principles, such as immutability and minimizing side-effects.
- **Rustiness**. The language shall follow rust syntax and philosophy as closely as possible. It should be a subset of rust whenever possible. 
  - Notable exceptions:
    - Types. Obviously we need to adapt the type system to be efficiently representable in finite fields, which are the basic building block of R1CS.
    - References and ownership. Memory management is very different in R1CS circuits compared to von Neumann architecture. The decision is to pass everything "by value" by default without moving ownership (see developer guide for explanation).
    - `for...while` loops. Combining `for` and `while` loops allows nicer syntax without hiding the fact that the `for...while` loop has a fixed number of iterations.

## General requirments

Speed of compilation matters a lot. Proper architecture must be put in place from the very beginning to allow fast and scalable compilation.

## Operators

- optimization of linear combinations
- caching reusable intermediary values and linear combinations

## Calling heavy functions

- conditional input parameters
- stack for heavy function calls in nested if branches
