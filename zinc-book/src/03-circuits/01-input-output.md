# Input and output

In terms of zero-knowledge circuits, the information that we are trying to prove
valid is called **public input**. The secret piece of information that may
be known only by the prover is called **witness**.

In the Zinc framework, the program's **result** becomes **public input**.
That means that whatever the `main` function returns should be known by the verifier.
All other runtime values including **arguments** represent the circuit **witness** data.

So when verifier checks the program's **result** and the **proof**
it is safe to state that:

> There is some set of **arguments** known to **prover**, which,
> being provided into **program** yields the same **output**.

The prover must provide arguments to the program to generate the result and proof.

Verifier will use the proof to check that the result has been obtained by
executing the program.

The following example illustrates a circuit proving knowledge of some
`sha256` hash preimage:

```rust,no_run,noplaypen
use std::crypto::sha256;

fn main(preimage: [bool; 256]) -> [bool; 256] {
    sha256(preimage)
}
```
