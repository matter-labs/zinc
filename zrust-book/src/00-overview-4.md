# Program structure

A ZRust project consists of a binary file called `main.zrs` and zero or more library files whose contents can be imported into the binary file.

Binary file must contain the three mandatory structures `input`, `witness`, `output`, and the `main` function.

Library files may contain only declarations of types, functions, and constants.

## Public inputs and secret witness

```rust
    input {
        {identifier}: {type},
        ...
    }

    witness {
        {identifier}: {type},
        ...
    }

    output {
        {identifier}: {type},
        ...
    }
```

Public inputs \(defined in the `inputs` block\) and secret witness \(defined in the `witness` block\) are the arguments of the program for which the circuit is implemented. The prover must provide both public inputs and secret witness data in order to generate proofs. The verifier must provide the same public input to verify the satisfiability of the proof.

Inputs, witnesses, and outputs must be defined at the beginning of a circuit. Variable names for input and witness are declared in the global variable scope.

Outputs contain the results of a circuit.

Each circuit can have 0 or more input, witness, and output arguments, but all the sections must be declared, even if they are empty.

```rust
    input {
        x: u128,
        ...
    }

    witness {
        cubic_root: u128,
        ...
    }

    output {
        result: field,
        ...
    }
```

## Standard library \(TODO\)

* hashes: `sha256`, `pedersen`, `poseidon`, `blake2s`
* signatures: `eddsa_verify`
* curve primitives: `ecc`
* into\_bits / from\_bits

