# Program structure

A ZRust project consists of a binary file called `main.zrs` and zero or more
library files whose contents can be imported into the binary file.

Binary file must contain the `main` function, which accepts two mandatory
arguments for input data and witnesses, and returns the output data.

Library files may contain only declarations of types, functions, and constants.

## Inputs, witnesses and the output

```rust
struct Input {
    {identifier}: {type},
    ...
}

struct Witness {
    {identifier}: {type},
    ...
}

struct Output {
    {identifier}: {type},
    ...
}

fn main(input: Input, witness: Witness) -> Output { Output { ... } }
```

Public inputs (defined as the `Input` type) and secret witness (defined as the
`Witness` type) are the arguments of the program for which the circuit is
implemented. The prover must provide both public inputs and secret witness data
in order to generate proofs. The verifier must provide the same public input
to verify the satisfiability of the proof.

Inputs, witnesses, and outputs must be defined at the beginning of a circuit.
Variable names for input and witness are declared in the global variable scope.

The output data defined as the `Output` type contain the result of a
circuit execution.

If either inputs or witnesses are empty, they must be specified as the `()` type.
If there is no output data, the return type can be omitted like with an
ordinary function.

## Standard library

Most of the standard library functions will be embedded into the ZRust virtual
machine, which is described in the **Chapter 8**.

The standard library will provide computation-heavy algorithms like:
- hashes: `sha256`, `pedersen`, `poseidon`, `blake2s`
- signatures: `eddsa_verify`
- curve primitives: `ecc`
- into_bits / from_bits
