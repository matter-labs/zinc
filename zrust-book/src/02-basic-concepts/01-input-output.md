# Inputs, witnesses and the output

Public inputs (defined as the `Input` type) and secret witness (defined as the
`Witness` type) are the arguments of the program for which the circuit is
implemented. The prover must provide both public inputs and secret witness data
in order to generate proofs. The verifier must provide the same public input
to verify the satisfiability of the proof.

The output data defined as the `Output` type contain the result of a
circuit execution.

Actually, `input`, `witness` and the output can be of any type, but using
structures is considered a good design as it makes their use in the code more
explicit.

```rust,no_run,noplaypen
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
