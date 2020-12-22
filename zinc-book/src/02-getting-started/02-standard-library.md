# Standard library

The standard library is currently located in a built-in module called `std`.
The library contains the following modules:
- `crypto` - cryptographic and hash functions
    - `ecc` - elliptic curve cryptography
    - `schnorr` - EDDSA signature verification
- `convert` - bit array conversion functions
- `array` - array processing functions
- `ff` - finite field functions
- `collections` - data collection types

All the standard library contents are listed in the [Appendix E](../appendix/E-standard-library.md).

Standard library items can be used directly or imported with `use`:

```rust,no_run,noplaypen
use std::crypto::sha256; // an import

fn main(preimage: [bool; 256]) -> ([bool; 256], (field, field)) {
    let input_sha256 = sha256(preimage); // imported
    let input_pedersen = std::crypto::pedersen(preimage); // directly

    (input_sha256, input_pedersen)
}
```

# The zkSync library

The zkSync library is an emerging library, which for now only contains the global
transaction `msg` variable:

```rust,no_run,noplaypen
let amount = zksync::msg.amount;
```

The zkSync library contents are listed in the [Appendix F](../appendix/F-zksync-library.md).
