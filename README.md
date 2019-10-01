# Jabberwocky Compiler

The Jabberwocky language compiler tools.

## Language design philosophy

The goal of Jabberwocky is to make writing zero-knowledge programs and smart contracts easy. It is being designed with the following principles in mind:

- **Ease of learning**. Anyone familiar with C-like languages (Javascript, Java, C++, Rust, Solidity) should be able to learn Jabberwocky quickly and with minimum effort.
- **Readability**. The code in Jabberwocky should be easy to read und intuitively comprehensible for anybody familiar with the C++ language familiy. There should be no counter-intuitive concepts.
- **Security**. It should be easy to write deterministic and secure programs. It should be possible to write safe code without need to understand language subtleties. Conversely, it should be difficult to write code which does not do what it intuitively appears to be doing.
- **Minimalism and simplicity**. Less code is better. There should ideally be only one way to do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building complex programs easy.
- **Efficiency**. The code should compile to the most efficient circuit possible.
- **Expose non-optimizable costs**. Costs that cannot be optimized efficiently must be made explicit to the developers. An example is the requirement to explicitly specify the loop range with constants.

These goals led to the following decisions:

- **Functional programming**. The langauge should have first-class support for functional programming principles, such as immutability and minimizing side-effects.
- **Rustiness**. The language shall follow rust syntax and philosophy as closely as possible. It should be a subset of rust whenever possible. 
  - Notable exceptions:
    - Types. Obviously we need to adapt the type system to be efficiently representable in finite fields, which are the basic building block of R1CS.
    - References and ownership. Memory management is very different in R1CS circuits compared to the von Neumann architecture. The decision is to pass everything "by value" by default without moving ownership (see the developer guide for explanation).
    - `for-while` loops. Combining `for` and `while` loops allows nicer syntax without hiding the fact that the `for-while` loop has a fixed number of iterations.

## Compiler command line interface

### The interface description

```
jabc 0.1.0
hedgar2017 <hedgar2017@gmail.com>
The Jabberwocky language compiler

USAGE:
    jabc.exe [FLAGS] --input <INPUT> --output <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -m, --meta       Generates meta info
    -p, --profile    Runs the profiler and prints cost information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>      Specifies the input *.jab file name
    -o, --output <OUTPUT>    Specifies the output *.rs file name
```

### Meta info

```json
{
    "inputs": [
        {
          "identifier": { "name": "a" },
          "type": {"name": "uint", "bitlength": 8}
        }
    ],
    "witnesses": [
        {
          "identifier": { "name": "b" },
          "type": {"name": "int", "bitlength": 248}
        }
    ]
}
```

### Cost profiler output

The cost profiler must print number of constraints for each line in the following `json` format:

```json
{
    "file": "filename.jab",
    "md5":  "md5 hash of the file",
    "constraints": {
        "1": 2,
        "2": 0,
        "3": 1,
        "4": {"inline": 4, "block": 25},
    }
}
```

Each line must sum up constraints in all statements that **begin** in this line.

If a line contains the beginning of a block enclosed in `{ ... }`, the costs must include the total cost of the block in curly brackets:

```rust
1: if a == b { // 3 constraints
2:     t = a * b; // 1 constraints
3: } else {
4:     t = a * b * c; // 2 constraint
5: }
```

=>

```json
"constraints": {
    "1": {"inline": 3, "block": 4},
    "2": 1,
    "3": {"inline": 0, "block": 2},
    "4": 2
}
```

This information will be used to visualize the cost with IDE plugins.

## Interpreter command line interface

### The interface description

```
jabi 0.1.0
hedgar2017 <hedgar2017@gmail.com>
The Jabberwocky language interpreter

USAGE:
    jabi.exe <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>
```
