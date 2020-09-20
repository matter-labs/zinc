# Standard library

The standard library is currently located in a built-in module called `std`.
The library contains the following modules:
- `crypto` - cryptographic and hash functions
    - `ecc` - elliptic curve cryptography
    - `schnorr` - EDDSA signature verification
- `convert` - bit array conversion functions
- `array` - array processing functions
- `ff` - finite field functions

All the standard library contents are listed in [Appendix E](../appendix/E-standard-library.md).

Standard library items can be used directly or imported with `use`:

```rust,no_run,noplaypen
use std::crypto::sha256;

fn main(preimage: [bool; 256]) -> ([bool; 256], (field, field)) {
    let input_sha256 = sha256(preimage); // imported
    dbg!(input_sha256);

    let input_pedersen = std::crypto::pedersen(preimage); // directly
    dbg!(input_pedersen);

    (input_sha256, input_pedersen)
}
```
