# Input and output

The arguments of the `main` function are the secret witness data, and the result
of the function is the public data. The prover must provide secret witness data
in order to generate proofs. The verifier must provide the same public input to
verify the satisfiability of the proof.

The following example illustrates a circuit proving knowledge of some
`sha256` hash preimage:

```rust,no_run,noplaypen
use std::sha256;

fn main(preimage: [u8; 256]) -> [u8; 32] {
    sha256(preimage)
}
```
