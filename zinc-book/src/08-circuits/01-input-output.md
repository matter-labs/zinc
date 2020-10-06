# Input and output

In terms of zero-knowledge circuits, the information that we are trying to prove
valid is called **public input**. The secret piece of information that may
be known only by the prover is called **witness**.

In the Zinc framework, the circuit **output** becomes **public input**.
This means that whatever the `main` function returns should be known by the verifier.
All other runtime values including **arguments** represent the circuit **witness** data.

So when verifier checks the circuit **output** and the **proof**
it is safe to state that:

> There is some set of **arguments** known to **prover**, which,
> being provided to circuit yields the same **output**.

The prover must provide arguments to the application to generate the result and proof.

Verifier will use the proof to check that the result has been obtained by
executing the circuit.

The following example illustrates a circuit proving knowledge of some
`sha256` hash preimage:

```rust,no_run,noplaypen
use std::crypto::sha256;

fn main(preimage: [bool; 512]) -> [bool; 256] {
    sha256(preimage)
}
```
