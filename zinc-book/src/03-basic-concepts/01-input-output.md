# Input and output

In terms of zero-knowledge circuits the information that we are trying to prove valid is called **public input**.
And the secret piece of information that may be known only by prover is called **witness**.

In Zinc framework the program's **result** becomes **public witness**.
That means that whatever the `main` function returns should be known by verifier.
All other runtime values including **arguments** represent circuit's **witness**.

So when verifier checks the program's **result** and the **proof**
it is safe to state that:

> There is some set of **arguments** known to **prover**, which
> when provided into **program** will yield the same **output**.


The prover must provide program's arguments in order to generate result and proof.

Verifier will use the proof to check that result is obtained by executing the program.

The following example illustrates a circuit proving knowledge of some
`sha256` hash preimage:

```rust,no_run,noplaypen
use std::sha256;

fn main(preimage: [bool; 256]) -> [bool; 256] {
    sha256(preimage)
}
```
