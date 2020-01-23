# Input and output

Public inputs and secret witness are the arguments of the program for which the
circuit is implemented. The prover must provide both public inputs and secret
witness data in order to generate proofs. The verifier must provide the same
public input to verify the satisfiability of the proof.

The output data contains the result of circuit execution.

The following example illustrates a circuit proving knowledge of some
`sha256` hash preimage:

```rust,no_run,noplaypen
use std::sha256;

fn main(preimage: [u8; 256]) -> [u8; 32] {
    sha256(preimage)
}
```
