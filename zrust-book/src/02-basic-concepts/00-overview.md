# Getting started

A ZRust project consists of a binary file called `main.zrs` and zero or more
library files whose contents can be imported into the binary file.

The binary file must contain the `main` function, which accepts input and witness
data, and returns the output data.

Library files may contain only declarations of types, functions, and constants.

## Zargo build tool

The Zargo build tool is the ZRust project builder which will be available soon.
It is being designed after the `cargo` tool from the Rust ecosystem.
